import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getControllerSeedFromEnvOrPrompt } from "../utils/account";
import { perbillFromPercent } from "../utils/perbill";
import { StakingPalletValidatorPrefs, validate } from "../utils/validate";
import {
  inputOrDefault,
  parseBoolean,
  parsePercentAsPerbill,
} from "../utils/parsing";

export function makeValidateCommand() {
  const cmd = new Command("validate");
  cmd.description("Signal intention to validate from a Controller account");
  cmd.option(
    "--commission [commission]",
    "Specify commission for validator in percent"
  );
  cmd.option(
    "--blocked",
    "Specify if validator is blocked for new nominations"
  );
  cmd.action(validateAction);
  return cmd;
}

async function validateAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const controllerSeed = await getControllerSeedFromEnvOrPrompt();

  // Default commission is 0%
  const commission = parsePercentAsPerbill(
    inputOrDefault(options.commission, "0")
  );

  const blocked = parseBoolean(options.blocked);

  const preferences: StakingPalletValidatorPrefs = { commission, blocked };

  console.log("Creating validate transaction...");

  const result = await validate(controllerSeed, preferences, api);

  console.log(result.info);
  process.exit(0);
}
