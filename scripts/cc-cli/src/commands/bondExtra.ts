import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { bond, parseRewardDestination } from "../utils/bond";
import { promptContinue } from "../utils/promptContinue";
import { Balance, getBalance, toMicrounits } from "../utils/balance";
import { bondExtra } from "../utils/bondExtra";
import { BN } from "creditcoin-js";

export function makeBondExtraCommand() {
  const cmd = new Command("bond-extra");
  cmd.description("Add CTC to an existing bond from a Stash account");
  cmd.option("-a, --amount [amount]", "Amount to bond");
  cmd.option("-s, --seed [seed phrase]", "Specify seed phrase to bond from");
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with seed phrase to bond from"
  );
  cmd.action(bondExtraAction);
  return cmd;
}

async function bondExtraAction(options: OptionValues) {
  // If no amount error and exit
  if (!options.amount || !parseInt(options.amount, 10)) {
    console.log("Must specify amount to bond");
    process.exit(1);
  }

  const { api } = await newApi(options.url);

  const stashSeed = getSeedFromOptions(options);

  // Check balance
  const stashKeyring = initKeyringPair(stashSeed);
  const stashAddress = stashKeyring.address;
  const balance = await getBalance(stashAddress, api);
  const amount = parseInt(options.amount, 10);
  checkBalanceAgainstBondAmount(balance, amount);

  const alreadyBonded = checkIfAlreadyBonded(stashAddress, balance);
  if (!alreadyBonded) {
    console.log("Must already be bonded to use bond-extra, use bond instead");
    process.exit(1);
  }

  console.log("Creating bond extra transaction...");
  console.log("Amount:", parseInt(options.amount, 10));

  await promptContinue();

  const bondTxHash = await bondExtra(stashSeed, options.amount, api);

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

function checkIfAlreadyBonded(address: string, balance: Balance) {
  if (balance.miscFrozen.gt(new BN(0))) {
    return true;
  } else {
    return false;
  }
}
