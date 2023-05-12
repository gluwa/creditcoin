import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { ApiPromise } from "creditcoin-js";
import { StakingPalletValidatorPrefs, validate } from "../utils/validate";

export function makeValidateCommand() {
    const cmd = new Command('validate');
    cmd.description('Signal intention to validate from a Controller account');
    cmd.option('-s, --seed [mneomonic]', 'Specify mnemonic phrase to use for new account')
    cmd.option('-f, --file [file-name]', 'Specify file with mnemonic phrase to use for new account')
    cmd.option('--commission [commission]', 'Specify commission for validator')
    cmd.option('--blocked', 'Specify if validator is blocked for new nominations')
    cmd.action(validateAction);
    return cmd;
}

async function validateAction(options: OptionValues) {
    const api = await newApi(options.url);

    const stashSeed = getSeedFromOptions(options);

    const commission = options.commission ? options.commission : 0;
    const blocked = options.blocked ? options.blocked : false;
    const preferences: StakingPalletValidatorPrefs = { commission: commission, blocked: blocked };

    console.log("Creating validate transaction...");

    const validateTxHash = await validate(stashSeed,preferences, api);

    console.log("Validate transaction sent with hash:", validateTxHash.toHex());
    process.exit(0);
}
