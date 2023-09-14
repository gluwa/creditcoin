import { Command, Option, OptionValues } from "commander";
import { fatalErr } from "./registerAddress";
import { newApi } from "../api";
import { initCallerKeyring } from "../utils/account";
import { isAddress } from "web3-validator";
import { utils } from "ethers";
import chalk from "chalk";

export function makeCollectCoinsCmd() {
  const externalAddressOpt = new Option(
    "-e, --external-address <addr> The previously registered external address that called the burn tx",
  ).env("EXTERNAL_ADDR");

  const burnTxHashOpt = new Option(
    "-b, --burn-tx-hash <hash> The hash of the burn transaction",
  ).env("BURN_TX_HASH");

  return new Command("collect-coins")
    .description("Swap GCRE for CTC")
    .addOption(externalAddressOpt)
    .addOption(burnTxHashOpt)
    .action(collectCoinsActionSync);
}

function collectCoinsActionSync(options: OptionValues) {
  collectCoinsAction(options)
    .then(() => {
      console.log(chalk.green("Success!"));
      process.exit(0);
    })
    .catch((reason) => {
      fatalErr(
        `ERROR: The call to request_collect_coins was unsuccessful: ${
          reason as string
        }`,
      );
    });
}

async function collectCoinsAction(options: OptionValues) {
  validateOptsOrExit(options);

  const {
    extrinsics: { requestCollectCoins },
  } = await newApi(options.url);
  const signer = await initCallerKeyring(options);

  const event = await requestCollectCoins(
    options.externalAddress,
    signer,
    options.burnTxHash,
  );
  await event.waitForVerification(800_000);
}

function validateOptsOrExit(options: OptionValues) {
  if (options.externalAddress === undefined) {
    fatalErr(`ERROR: No external address specified`);
  }

  if (options.burnTxHash === undefined) {
    fatalErr("ERROR: No burn transaction hash specified");
  }

  if (!isTxHashValid(options.burnTxHash)) {
    fatalErr(
      `ERROR: The transaction hash is invalid: ${options.burnTxHash as string}`,
    );
  }

  if (!isExternalAddressValid(options.externalAddress)) {
    fatalErr(
      `ERROR: The external address is invalid: ${
        options.externalAddress as string
      }`,
    );
  }
}

export function isTxHashValid(hash: string): boolean {
  // 32 byte hexadecimal, 64 character string, 66 with 0x prefix
  return utils.isHexString(hash, 32);
}

export function isExternalAddressValid(addr: string): boolean {
  return isAddress(addr);
}
