import chalk from "chalk";
import { Command, Option, OptionValues } from "commander";
import { newApi } from "../api";
import { initCallerKeyring } from "../utils/account";
import { Wallet } from "ethers";
import { signAccountId } from "creditcoin-js/lib/utils";
import { AddressRegistered } from "creditcoin-js/lib/extrinsics/register-address";

const blockchains = ["Ethereum", "Rinkeby", "Luniverse", "Bitcoin", "Other"];

export function makeRegisterAddressCmd() {
    const blockchainOpt = new Option(
        "-b, --blockchain <chain> The blockchain that this external address belongs to",
    )
        .choices(blockchains)
        .env("BLOCKCHAIN");

    const privateKeyOpt = new Option(
        "-p, --private-key <key> The private key for the address that you want to register.",
    ).env("PRIVATE_KEY");

    return new Command("register-address")
        .description(
            "Register an external off-chain address as belonging to a CreditCoin address",
        )
        .addOption(privateKeyOpt)
        .addOption(blockchainOpt)
        .action(registerAddressAction);
}

async function registerAddressAction(options: OptionValues) {
    validateOptsOrExit(options);

    const {
        api,
        extrinsics: { registerAddress },
    } = await newApi(options.url);
    const signer = await initCallerKeyring(options);
    const wallet = new Wallet(options.privateKey);
    const proof = signAccountId(api, wallet, signer.address);
    console.log(signer.address);

    registerAddress(wallet.address, options.blockchain, proof, signer)
        .then(handleSuccess)
        .catch(handleError);
}

function handleSuccess(_: AddressRegistered) {
    console.log(chalk.green(`Address Registered Successfully!`));
    process.exit(0);
}

function handleError(reason: any) {
    fatalErr(`ERROR: The call to register address was unsuccessful: ${reason}`);
}

function validateOptsOrExit(options: OptionValues) {
    if (options.blockchain === undefined) {
        fatalErr(
            `ERROR: A blockchain must be specified (possible values: ${blockchains})`,
        );
    }

    if (options.privateKey === undefined) {
        fatalErr("ERROR: No external address specified");
    }
}

export function fatalErr(s: string) {
    errorMsg(s);
    process.exit(1);
}

function errorMsg(s: string) {
    console.log(chalk.red(s));
}
