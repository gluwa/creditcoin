import { Command, OptionValues } from "commander";
import { ApiPromise, BN } from "creditcoin-js";
import { newApi } from "../api";
import {
  getCallerSeedFromEnvOrPrompt,
  initECDSAKeyringPairFromPK,
  initKeyringPair,
} from "../utils/account";
import { getBalance } from "../utils/balance";
import { signSendAndWatch } from "../utils/tx";
import {
  parseAddresOrExit,
  parseAmountOrExit,
  parseBoolean,
  requiredInput,
} from "../utils/parsing";

export function makeSendCommand() {
  const cmd = new Command("send");
  cmd.description("Send CTC from an account");
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

  const { amount, recipient, useEcdsa } = parseOptions(options);

  const seed = await getCallerSeedFromEnvOrPrompt();
  const caller = useEcdsa
    ? initECDSAKeyringPairFromPK(seed)
    : initKeyringPair(seed);

  await checkEnoughFundsToSend(caller.address, amount, api);

  const tx = api.tx.balances.transfer(recipient, amount.toString());

  const result = await signSendAndWatch(tx, api, caller);
  console.log(result.info);

  process.exit(0);
}

function parseOptions(options: OptionValues) {
  const amount = parseAmountOrExit(
    requiredInput(options.amount, "Failed to send CTC: Must specify an amount")
  );

  const recipient = parseAddresOrExit(
    requiredInput(options.to, "Failed to send CTC: Must specify a recipient")
  );

  const useEcdsa = parseBoolean(options.useEcdsa);

  return { amount, recipient, useEcdsa };
}

async function checkEnoughFundsToSend(
  address: string,
  amount: BN,
  api: ApiPromise
) {
  const balance = await getBalance(address, api);
  if (balance.transferable.lt(amount)) {
    console.log(
      `Caller ${address} has insufficient funds to send ${amount.toString()}`
    );
    process.exit(1);
  }
}
