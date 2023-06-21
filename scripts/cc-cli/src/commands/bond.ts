import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  checkAddress,
  getStashSeedFromEnvOrPrompt,
  initKeyringPair
} from "../utils/account";
import { Balance, getBalance, toMicrounits } from "../utils/balance";
import { bond, parseRewardDestination } from "../utils/bond";
import { promptContinue } from "../utils/promptContinue";

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

  // If no controller error and exit
  checkAddress(options.controller, api);

  // If no amount error and exit
  if (!options.amount || !parseInt(options.amount, 10)) {
    console.log("Must specify amount to bond");
    process.exit(1);
  }

  const stashSeed = await getStashSeedFromEnvOrPrompt();

  // Check balance
  const stashKeyring = initKeyringPair(stashSeed);
  const stashAddress = stashKeyring.address;
  const balance = await getBalance(stashAddress, api);
  const amount = parseInt(options.amount, 10);
  checkBalanceAgainstBondAmount(balance, amount);

  const rewardDestination = options.rewardDestination
    ? parseRewardDestination(options.rewardDestination)
    : "Staked";

  console.log("Creating bond transaction...");
  console.log("Controller address:", options.controller);
  console.log("Reward destination:", rewardDestination);
  console.log("Amount:", parseInt(options.amount, 10));

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

function checkBalanceAgainstBondAmount(balance: Balance, amount: number) {
  if (balance.free.sub(balance.miscFrozen).lt(toMicrounits(amount))) {
    throw new Error(
      `Insufficient funds to bond ${toMicrounits(amount).toString()}`
    );
  }
}
