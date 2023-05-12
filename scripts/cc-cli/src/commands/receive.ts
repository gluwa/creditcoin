import { Command, OptionValues } from "commander";
import { ApiPromise } from "creditcoin-js";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";

export function makeReceiveCommand() {
    const cmd = new Command('receive');
    cmd.description('Show account address');
    cmd.option('-s, --seed [mneomonic]', 'Specify mnemonic phrase to use for new account')
    cmd.option('-f, --file [file-name]', 'Specify file with mnemonic phrase to use for new account')
    // cmd.option('-p, --password [password]', 'Specify password to use for new account')
    cmd.action(receiveAction);
    return cmd;
}

async function receiveAction(options: OptionValues) {
    const api = await ApiPromise.create();

    const seed = getSeedFromOptions(options);
    const pair = initKeyringPair(seed);
    const address = pair.address;

    console.log("Account address:", address);

    process.exit(0);
}