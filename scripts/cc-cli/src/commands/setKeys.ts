import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";

export function makeSetKeysCommand() {
    const cmd = new Command('set-keys');
    cmd.description('Set session keys for a Controller account');
    cmd.option('-s, --seed [seed]', 'Specify mnemonic phrase to set keys from')
    cmd.option('-f, --file [file]', 'Specify file with mnemonic phrase to set keys from')
    cmd.option('-k, --keys [keys]', 'Specify keys to set')
    cmd.option('-u, --url [url]', 'URL for the Substrate node')
    cmd.action(setKeysAction);
    return cmd;
}

async function setKeysAction(options: OptionValues) {
    const api = await newApi(options.url);

    // Build account
    const seed = getSeedFromOptions(options);
    const stash = initKeyringPair(seed);

    // Send transaction
    const tx = api.tx.session.setKeys(options.keys, []);
    const hash = await tx.signAndSend(stash);

    console.log("Set keys transaction hash: " + hash.toHex());

    process.exit(0);
}
