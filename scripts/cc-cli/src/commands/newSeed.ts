import { mnemonicGenerate } from "@polkadot/util-crypto";
import { Command, OptionValues } from "commander";

export function makeNewSeedCommand() {
  const cmd = new Command("new");
  cmd.description("Create new seed phrase");
  cmd.option("-l, --length [word-length]", "Specify the amount of words");
  cmd.action(newSeedAction);
  return cmd;
}

function newSeedAction(options: OptionValues) {
  console.log("Creating new seed phrase...");
  const length = options.length ? parseLength(options.length) : 12;
  const seedPhrase = mnemonicGenerate(length);
  console.log("Seed phrase:", seedPhrase);
  process.exit(0);
}

function parseLength(length: string): 12 | 15 | 18 | 21 | 24 {
  const parsed = parseInt(length, 10);
  if (
    parsed !== 12 &&
    parsed !== 15 &&
    parsed !== 18 &&
    parsed !== 21 &&
    parsed !== 24
  ) {
    console.error(
      "Failed to create new seed phrase: Invalid length, must be one of 12, 15, 18, 21 or 24",
    );
    process.exit(1);
  }
  return parsed;
}
