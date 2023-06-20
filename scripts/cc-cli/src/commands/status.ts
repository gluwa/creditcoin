import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { checkAddress } from "../utils/account";
import { getStatus, printValidatorStatus } from "../utils/status";

export function makeStatusCommand() {
  const cmd = new Command("status");
  cmd.description("Get staking status for an address");
  cmd.option("-a, --address [address]", "Address to get status for");
  cmd.action(statusAction);
  return cmd;
}

async function statusAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  // Check options
  checkAddress(options.address, api);

  const validatorStatus = await getStatus(options.address, api);

  await printValidatorStatus(validatorStatus, api);

  process.exit(0);
}
