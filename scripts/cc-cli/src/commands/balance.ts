import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getBalance, printBalance } from "../utils/balance";
import { checkAddress } from "../utils/account";

export function makeBalanceCommand() {
  const cmd = new Command("balance");
  cmd.description("Get balance of an account");
  cmd.option("-a, --address [address]", "Specify address to get balance of");
  cmd.action(balanceAction);
  return cmd;
}

async function balanceAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  // Check options
  checkAddress(options.address, api);

  const balance = await getBalance(options.address, api);
  printBalance(balance);

  process.exit(0);
}
