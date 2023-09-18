import chalk from "chalk";
import { Command, Option, OptionValues } from "commander";
import { newApi } from "../api";
import { initCallerKeyring } from "../utils/account";
import { Wallet } from "ethers";
import { signAccountId } from "creditcoin-js/lib/utils";
import { AddressRegistered } from "creditcoin-js/lib/extrinsics/register-address";
import { utils } from "ethers";
import { getErrorMessage } from "../utils/error";
import prompts from "prompts";

const blockchains = ["Ethereum", "Rinkeby", "Luniverse", "Bitcoin", "Other"];

export function makeRegisterAddressCmd() {
  const blockchainOpt = new Option(
    "-b, --blockchain <chain> The blockchain that this external address belongs to",
  ).choices(blockchains);

  const privateKeyOpt = new Option(
    "-p, --private-key <key> The private key for the Ethereum address that you want to register.",
  ).env("ETH_PRIVATE_KEY");

  return new Command("register-address")
    .description(
      "Link a CreditCoin address to an address from another blockchain",
    )
    .addOption(privateKeyOpt)
    .addOption(blockchainOpt)
    .option(
      "--eth-mnemonic",
      "Specify the ethereum address using a mnemonic rather than a private key",
    )
    .action(registerAddressAction);
}

async function registerAddressAction(options: OptionValues) {
  validateOptsOrExit(options);

  const {
    api,
    extrinsics: { registerAddress },
  } = await newApi(options.url);

  // Reads CC_SECRET env variable if it exists or propmpts the user to enter a mnemonic
  const signer = await initCallerKeyring(options);

  // Reads ETH_PRIVATE_KEY env variable if found or prompts the user to enter an ethereum mnemonic
  const wallet = await initCallerEthWallet(options);

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

export async function initCallerEthWallet(
  options: OptionValues,
): Promise<Wallet> {
  try {
    return await initWalletFromEnvOrPrompt("ETH_PRIVATE_KEY", options);
  } catch (e) {
    console.error(getErrorMessage(e));
    process.exit(1);
  }
}

async function initWalletFromEnvOrPrompt(
  envVar: string,
  options: OptionValues,
): Promise<Wallet> {
  const interactive = options.input;
  const useMnemonic = options.ethMnemonic;
  const inputName = useMnemonic ? "mnemonic" : "private key";
  const generateWallet = useMnemonic
    ? newWalletFromMnemonic
    : newWalletFromPrivateKey;
  const validateInput = useMnemonic ? isMnemonicValid : isValidPrivateKey;

  if (!interactive && process.env[envVar] === undefined) {
    throw new Error(
      "Error: Must specify a private key using the environment variable ETH_PRIVATE_KEY or an interactive prompt",
    );
  }

  if (process.env[envVar] !== undefined) {
    const seed = process.env[envVar] as string;

    if (!validateInput(seed)) {
      throw new Error("Error: Private key is invalid");
    }

    return generateWallet(seed);
  } else if (interactive) {
    const promptResult = await prompts([
      {
        type: "password",
        name: "seed",
        message: `Specify the ${inputName} for the ethereum address`,
        validate: validateInput,
      },
    ]);

    const seed = promptResult.seed;

    if (!seed) {
      throw new Error("The mnemonic could not be retrieved from the prompt");
    }

    return generateWallet(seed);
  }

  throw new Error("The ethereum wallet could not be");
}

function newWalletFromMnemonic(mnemonic: string): Wallet {
  let wallet: Wallet;

  try {
    wallet = Wallet.fromMnemonic(mnemonic);
  } catch (e) {
    throw new Error(
      `Error: Could not create wallet from mnemonic: ${getErrorMessage(e)}`,
    );
  }

  return wallet;
}

function newWalletFromPrivateKey(pk: string): Wallet {
  let wallet: Wallet;

  try {
    wallet = new Wallet(pk);
  } catch (e) {
    throw new Error(
      `Error: Could not create wallet from private key: ${getErrorMessage(e)}`,
    );
  }

  return wallet;
}

function isMnemonicValid(mnemonic: string): boolean {
  return utils.isValidMnemonic(mnemonic);
}
