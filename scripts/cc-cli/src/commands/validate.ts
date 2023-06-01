import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions } from "../utils/account";
import { StakingPalletValidatorPrefs, validate } from "../utils/validate";
import { perbillFromPercent } from "../utils/perbill";

export function makeValidateCommand() {
  const cmd = new Command("validate");
  cmd.description("Signal intention to validate from a Controller account");
  cmd.option(
    "-s, --seed [mnemonic]",
    "Specify mnemonic phrase to use for new account"
  );
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with mnemonic phrase to use for new account"
  );
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
  const api = await newApi(options.url);

  const stashSeed = getSeedFromOptions(options);

  const commission = options.commission
    ? perbillFromPercent(options.commission)
    : perbillFromPercent(0);
  const blocked = options.blocked ? options.blocked : false;
  const preferences: StakingPalletValidatorPrefs = { commission, blocked };

  console.log("Creating validate transaction...");

  const validateTxHash = await validate(stashSeed, preferences, api);

  console.log("Validate transaction sent with hash:", validateTxHash.toHex());
  process.exit(0);
}
