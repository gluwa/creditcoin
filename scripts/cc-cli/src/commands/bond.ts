import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  checkAddress,
  getStashSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import { bond, parseRewardDestination } from "../utils/bond";
import { promptContinue } from "../utils/promptContinue";
import {
  Balance,
  getBalance,
  parseCTCString,
  toCTCString,
  checkAmount,
} from "../utils/balance";
import { BN } from "creditcoin-js";

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

  const stashSeed = await getStashSeedFromEnvOrPrompt();
  const stashKeyring = initKeyringPair(stashSeed);
  const stashAddress = stashKeyring.address;

  const amount = parseCTCString(options.amount);

  // Check inputs
  checkAddress(options.controller, api);
  checkAmount(amount);
  await checkBalance(amount, api, stashAddress);

  const rewardDestination = options.rewardDestination
    ? parseRewardDestination(options.rewardDestination)
    : "Staked";

  console.log("Creating bond transaction...");
  console.log("Controller address:", options.controller);
  console.log("Reward destination:", rewardDestination);
  console.log("Amount:", toCTCString(amount));

  await promptContinue();

  console.log("Extra: ", options.extra);

  const bondTxHash = await bond(
    stashSeed,
    options.controller,
    amount,
    rewardDestination,
    api,
    options.extra
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
