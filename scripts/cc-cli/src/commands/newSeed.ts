import { Command, OptionValues } from "commander";
import { mnemonicGenerate } from "@polkadot/util-crypto";
import { writeFileSync } from "fs";

export function makeNewSeedCommand() {
  const cmd = new Command("new");
  cmd.description("Create new seed phrase");
  cmd.option("-l, --length [word-length]", "Specify the amount of words");
  cmd.option("-s, --save [file-name]", "Save the new seed to a file");
  cmd.action(newSeedAction);
  return cmd;
}

function newSeedAction(options: OptionValues) {
  console.log("Creating new seed phrase...");
  const seedPhrase = options.length
    ? mnemonicGenerate(options.length)
    : mnemonicGenerate();
  if (options.save) {
    console.log("Saving seed to file:", options.save);
    // Write seed phrase to a file
    writeFileSync(options.save, seedPhrase);
  } else {
    console.log("Seed phrase:", seedPhrase);
  }
  process.exit(0);
}
