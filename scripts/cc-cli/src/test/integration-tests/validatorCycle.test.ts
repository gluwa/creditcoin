import { BN } from "creditcoin-js";
import { newApi } from "../../api";
import { initKeyringPair } from "../../utils/account";
import { signSendAndWatch } from "../../utils/tx";
import {
  parseAddressInternal,
  parseAmountInternal,
  parseHexStringInternal,
} from "../../utils/parsing";
import { getBalance, printBalance } from "../../utils/balance";
import { mnemonicValidate } from "@polkadot/util-crypto";
import execa from "execa";
import { getValidatorStatus } from "../../utils/validatorStatus";
import {
  ALICE_NODE_URL,
  BOB_NODE_URL,
  fundFromSudo,
  waitEras,
} from "./helpers";

describe("integration test: validator manual setup", () => {
  test("full validator cycle using manual setup", async () => {
    // Bob's node is used for checking its configuration as a validator
    // and for sending extrinsics using the CLI
    const bobApi = (await newApi(BOB_NODE_URL)).api;

    // While CLI commands always send extrinsics through Bob's node,
    // sudo calls and state checks both use Alice's node
    const aliceApi = (await newApi(ALICE_NODE_URL)).api;

    // Creating two accounts using `new` should return two valid mnemonic seeds
    const stashSeed = execa
      .commandSync("creditcoin-cli new")
      .stdout.split("Seed phrase: ")[1];
    console.log("Stash seed: ", stashSeed);

    const controllerSeed = execa
      .commandSync("creditcoin-cli new")
      .stdout.split("Seed phrase: ")[1];
    console.log("Controller seed: ", controllerSeed);

    expect(mnemonicValidate(stashSeed)).toBe(true);
    expect(mnemonicValidate(controllerSeed)).toBe(true);

    // Getting the addresses using `show-address` should return two valid addresses
    const stashAddress = parseAddressInternal(
      execa
        .commandSync(`creditcoin-cli show-address`, {
          env: {
            CC_SECRET: stashSeed,
          },
        })
        .stdout.split("Account address: ")[1],
    );

    const controllerAddress = parseAddressInternal(
      execa
        .commandSync(`creditcoin-cli show-address`, {
          env: {
            CC_SECRET: controllerSeed,
          },
        })
        .stdout.split("Account address: ")[1],
    );

    // Funding the stash account should make its balance equal to the amount funded
    const fundAmount = parseAmountInternal("10000");

    const fundTx = await fundFromSudo(stashAddress, fundAmount);
    await signSendAndWatch(fundTx, aliceApi, initKeyringPair("//Alice"));
    const stashBalance = (await getBalance(stashAddress, aliceApi))
      .transferable;
    expect(stashBalance.toString()).toBe(fundAmount.toString());

    // Sending 1k ctc from stash to controller should make the controller balance equal to 1k ctc
    const sendAmount = "1000";
    execa.commandSync(
      // CLI commands are sent through Bob's node
      `creditcoin-cli send --amount ${sendAmount} --to ${controllerAddress} --url ${BOB_NODE_URL}`,
      {
        env: {
          CC_SECRET: stashSeed,
        },
      },
    );
    const controllerBalance = (await getBalance(controllerAddress, aliceApi))
      .transferable;
    expect(controllerBalance.toString()).toBe(
      parseAmountInternal(sendAmount).toString(),
    );

    // Bonding 1k ctc from stash and setting the controller should
    // - make the stash bonded balance equal to 1k ctc
    // - make the stash's controller be the controller address
    // - make controller's stash be the stash address
    const bondAmount = "1000";
    execa.commandSync(
      `creditcoin-cli bond --controller ${controllerAddress} --amount ${bondAmount} --url ${BOB_NODE_URL}`,
      {
        env: {
          CC_STASH_SECRET: stashSeed,
        },
      },
    );
    // wait 5 seconds for nodes to sync
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const stashStatus = await getValidatorStatus(stashAddress, aliceApi);
    expect(stashStatus.bonded).toBe(true);

    const controllerStatus = await getValidatorStatus(
      controllerAddress,
      aliceApi,
    );
    expect(controllerStatus.stash).toBe(stashAddress);

    const stashBondedBalance = (await getBalance(stashAddress, aliceApi))
      .bonded;
    expect(stashBondedBalance.toString()).toBe(
      parseAmountInternal(bondAmount).toString(),
    );

    // Rotating session keys for the node should return a valid hex string
    const newKeys = parseHexStringInternal(
      execa
        .commandSync(`creditcoin-cli rotate-keys --url ${BOB_NODE_URL}`)
        .stdout.split("New keys: ")[1],
    );

    // Setting session keys for the controller should
    // - make the validator (stash) next session keys equal to the new keys
    // - make the new keys appear as the node's session keys
    execa.commandSync(
      `creditcoin-cli set-keys --keys ${newKeys} --url ${BOB_NODE_URL}`,
      {
        env: {
          CC_CONTROLLER_SECRET: controllerSeed,
        },
      },
    );
    // wait 5 seconds for nodes to sync
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const validatorSessionKeys = await aliceApi.query.session.nextKeys(
      stashAddress,
    );
    expect(validatorSessionKeys.toHex()).toBe(newKeys);
    const nodeHasKeys = (await bobApi.rpc.author.hasSessionKeys(newKeys))
      .isTrue;
    expect(nodeHasKeys).toBe(true);

    // Signaling intention to validate should make the validator (stash) appear as waiting
    execa.commandSync(
      `creditcoin-cli validate --commission 1 --url ${BOB_NODE_URL}`,
      {
        env: {
          CC_CONTROLLER_SECRET: controllerSeed,
        },
      },
    );

    const stashStatusAfterValidating = await getValidatorStatus(
      stashAddress,
      bobApi,
    );
    expect(stashStatusAfterValidating.waiting).toBe(true);

    // After increasing the validator count, (forcing an era- currently not) and waiting for the next era,
    // the validator should become elected & active.
    const increaseValidatorCountTx = aliceApi.tx.staking.setValidatorCount(2);
    const increaseValidatorCountSudoTx = aliceApi.tx.sudo.sudo(
      increaseValidatorCountTx,
    );
    await signSendAndWatch(
      increaseValidatorCountSudoTx,
      aliceApi,
      initKeyringPair("//Alice"),
    );
    const validatorCount = (
      await aliceApi.query.staking.validatorCount()
    ).toNumber();
    expect(validatorCount).toBe(2);
    await waitEras(2, aliceApi);
    const stashStatusAfterEra = await getValidatorStatus(stashAddress, bobApi);
    expect(stashStatusAfterEra.active).toBe(true);

    // After waiting for another era, the validator should have accumulated era rewards to distribute
    const startingEra = (
      await aliceApi.derive.session.info()
    ).activeEra.toNumber();
    console.log("Starting era: ", startingEra);
    await waitEras(1, aliceApi);

    // After distributing rewards, the validator staked balance should increase
    // (because it was set to staked)
    const balanceBeforeRewards = await getBalance(stashAddress, aliceApi);
    console.log(balanceBeforeRewards.bonded.toString());

    execa.commandSync(
      `creditcoin-cli distribute-rewards --url ${BOB_NODE_URL} --validator-id ${stashAddress} --era ${startingEra}`,
      {
        env: {
          CC_SECRET: stashSeed,
        },
      },
    );

    // wait 5 seconds for nodes to sync
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const balanceAfterRewards = await getBalance(stashAddress, aliceApi);
    console.log(balanceAfterRewards.bonded.toString());
    const balanceIncreased = balanceAfterRewards.bonded.gt(
      balanceBeforeRewards.bonded,
    );
    expect(balanceIncreased).toBe(true);

    // After executing the chill commmand, the validator should no longer be active nor waiting
    execa.commandSync(`creditcoin-cli chill --url ${BOB_NODE_URL}`, {
      env: {
        CC_CONTROLLER_SECRET: controllerSeed,
      },
    });
    // wait 5 seconds for nodes to sync
    await waitEras(2, aliceApi);
    const stashStatusAfterChill = await getValidatorStatus(
      stashAddress,
      bobApi,
    );
    expect(stashStatusAfterChill.active).toBe(false);
    expect(stashStatusAfterChill.waiting).toBe(false);

    // After unbonding, the validator should no longer be bonded
    execa.commandSync(
      // Unbonding defaults to max if it exceeds the bonded amount
      `creditcoin-cli unbond --url ${BOB_NODE_URL} -a 100000`,
      {
        env: {
          CC_CONTROLLER_SECRET: controllerSeed,
        },
      },
    );
    // wait 5 seconds for nodes to sync
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const balanceAfterUnbonding = await getBalance(stashAddress, aliceApi);
    const isUnbonding = balanceAfterUnbonding.unbonding.gt(new BN(0));
    printBalance(balanceAfterRewards);
    printBalance(balanceAfterUnbonding);
    const isUnbondingAll = balanceAfterUnbonding.unbonding.eq(
      balanceAfterRewards.bonded,
    );
    expect(isUnbonding).toBe(true);
    expect(isUnbondingAll).toBe(true);

    // After unbonding and waiting for the unbonding period, the validator should be able to withdraw
    // the unbonded amount and end up with more funds than the initial funding
    const unbondingPeriod = aliceApi.consts.staking.bondingDuration.toNumber();
    console.log("Unbonding period: ", unbondingPeriod);
    await waitEras(unbondingPeriod + 1, aliceApi, true);

    execa.commandSync(
      `creditcoin-cli withdraw-unbonded --url ${BOB_NODE_URL}`,
      {
        env: {
          CC_CONTROLLER_SECRET: controllerSeed,
        },
      },
    );

    // wait 5 seconds for nodes to sync
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const balanceAfterWithdraw = await getBalance(stashAddress, aliceApi);
    printBalance(balanceAfterWithdraw);
    const stashAmount = fundAmount.sub(parseAmountInternal(sendAmount));
    expect(balanceAfterWithdraw.transferable.gte(stashAmount)).toBe(true);
  }, 2000000);
});
