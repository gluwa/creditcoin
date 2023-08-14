import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { bond, checkRewardDestination } from "../utils/bond";
import { promptContinue, setInteractivity } from "../utils/interactive";
import {
  AccountBalance,
  getBalance,
  toCTCString,
  checkAmount,
} from "../utils/balance";
import { BN } from "creditcoin-js";
import {
  inputOrDefault,
  parseAddressOrExit,
  parseAmountOrExit,
  parseBoolean,
  parseChoiceOrExit,
  requiredInput,
} from "../utils/parsing";
import { initKeyringFromEnvOrPrompt, initStashKeyring } from "../utils/account";

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

  const { amount, controller, rewardDestination, extra, interactive } =
    parseOptions(options);

  const stashKeyring = await initStashKeyring(api);
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

  await promptContinue(interactive);

  const bondTxResult = await bond(
    stashKeyring,
    controller,
    amount,
    rewardDestination,
    api,
    extra
  );

  console.log(bondTxResult.info);
  process.exit(0);
}

async function checkBalance(amount: BN, api: any, address: string) {
  const balance = await getBalance(address, api);
  checkBalanceAgainstBondAmount(balance, amount);
}

function checkBalanceAgainstBondAmount(balance: AccountBalance, amount: BN) {
  if (balance.transferable.lt(amount)) {
    console.error(
      `Insufficient funds to bond ${toCTCString(amount)}, only ${toCTCString(
        balance.transferable
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

  const controller = parseAddressOrExit(
    requiredInput(
      options.controller,
      "Failed to bond: Must specify a controller address"
    )
  );

  const rewardDestination = checkRewardDestination(
    parseChoiceOrExit(inputOrDefault(options.rewardDestination, "Staked"), [
      "Staked",
      "Stash",
      "Controller",
    ])
  );

  const extra = parseBoolean(options.extra);

  const interactive = setInteractivity(options);

  return { amount, controller, rewardDestination, extra, interactive };
}
