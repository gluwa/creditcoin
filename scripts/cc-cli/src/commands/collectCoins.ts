import { Command, Option, OptionValues } from "commander";
import { fatalErr } from "./registerAddress";
import { newApi } from "../api";
import { initCallerKeyring } from "../utils/account";
import { CollectCoinsEvent } from "creditcoin-js/lib/extrinsics/request-collect-coins";
import chalk from "chalk";
import { isAddress } from "web3-validator";

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
    .action(collectCoinsAction);
}

async function collectCoinsAction(options: OptionValues) {
  validateOptsOrExit(options);

  const {
    extrinsics: { requestCollectCoins },
  } = await newApi(options.url);
  const signer = await initCallerKeyring(options);

  requestCollectCoins(options.externalAddress, signer, options.burnTxHash)
    .then(handleSuccess)
    .catch(handleError);
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

async function handleSuccess(value: CollectCoinsEvent) {
  await value.waitForVerification(800_000);
  console.log(chalk.green("Success!"));
}

function handleError(reason: any) {
  fatalErr(
    `ERROR: The call to request_collect_coins was unsuccessful: ${
      reason as string
    }`,
  );
}

export function isTxHashValid(hash: string): boolean {
  return /^0x([A-Fa-f0-9]{64})$/.test(hash);
}

export function isExternalAddressValid(addr: string): boolean {
  return isAddress(addr);
}
