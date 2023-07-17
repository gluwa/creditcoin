import { Command, OptionValues } from "commander";
import { BN } from "creditcoin-js";
import { newApi } from "../api";
import {
  getControllerSeedFromEnvOrPrompt,
  getStashSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import {
  AccountBalance,
  getBalance,
  parseCTCString,
  printBalance,
  toCTCString,
  MICROUNITS_PER_CTC,
} from "../utils/balance";
import { bond, checkRewardDestination } from "../utils/bond";
import { percentFromPerbill } from "../utils/perbill";
import { promptContinue, promptContinueOrSkip } from "../utils/interactive";
import { StakingPalletValidatorPrefs } from "../utils/validate";
import {
  TxStatus,
  requireEnoughFundsToSend,
  signSendAndWatch,
} from "../utils/tx";
import {
  inputOrDefault,
  parseAmountOrExit,
  parseBoolean,
  parseChoiceOrExit,
  parsePercentAsPerbillOrExit,
  requiredInput,
} from "../utils/parsing";

export function makeWizardCommand() {
  const cmd = new Command("wizard");
  cmd.description(
    "Run the validator setup wizard. Only requires funded stash and controller accounts."
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

    const { amount, rewardDestination, commission, blocked } =
      parseOptions(options);

    // Create new API instance
    const { api } = await newApi(options.url);

    // Generate stash keyring
    const stashSeed = await getStashSeedFromEnvOrPrompt();
    const stashKeyring = initKeyringPair(stashSeed);
    const stashAddress = stashKeyring.address;

    // Generate controller keyring
    const controllerSeed = await getControllerSeedFromEnvOrPrompt();
    const controllerKeyring = initKeyringPair(controllerSeed);
    const controllerAddress = controllerKeyring.address;

    // Validate prefs
    const preferences: StakingPalletValidatorPrefs = { commission, blocked };

    // Node settings
    const nodeUrl: string = options.url ? options.url : "ws://localhost:9944";

    // State parameters being used
    console.log("Using the following parameters:");
    console.log(`💰 Stash account: ${stashAddress}`);
    console.log(`🎮 Controller account: ${controllerAddress}`);
    console.log(`🪙 Amount to bond: ${toCTCString(amount)}`);
    console.log(`🎁 Reward destination: ${rewardDestination}`);
    console.log(`📡 Node URL: ${nodeUrl}`);
    console.log(`💸 Commission: ${percentFromPerbill(commission).toString()}`);
    console.log(`🔐 Blocked: ${blocked ? "Yes" : "No"}`);

    // Prompt continue
    await promptContinue();

    // get balances.
    const stashBalance = await getBalance(stashAddress, api);
    const controllerBalance =
      stashAddress === controllerAddress
        ? stashBalance
        : await getBalance(controllerAddress, api);

    // ensure they have enough fee's and balance to cover the wizard.
    const grosslyEstimatedFee = parseCTCString("2");
    if (controllerAddress === stashAddress) {
      const amountWithFee = amount.add(grosslyEstimatedFee);
      checkStashBalance(stashAddress, stashBalance, amountWithFee);
    } else {
      checkStashBalance(stashAddress, stashBalance, amount);
      checkControllerBalance(
        controllerAddress,
        controllerBalance,
        grosslyEstimatedFee
      );
    }

    const bondExtra: boolean = checkIfAlreadyBonded(stashBalance);

    if (bondExtra) {
      console.log(
        "⚠️  Warning: Stash account already bonded. This will increase the amount bonded."
      );
      if (await promptContinueOrSkip(`Continue or skip bonding extra funds?`)) {
        checkStashBalance(stashAddress, stashBalance, amount);
        // Bond extra
        console.log("Sending bond transaction...");
        const bondTxResult = await bond(
          stashSeed,
          controllerAddress,
          amount,
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
        amount,
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
    const setKeysTx = api.tx.session.setKeys(newKeys, "");

    // Validate
    console.log("Creating validate transaction...");
    const validateTx = api.tx.staking.validate(preferences);

    // Send transactions
    console.log("Sending setKeys and validate transactions...");
    const txs = [setKeysTx, validateTx];

    const batchTx = api.tx.utility.batchAll(txs);
    await requireEnoughFundsToSend(batchTx, controllerAddress, api);

    const batchResult = await signSendAndWatch(batchTx, api, controllerKeyring);

    console.log(batchResult.info);

    // // Inform process
    console.log("🧙 Validator wizard completed successfully!");
    console.log("Your validator should appear on the waiting queue.");

    process.exit(0);
  });
  return cmd;
}

function checkControllerBalance(
  address: string,
  balance: AccountBalance,
  amount: BN
) {
  if (balance.transferable.lt(amount)) {
    console.log(
      "Controller account does not have enough funds to pay transaction fees"
    );
    printBalance(balance);
    console.log(
      `Please send at least ${toCTCString(
        amount
      )} to controller address ${address} and try again.`
    );
    process.exit(1);
  }
}

function checkStashBalance(
  address: string,
  balance: AccountBalance,
  amount: BN
) {
  if (balance.transferable.lt(amount)) {
    console.log(
      `Stash account does not have enough funds to bond ${toCTCString(amount)}`
    );
    printBalance(balance);
    console.log(`Please send funds to stash address ${address} and try again.`);
    process.exit(1);
  }
}

function checkIfAlreadyBonded(balance: AccountBalance) {
  if (balance.bonded.gt(new BN(0))) {
    return true;
  } else {
    return false;
  }
}

function parseOptions(options: OptionValues) {
  const amount = parseAmountOrExit(
    requiredInput(
      options.amount,
      "Failed to setup wizard: Bond amount required"
    )
  );
  if (amount.lt(new BN(1).mul(MICROUNITS_PER_CTC))) {
    console.log("Failed to setup wizard: Bond amount must be at least 1 CTC");
    process.exit(1);
  }

  const rewardDestination = checkRewardDestination(
    parseChoiceOrExit(inputOrDefault(options.rewardDestination, "Staked"), [
      "Staked",
      "Stash",
      "Controller",
    ])
  );

  const commission = parsePercentAsPerbillOrExit(
    inputOrDefault(options.commission, "0")
  );

  const blocked = parseBoolean(options.blocked);

  return { amount, rewardDestination, commission, blocked };
}
