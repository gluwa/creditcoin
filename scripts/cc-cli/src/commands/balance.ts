import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { balanceIsZero, getBalance, printBalance } from "../utils/balance";

export function makeBalanceCommand () {
    const cmd = new Command('balance');
    cmd.description('Get balance of an account');
    cmd.option('-s, --seed [mneomonic]', 'Specify mnemonic phrase to use for new account')
    cmd.option('-f, --file [file-name]', 'Specify file with mnemonic phrase to use for new account')
    cmd.action(balanceAction);
    return cmd;
}

async function balanceAction(options: OptionValues) {
    const api = await newApi(options.url);

    const seed = getSeedFromOptions(options);
    const pair = initKeyringPair(seed);
    const address = pair.address;

    const balance = await getBalance(address, api);

    if (balanceIsZero(balance)) {
        console.log("Account address " + address + " has no funds");
    } else {
        printBalance(balance);
    }

    process.exit(0);
}