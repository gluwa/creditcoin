import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getStatus, printValidatorStatus } from "../utils/status";
import { parseAddressOrExit, requiredInput } from "../utils/parsing";

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
  const address = parseAddressOrExit(
    requiredInput(
      options.address,
      "Failed to show validator status: Must specify an address"
    )
  );

  const validatorStatus = await getStatus(address, api);

  await printValidatorStatus(validatorStatus, api);

  process.exit(0);
}
