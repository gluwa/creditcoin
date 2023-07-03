import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getBalance, printBalance } from "../utils/balance";
import { parseAddresOrExit, requiredInput } from "../utils/parsing";

export function makeBalanceCommand() {
  const cmd = new Command("balance");
  cmd.description("Get balance of an account");
  cmd.option("-a, --address [address]", "Specify address to get balance of");
  cmd.action(balanceAction);
  return cmd;
}

async function balanceAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const address = parseAddresOrExit(
    requiredInput(
      options.address,
      "Failed to show balance: Must specify an address"
    )
  );

  const balance = await getBalance(address, api);
  printBalance(balance);

  process.exit(0);
}
