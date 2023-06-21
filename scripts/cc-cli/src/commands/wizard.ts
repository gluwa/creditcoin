import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { readFileSync } from "fs";
import { initKeyringPair } from "../utils/account";
import { promptContinue, promptContinueOrSkip } from "../utils/promptContinue";
import { bond, parseRewardDestination } from "../utils/bond";
import { StakingPalletValidatorPrefs } from "../utils/validate";
import { perbillFromPercent, percentFromPerbill } from "../utils/perbill";
import {
  Balance,
  getBalance,
  printBalance,
  toMicrounits,
} from "../utils/balance";
import { BN } from "creditcoin-js";
import { TxStatus, signSendAndWatch } from "../utils/tx";

export function makeWizardCommand() {
  const cmd = new Command("wizard");
  cmd.description(
    "Run the validator setup wizard. Only requires funded stash and controller accounts."
  );
  cmd.option(
    "-ss, --stash-seed [stash-seed]",
    "Specify mnemonic phrase to use for stash account"
  );
  cmd.option(
    "-sf, --stash-file [stash-file]",
    "Specify file with mnemonic phrase to use for stash account"
  );
  cmd.option(
    "-cs, --controller-seed [controller-seed]",
    "Specify mnemonic phrase to use for controller account"
  );
  cmd.option(
    "-cf, --controller-file [controller-file]",
    "Specify file with mnemonic phrase to use for controller account"
  );
  cmd.option(
    "-r, --reward-destination [reward-destination]",
    "Specify reward destination account to use for new account"
  );
  cmd.option("-a, --amount [amount]", "Amount to bond");
  cmd.option("--commission [commission]", "Specify commission for validator");
  cmd.option(
    "--blocked",
    "Specify if validator is blocked for new nominations"
  );
  cmd.action(async (options: OptionValues) => {
    console.log("🧙 Running staking wizard...");

    // Create new API instance
    const { api } = await newApi(options.url);

    // Generate stash keyring
    const stashSeed = getStashSeedFromOptions(options);
    const stashKeyring = initKeyringPair(stashSeed);
    const stashAddress = stashKeyring.address;

    // Generate controller keyring
    const controllerSeed = getControllerSeedFromOptions(options);
    const controllerKeyring = initKeyringPair(controllerSeed);
    const controllerAddress = controllerKeyring.address;

    // Bond prefs
    const amount: string = options.amount
      ? parseInt(options.amount, 10).toString()
      : "0";
    const rewardDestination = options.rewardDestination
      ? parseRewardDestination(options.rewardDestination)
      : "Staked";

    // Validate prefs
    const commission = options.commission
      ? perbillFromPercent(options.commission)
      : 0;
    const blocked: boolean = options.blocked ? options.blocked : false;
    const preferences: StakingPalletValidatorPrefs = { commission, blocked };

    // Node settings
    const nodeUrl: string = options.url ? options.url : "ws://localhost:9944";

    // State parameters being used
    console.log("Using the following parameters:");
    console.log(`💰 Stash account: ${stashAddress}`);
    console.log(`🕹️  Controller account: ${controllerAddress}`);
    console.log(`🪙  Amount to bond: ${amount} CTC`);
    console.log(`🎁 Reward destination: ${rewardDestination}`);
    console.log(`📡 Node URL: ${nodeUrl}`);
    console.log(`💸 Commission: ${percentFromPerbill(commission).toString()}`);
    console.log(`🔐 Blocked: ${blocked ? "Yes" : "No"}`);

    // Prompt continue
    await promptContinue();

    // Check both accounts have funds
    const stashBalance = await getBalance(stashAddress, api);
    const controllerBalance = await getBalance(controllerAddress, api);
    checkStashBalance(stashAddress, stashBalance, options.amount);
    checkControllerBalance(controllerAddress, controllerBalance, 1);
    const bondExtra: boolean = checkIfAlreadyBonded(stashAddress, stashBalance);

    if (bondExtra) {
      console.log(
        "⚠️  Warning: Stash account already bonded. This will increase the amount bonded."
      );
      if (await promptContinueOrSkip(`Continue or skip bonding extra funds?`)) {
        checkStashBalance(stashAddress, stashBalance, options.amount);
        // Bond extra
        console.log("Sending bond transaction...");
        const bondTxResult = await bond(
          stashSeed,
          controllerAddress,
          parseInt(options.amount, 10),
          rewardDestination,
          api,
          bondExtra
        );
        console.log(bondTxResult.info);
        if (bondTxResult.status === TxStatus.failed) {
          console.log("Bond transaction failed. Exiting.");
          process.exit(1);
        }
      }
    } else {
      // Bond
      console.log("Sending bond transaction...");
      const bondTxResult = await bond(
        stashSeed,
        controllerAddress,
        parseInt(options.amount, 10),
        rewardDestination,
        api
      );
      console.log(bondTxResult.info);
      if (bondTxResult.status === TxStatus.failed) {
        console.log("Bond transaction failed. Exiting.");
        process.exit(1);
      }
    }

    // Rotate keys
    console.log("Generating new session keys on node...");
    const newKeys = (await api.rpc.author.rotateKeys()).toString();
    console.log("New node session keys:", newKeys);

    // Set keys
    console.log("Creating setKeys transaction...");
    const setKeysTx = api.tx.session.setKeys(newKeys, preferences);

    // Validate
    console.log("Creating validate transaction...");
    const validateTx = api.tx.staking.validate({ preferences });

    // Send transactions
    console.log("Sending setKeys and validate transactions...");
    const txs = [setKeysTx, validateTx];

    const batchTx = api.tx.utility.batchAll(txs);

    const batchResult = await signSendAndWatch(batchTx, api, controllerKeyring);

    console.log(batchResult.info);

    // // Inform process
    console.log("🧙 Validator wizard completed successfully!");
    console.log("Your validator should appear on the waiting queue.");

    process.exit(0);
  });
  return cmd;
}

function getStashSeedFromOptions(options: OptionValues) {
  if (options.stashSeed) {
    return options.stashSeed;
  } else if (options.stashFile) {
    return readFileSync(options.stashFile).toString();
  } else {
    console.log("Must specify either seed or file for the Stash account");
    process.exit(1);
  }
}

function getControllerSeedFromOptions(options: OptionValues) {
  if (options.controllerSeed) {
    return options.controllerSeed;
  } else if (options.controllerFile) {
    return readFileSync(options.controllerFile).toString();
  } else {
    console.log("Must specify either seed or file for the Controller account");
    process.exit(1);
  }
}

function checkControllerBalance(
  address: string,
  balance: Balance,
  amount: number
) {
  if (balance.free < new BN(amount)) {
    console.log(
      "Controller account does not have enough funds to pay transaction fees"
    );
    printBalance(balance);
    console.log(
      `Please send at least ${amount.toString()} CTC to controller address ${address} and try again.`
    );
    process.exit(1);
  }
}

function checkStashBalance(address: string, balance: Balance, amount: number) {
  if (balance.free.sub(balance.miscFrozen).lt(toMicrounits(amount))) {
    console.log(
      `Stash account does not have enough funds to bond ${amount.toString()} CTC`
    );
    printBalance(balance);
    console.log(`Please send funds to stash address ${address} and try again.`);
    process.exit(1);
  }
}

function checkIfAlreadyBonded(address: string, balance: Balance) {
  if (balance.miscFrozen.gt(new BN(0))) {
    return true;
  } else {
    return false;
  }
}
