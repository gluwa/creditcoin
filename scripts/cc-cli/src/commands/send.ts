import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { requireEnoughFundsToSend, signSendAndWatch } from "../utils/tx";
import {
  parseAddressOrExit,
  parseAmountOrExit,
  parseBoolean,
  requiredInput,
} from "../utils/parsing";
import { setInteractivity } from "../utils/interactive";
import { initCallerKeyring } from "../utils/account";

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

  const { amount, recipient } = parseOptions(options);

  const caller = await initCallerKeyring(options);

  const tx = api.tx.balances.transfer(recipient, amount.toString());

  await requireEnoughFundsToSend(tx, caller.address, api, amount);

  const result = await signSendAndWatch(tx, api, caller);
  console.log(result.info);

  process.exit(0);
}

function parseOptions(options: OptionValues) {
  const amount = parseAmountOrExit(
    requiredInput(options.amount, "Failed to send CTC: Must specify an amount")
  );

  const recipient = parseAddressOrExit(
    requiredInput(options.to, "Failed to send CTC: Must specify a recipient")
  );

  return { amount, recipient };
}
