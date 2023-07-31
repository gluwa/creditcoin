import { ApiPromise, BN } from "creditcoin-js";
import { newApi } from "../api";
import { initKeyringPair } from "../utils/account";
import { signSendAndWatch } from "../utils/tx";
import { execSync } from "child_process";
import { parseAddress, parseAmount, parseHexString } from "../utils/parsing";
import { getStatus, printValidatorStatus } from "../utils/status";
import { getBalance, printBalance } from "../utils/balance";
import { mnemonicGenerate, mnemonicValidate } from "@polkadot/util-crypto";
import execa from "execa";

function randomAccount() {
  const seed = mnemonicGenerate();
  const keyring = initKeyringPair(seed);
  const address = keyring.address;
  return { seed, keyring, address };
}

async function fundAccounts(amount: BN) {
  const { api } = await newApi("ws://localhost:9944");
  const stash = randomAccount();
  const controller = randomAccount();
  const tx = await fundAddressesFromSudo(
    [stash.address, controller.address],
    amount
  );
  await signSendAndWatch(tx, api, initKeyringPair("//Alice"));

  return { stash, controller };
}

// beforeAll(() => {

describe.skip("integration test: validator wizard setup", () => {
  test("new validator should appear as waiting after using the wizard setup", async () => {
    // Fund stash and controller
    const { stash, controller } = await fundAccounts(parseAmount("10000"));
    // Run wizard setup with 1k ctc ang to pair with node Bob
    const out = execa.commandSync(
      `creditcoin-cli wizard --amount 1000 --url ws://localhost:9945`,
      {
        env: {
          CC_STASH_SEED: stash.seed,
          CC_CONTROLLER_SEED: controller.seed,
        },
      }
    );

    const { api } = await newApi("ws://localhost:9944");
    const validatorStatus = await getStatus(stash.address, api);

    expect(validatorStatus.waiting).toBe(true);
  }, 100000);
});

describe("integration test: validator manual setup", () => {
  test.skip("util test", async () => {
    const { api } = await newApi();

    const increaseValidatorCountTx = api.tx.staking.setValidatorCount(2);
    const increaseValidatorCountSudoTx = api.tx.sudo.sudo(
      increaseValidatorCountTx
    );
    await signSendAndWatch(
      increaseValidatorCountSudoTx,
      api,
      initKeyringPair("//Alice")
    );

    const validatorCount = (
      await api.query.staking.validatorCount()
    ).toNumber();
    console.log(validatorCount);
  }, 100000);

  test("full validator cycle using manual setup", async () => {
    // Bob's node is used for checking its configuration as a validator
    // and for sending extrinsics using the CLI
    const bobApi = (await newApi("ws://localhost:9945")).api;

    // While CLI commands always send extrinsics through Bob's node,
    // sudo calls and state checks both use Alice's node
    const aliceApi = (await newApi("ws://localhost:9944")).api;

    // Creating two accounts using `new` should return two valid mnemonic seeds
    const stashSeed = execa
      .commandSync("creditcoin-cli new")
      .stdout.split("Seed phrase: ")[1];

    const controllerSeed = execa
      .commandSync("creditcoin-cli new")
      .stdout.split("Seed phrase: ")[1];

    expect(mnemonicValidate(stashSeed)).toBe(true);
    expect(mnemonicValidate(controllerSeed)).toBe(true);

    // Getting the addresses using `show-address` should return two valid addresses
    const stashAddress = parseAddress(
      execa
        .commandSync(`creditcoin-cli show-address`, {
          env: {
            CC_SEED: stashSeed,
          },
        })
        .stdout.split("Account address: ")[1]
    );

    const controllerAddress = parseAddress(
      execa
        .commandSync(`creditcoin-cli show-address`, {
          env: {
            CC_SEED: controllerSeed,
          },
        })
        .stdout.split("Account address: ")[1]
    );

    // Funding the stash account should make its balance equal to the amount funded
    const fundAmount = parseAmount("10000");

    const fundTx = await fundFromSudo(stashAddress, fundAmount);
    await signSendAndWatch(fundTx, aliceApi, initKeyringPair("//Alice"));
    const stashBalance = (await getBalance(stashAddress, aliceApi))
      .transferable;
    expect(stashBalance.toString()).toBe(fundAmount.toString());

    // Sending 1k ctc from stash to controller should make the controller balance equal to 1k ctc
    const sendAmount = "1000";
    const sendResult = execa.commandSync(
      // CLI commands are sent through Bob's node
      `creditcoin-cli send --amount ${sendAmount} --to ${controllerAddress} --url ws://localhost:9945`,
      {
        env: {
          CC_SEED: stashSeed,
        },
      }
    );
    const controllerBalance = (await getBalance(controllerAddress, aliceApi))
      .transferable;
    expect(controllerBalance.toString()).toBe(
      parseAmount(sendAmount).toString()
    );

    // Bonding 1k ctc from stash and setting the controller should
    // - make the stash bonded balance equal to 1k ctc
    // - make the stash's controller be the controller address
    // - make controller's stash be the stash address
    const bondAmount = "1000";
    const bondResult = execa.commandSync(
      `creditcoin-cli bond --controller ${controllerAddress} --amount ${bondAmount} --url ws://localhost:9945`,
      {
        env: {
          CC_STASH_SEED: stashSeed,
        },
      }
    );
    // wait 5 seconds for nodes to sync
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const stashStatus = await getStatus(stashAddress, aliceApi);
    expect(stashStatus.bonded).toBe(true);

    const controllerStatus = await getStatus(controllerAddress, aliceApi);
    expect(controllerStatus.stash).toBe(stashAddress);

    const stashBondedBalance = (await getBalance(stashAddress, aliceApi))
      .bonded;
    expect(stashBondedBalance.toString()).toBe(
      parseAmount(bondAmount).toString()
    );

    // Rotating session keys for the node should return a valid hex string
    const newKeys = parseHexString(
      execa
        .commandSync(`creditcoin-cli rotate-keys --url ws://localhost:9945`)
        .stdout.split("New keys: ")[1]
    );

    // Setting session keys for the controller should
    // - make the validator (stash) next session keys equal to the new keys
    // - make the new keys appear as the node's session keys
    const setKeysResult = execa.commandSync(
      `creditcoin-cli set-keys --keys ${newKeys} --url ws://localhost:9945`,
      {
        env: {
          CC_CONTROLLER_SEED: controllerSeed,
        },
      }
    );
    // wait 5 seconds for nodes to sync
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const validatorSessionKeys = await aliceApi.query.session.nextKeys(
      stashAddress
    );
    expect(validatorSessionKeys.toHex()).toBe(newKeys);
    const nodeHasKeys = (await bobApi.rpc.author.hasSessionKeys(newKeys))
      .isTrue;
    expect(nodeHasKeys).toBe(true);

    // Signaling intention to validate should make the validator (stash) appear as waiting
    const signalResult = execa.commandSync(
      `creditcoin-cli validate --commission 1 --url ws://localhost:9945`,
      {
        env: {
          CC_CONTROLLER_SEED: controllerSeed,
        },
      }
    );

    const stashStatusAfterValidating = await getStatus(stashAddress, bobApi);
    expect(stashStatusAfterValidating.waiting).toBe(true);

    // After increasing the validator count, (forcing an era- currently not) and waiting for the next era,
    // the validator should become elected & active.
    const increaseValidatorCountTx = aliceApi.tx.staking.setValidatorCount(2);
    const increaseValidatorCountSudoTx = aliceApi.tx.sudo.sudo(
      increaseValidatorCountTx
    );
    await signSendAndWatch(
      increaseValidatorCountSudoTx,
      aliceApi,
      initKeyringPair("//Alice")
    );
    const validatorCount = await (
      await aliceApi.query.staking.validatorCount()
    ).toNumber();
    expect(validatorCount).toBe(2);
    await waitEras(2, aliceApi);
    const stashStatusAfterEra = await getStatus(stashAddress, bobApi);
    expect(stashStatusAfterEra.active).toBe(true);

    // After waiting for another era, the validator should have accumulated era rewards to distribute
    const startingEra = (
      await aliceApi.derive.session.info()
    ).activeEra.toNumber();
    console.log("Starting era: ", startingEra);
    await waitEras(1, aliceApi);

    // const eraRewards = await aliceApi.query.staking.erasRewardPoints(
    //   startingEra
    // );
    // expect(eraRewards.individual.entries()).toBeGreaterThan(0);

    // After distributing rewards, the validator staked balance should increase
    // (because it was set to staked)
    const balanceBeforeRewards = await getBalance(stashAddress, aliceApi);
    console.log(balanceBeforeRewards.bonded.toString());

    const distributeCommand = execa.commandSync(
      `creditcoin-cli distribute-rewards --url ws://localhost:9945 --validator-id ${stashAddress} --era ${startingEra}`,
      {
        env: {
          CC_SEED: stashSeed,
        },
      }
    );

    // wait 5 seconds for nodes to sync
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const balanceAfterRewards = await getBalance(stashAddress, aliceApi);
    console.log(balanceAfterRewards.bonded.toString());
    const balanceIncreased = balanceAfterRewards.bonded.gt(
      balanceBeforeRewards.bonded
    );
    expect(balanceIncreased).toBe(true);

    // After executing the chill commmand, the validator should no longer be active nor waiting
    const chillCommand = execa.commandSync(
      `creditcoin-cli chill --url ws://localhost:9945`,
      {
        env: {
          CC_CONTROLLER_SEED: controllerSeed,
        },
      }
    );
    // wait 5 seconds for nodes to sync
    await waitEras(2, aliceApi);
    const stashStatusAfterChill = await getStatus(stashAddress, bobApi);
    expect(stashStatusAfterChill.active).toBe(false);
    expect(stashStatusAfterChill.waiting).toBe(false);

    // After unbonding, the validator should no longer be bonded
    const unbondCommand = execa.commandSync(
      // Unbonding defaults to max if it exceeds the bonded amount
      `creditcoin-cli unbond --url ws://localhost:9945 -a 100000`,
      {
        env: {
          CC_CONTROLLER_SEED: controllerSeed,
        },
      }
    );
    // wait 5 seconds for nodes to sync
    await new Promise((resolve) => setTimeout(resolve, 5000));
    // const stashStatusAfterUnbonding = await getStatus(stashAddress, bobApi);
    const balanceAfterUnbonding = await getBalance(stashAddress, aliceApi);
    const isUnbonding = balanceAfterUnbonding.unbonding.gt(new BN(0));
    printBalance(balanceAfterRewards);
    printBalance(balanceAfterUnbonding);
    const isUnbondingAll = balanceAfterUnbonding.unbonding.eq(
      balanceAfterRewards.bonded
    );
    expect(isUnbonding).toBe(true);
    expect(isUnbondingAll).toBe(true);

    // After unbonding and waiting for the unbonding period, the validator should be able to withdraw
    // the unbonded amount and end up with more funds than the initial funding
    const unbondingPeriod = aliceApi.consts.staking.bondingDuration.toNumber();
    console.log("Unbonding period: ", unbondingPeriod);
    await waitEras(unbondingPeriod + 1, aliceApi, true);

    const withdrawCommand = execa.commandSync(
      `creditcoin-cli withdraw-unbonded --url ws://localhost:9945`,
      {
        env: {
          CC_CONTROLLER_SEED: controllerSeed,
        },
      }
    );

    // wait 5 seconds for nodes to sync
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const balanceAfterWithdraw = await getBalance(stashAddress, aliceApi);
    printBalance(balanceAfterWithdraw);
    const stashAmount = fundAmount.sub(parseAmount(sendAmount));
    expect(balanceAfterWithdraw.transferable.gte(stashAmount)).toBe(true);
  }, 2000000);
});

describe.skip("integration test: queries", () => {
  test("alice balance should be 1m ctc", async () => {
    const out = execSync(
      `creditcoin-cli balance --address 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY --json`
    );
    const json = JSON.parse(out.toString());
    expect(json.balance.transferable).toBe("1000000000000000000000000");
  });
});

async function fundFromSudo(
  address: string,
  amount: BN,
  url = "ws://localhost:9944"
) {
  const { api } = await newApi(url);
  const call = api.tx.balances.setBalance(address, amount.toString(), "0");
  const tx = api.tx.sudo.sudo(call);
  return tx;
}

async function fundAddressesFromSudo(
  addresses: string[],
  amount: BN,
  url = "ws://localhost:9944"
) {
  const { api } = await newApi(url);
  const txs = addresses.map((address) => {
    const fundTx = api.tx.balances.setBalance(address, amount.toString(), "0");
    return api.tx.sudo.sudo(fundTx);
  });
  const tx = api.tx.utility.batchAll(txs);
  return tx;
}

async function waitEras(eras: number, api: ApiPromise, force = true) {
  if (force) {
    forceNewEra(api);
  }
  let eraInfo = await api.derive.session.info();
  let currentEra = eraInfo.currentEra.toNumber();
  let targetEra = currentEra + eras;
  let blockTime = api.consts.babe.expectedBlockTime.toNumber();
  while (currentEra < targetEra) {
    console.log(`Waiting for era ${targetEra}, currently at ${currentEra}`);
    await new Promise((resolve) => setTimeout(resolve, blockTime));
    eraInfo = await api.derive.session.info();
    currentEra = eraInfo.currentEra.toNumber();
  }
}

async function waitSessions(sessions: number, api: ApiPromise) {
  let sessionInfo = await api.derive.session.info();
  let currentSession = sessionInfo.currentIndex.toNumber();
  let targetSession = currentSession + sessions;
  let blockTime = api.consts.babe.expectedBlockTime.toNumber();
  while (currentSession < targetSession) {
    console.log(
      `Waiting for session ${targetSession}, currently at ${currentSession}`
    );
    await new Promise((resolve) => setTimeout(resolve, blockTime));
    sessionInfo = await api.derive.session.info();
    currentSession = sessionInfo.currentIndex.toNumber();
  }
}

async function forceNewEra(api: ApiPromise) {
  const tx = api.tx.staking.forceNewEra();
  const sudoTx = api.tx.sudo.sudo(tx);
  signSendAndWatch(sudoTx, api, initKeyringPair("//Alice"));
}
