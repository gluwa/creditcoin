import { Command, OptionValues } from "commander";
import { newApi } from "../api";

export function makeRotateKeysCommand(){
    const cmd = new Command('rotate-keys');
    cmd.option('-u, --url [url]', 'URL for the Substrate node')
    cmd.description('Rotate session keys for a specified node');
    cmd.action(rotateKeysAction);
    return cmd;
}

async function rotateKeysAction(options: OptionValues) {
    const api = await newApi(options.url);
    const newKeys = await api.rpc.author.rotateKeys();
    console.log(newKeys.toString());
    process.exit(0);
}