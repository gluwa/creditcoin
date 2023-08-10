import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getControllerSeedFromEnvOrPrompt } from "../utils/account";
import { StakingPalletValidatorPrefs, validate } from "../utils/validate";
import {
  inputOrDefault,
  parseBoolean,
  parsePercentAsPerbillOrExit,
} from "../utils/parsing";
import { setInteractivity } from "../utils/interactive";

export function makeValidateCommand() {
  const cmd = new Command("validate");
  cmd.description("Signal intention to validate from a Controller account");
  cmd.option(
    "--commission [commission]",
    "Specify commission for validator in percent",
  );
  cmd.option(
    "--blocked",
    "Specify if validator is blocked for new nominations",
  );
  cmd.action(validateAction);
  return cmd;
}

async function validateAction(options: OptionValues) {
  const interactive = setInteractivity(options);
  const { api } = await newApi(options.url);

  const controllerSeed = await getControllerSeedFromEnvOrPrompt(interactive);

  // Default commission is 0%
  const commission = parsePercentAsPerbillOrExit(
    inputOrDefault(options.commission, "0"),
  );

  const blocked = parseBoolean(options.blocked);

  const preferences: StakingPalletValidatorPrefs = { commission, blocked };

  console.log("Creating validate transaction...");

  const result = await validate(controllerSeed, preferences, api);

  console.log(result.info);
  process.exit(0);
}
