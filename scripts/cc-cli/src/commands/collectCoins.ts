import { Command, Option, OptionValues } from "commander";
import { fatalErr } from "./registerAddress";
import { newApi } from "../api";
import { initCallerKeyring } from "../utils/account";
import { requiredInput } from "../utils/parsing";
import { isAddress } from "web3-validator";
import { utils } from "ethers";
import chalk from "chalk";
import { U64 } from '@polkadot/types-codec';

export function makeCollectCoinsCmd() {
  const externalAddressOpt = new Option(
    "-e, --external-address <addr>",
    "The previously registered external address that called the burn tx",
  );

  const burnTxHashOpt = new Option(
    "-b, --burn-tx-hash <hash>",
    "The hash of the burn transaction",
  );

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
        `ERROR: The call to request_collect_coins was unsuccessful: ${reason as string
        }`,
      );
    });
}

async function collectCoinsAction(options: OptionValues) {
  validateOptsOrExit(options);

  const { api,
    extrinsics: { requestCollectCoins },
  } = await newApi(options.url);
  const signer = await initCallerKeyring(options);

  const event = await requestCollectCoins(
    options.externalAddress,
    signer,
    options.burnTxHash,
  );

  const blockTime = (api.consts.babe.expectedBlockTime as U64).toNumber();
  const unverifiedTaskTimeout = Number(api.consts.creditcoin.unverifiedTaskTimeout.toString());

  await event.waitForVerification(blockTime * unverifiedTaskTimeout);
}

function validateOptsOrExit(options: OptionValues) {
  const externalAddress = requiredInput(
    options.externalAddress.trim(),
    "ERROR: externalAddress is required",
  );
  const burnTxHash = requiredInput(
    options.burnTxHash.trim(),
    "ERROR: Burn transaction hash is required",
  );

  if (!isTxHashValid(burnTxHash)) {
    fatalErr(`ERROR: The transaction hash is invalid: ${burnTxHash}`);
  }

  if (!isExternalAddressValid(externalAddress)) {
    fatalErr(`ERROR: The external address is invalid: ${externalAddress}`);
  }
}

export function isTxHashValid(hash: string): boolean {
  // 32 byte hexadecimal, 64 character string, 66 with 0x prefix
  return utils.isHexString(hash, 32);
}

export function isExternalAddressValid(addr: string): boolean {
  return isAddress(addr);
}
