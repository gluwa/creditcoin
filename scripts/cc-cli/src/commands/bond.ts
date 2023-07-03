import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  checkAddress,
  getStashSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import {
  bond,
  checkRewardDestination,
  parseRewardDestination,
} from "../utils/bond";
import { promptContinue } from "../utils/promptContinue";
import {
  Balance,
  getBalance,
  parseCTCString,
  toCTCString,
  checkAmount,
} from "../utils/balance";
import { BN } from "creditcoin-js";
import {
  inputOrDefault,
  parseAddresOrExit,
  parseAmountOrExit,
  parseBoolean,
  parseChoice,
  parseChoiceOrExit,
  requiredInput,
} from "../utils/parsing";

export function makeBondCommand() {
  const cmd = new Command("bond");
  cmd.description("Bond CTC from a Stash account");
  cmd.option("-a, --amount [amount]", "Amount to bond");
  cmd.option("-c, --controller [controller]", "Specify controller address");
  cmd.option(
    "-r, --reward-destination [reward-destination]",
    "Specify reward destination account to use for new account"
  );
  cmd.option(
    "-x, --extra",
    "Bond as extra, adding more funds to an existing bond"
  );
  cmd.action(bondAction);
  return cmd;
}

async function bondAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const { amount, controller, rewardDestination, extra } =
    parseOptions(options);

  const stashSeed = await getStashSeedFromEnvOrPrompt();
  const stashKeyring = initKeyringPair(stashSeed);
  const stashAddress = stashKeyring.address;

  // Check if stash has enough balance
  await checkBalance(amount, api, stashAddress);

  console.log("Creating bond transaction...");
  console.log("Controller address:", controller);
  console.log("Reward destination:", rewardDestination);
  console.log("Amount:", toCTCString(amount));
  if (extra) {
    console.log("Bonding as 'extra'; funds will be added to existing bond");
  }

  await promptContinue();

  const bondTxHash = await bond(
    stashSeed,
    controller,
    amount,
    rewardDestination,
    api,
    extra
  );

  console.log("Bond transaction sent with hash:", bondTxHash);
  process.exit(0);
}

async function checkBalance(amount: BN, api: any, address: string) {
  const balance = await getBalance(address, api);
  checkBalanceAgainstBondAmount(balance, amount);
}

function checkBalanceAgainstBondAmount(balance: Balance, amount: BN) {
  const available = balance.free.sub(balance.miscFrozen);
  if (available.lt(amount)) {
    console.error(
      `Insufficient funds to bond ${toCTCString(amount)}, only ${toCTCString(
        available
      )} available`
    );
    process.exit(1);
  }
}

function parseOptions(options: OptionValues) {
  const amount = parseAmountOrExit(
    requiredInput(
      options.amount,
      "Failed to bond: Must specify an amount to bond"
    )
  );
  checkAmount(amount);

  const controller = parseAddresOrExit(
    requiredInput(
      options.controller,
      "Failed to bond: Must specify a controller address"
    )
  );

  const rewardDestination = checkRewardDestination(
    parseChoice(inputOrDefault(options.rewardDestination, "Staked"), [
      "Staked",
      "Stash",
      "Controller",
    ])
  );

  const extra = parseBoolean(options.extra);

  return { amount, controller, rewardDestination, extra };
}
