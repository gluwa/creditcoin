import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  checkAddress,
  getSeedFromOptions,
  initECDSAKeyringPairFromPK,
  initKeyringPair,
} from "../utils/account";
import { toMicrounits } from "../utils/balance";
import { ApiPromise } from "creditcoin-js";

export function makeSendCommand() {
  const cmd = new Command("send");
  cmd.description("Send CTC from an account");
  cmd.option(
    "-s, --seed [mnemonic]",
    "Specify mnemonic phrase to use for sending CTC"
  );
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with mnemonic phrase to use for sending CTC"
  );
  cmd.option(
    "--use-ecdsa",
    "Use ECDSA signature scheme and a private key instead of a mnemonic phrase"
  );
  cmd.option("-a, --amount [amount]", "Amount to send");
  cmd.option("-t, --to [to]", "Specify recipient address");
  cmd.action(sendAction);
  return cmd;
}

async function sendAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  // Check options
  checkAmount(options);
  checkAddress(options.to, api, "send funds to");

  if (options.useEcdsa) {
    const hash = await sendFromECDSA(options, api);
    console.log("Transfer transaction hash: " + hash.toHex());
  } else {
    const hash = await sendFromSr25519(options, api);
    console.log("Transfer transaction hash: " + hash.toHex());
  }

  process.exit(0);
}

function checkAmount(options: OptionValues) {
  if (!options.amount) {
    console.log("Must specify amount to send");
    process.exit(1);
  }
}

async function sendFromSr25519(options: OptionValues, api: ApiPromise) {
  // Build account
  const seed = getSeedFromOptions(options);
  const stash = initKeyringPair(seed);

  // Send transaction
  const tx = api.tx.balances.transfer(
    options.to,
    toMicrounits(options.amount).toString()
  );
  const hash = await tx.signAndSend(stash);
  return hash;
}

async function sendFromECDSA(options: OptionValues, api: ApiPromise) {
  // Build account
  const seed = getSeedFromOptions(options);
  const stash = initECDSAKeyringPairFromPK(seed);
  console.log(stash.address);

  // Send transaction
  const tx = api.tx.balances.transfer(
    options.to,
    toMicrounits(options.amount).toString()
  );
  const hash = await tx.signAndSend(stash);
  return hash;
}
