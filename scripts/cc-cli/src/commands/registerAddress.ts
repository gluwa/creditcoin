import chalk from "chalk";
import { Command, Option, OptionValues } from "commander";
import { newApi } from "../api";
import { initCallerKeyring } from "../utils/account";
import { Wallet } from "ethers";
import { signAccountId } from "creditcoin-js/lib/utils";
import { AddressRegistered } from "creditcoin-js/lib/extrinsics/register-address";
import { utils } from "ethers";

const blockchains = ["Ethereum", "Rinkeby", "Luniverse", "Bitcoin", "Other"];

export function makeRegisterAddressCmd() {
  const blockchainOpt = new Option(
    "-b, --blockchain <chain> The blockchain that this external address belongs to",
  ).choices(blockchains);

  const privateKeyOpt = new Option(
    "-p, --private-key <key> The private key for the address that you want to register.",
  ).env("PRIVATE_KEY");

  return new Command("register-address")
    .description(
      "Link a CreditCoin address to an address from another blockchain",
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

  // Reads CC_SECRET env variable if it exists or propmpts the user to enter a mneumonic
  const signer = await initCallerKeyring(options);
  const wallet = new Wallet(options.privateKey);

  // create the cryptographic proof of ownership
  const proof = signAccountId(api, wallet, signer.address);

  registerAddress(wallet.address, options.blockchain, proof, signer)
    .then(handleSuccess)
    .catch(handleError);
}

function handleSuccess(addressRegistered: AddressRegistered) {
  console.log(
    chalk.green(
      `Address Registered Successfully!: ${addressRegistered.itemId}`,
    ),
  );
  process.exit(0);
}

function handleError(reason: any) {
  fatalErr(
    `ERROR: The call to register address was unsuccessful: ${reason as string}`,
  );
}

function validateOptsOrExit(options: OptionValues) {
  if (options.blockchain === undefined) {
    fatalErr(
      `ERROR: A blockchain must be specified (possible values: ${blockchains.toString()})`,
    );
  }

  if (options.privateKey === undefined) {
    fatalErr("ERROR: No external address specified");
  }

  if (!isValidPrivateKey(options.privateKey)) {
    fatalErr(`ERROR: Invalid private key: ${options.privateKey as string}`);
  }
}

export function fatalErr(s: string) {
  errorMsg(s);
  process.exit(1);
}

function errorMsg(s: string) {
  console.log(chalk.red(s));
}

// https://github.com/ethers-io/ethers.js/discussions/2939
export function isValidPrivateKey(pk: string): boolean {
  return utils.isHexString(pk, 32);
}
