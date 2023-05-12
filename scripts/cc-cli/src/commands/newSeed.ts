import { Command, OptionValues } from "commander";
import { mnemonicGenerate } from "@polkadot/util-crypto";
import { writeFileSync } from "fs";


export function makeNewSeedCommand() {
    const cmd = new Command('new');
    cmd.description('Create new mnemonic seed');
    cmd.option('-l, --lenght [word-lenght]', 'Specify the amount of words')
    cmd.option('-s, --save [file-name]', 'Save the new stash seed to a file')
    cmd.action(async (options: OptionValues) => {
        console.log("Creating new mnemonic...")
        const mnemonic = options.lenght ? mnemonicGenerate(options.lenght) : mnemonicGenerate();
        if (options.save) {
            console.log("Saving seed to file:", options.save);
            // Write mnemonic to a file
            writeFileSync(options.save, mnemonic);
        } else {
            console.log("Mnemonic:", mnemonic);
        }
        process.exit(0);
    });
    return cmd;
}