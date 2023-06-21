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
  const seedPhrase = options.length
    ? mnemonicGenerate(options.length)
    : mnemonicGenerate();
  console.log("Seed phrase:", seedPhrase);
  process.exit(0);
}
