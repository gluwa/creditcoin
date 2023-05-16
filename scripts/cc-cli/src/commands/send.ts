import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";

export function makeSendCommand() {
  const cmd = new Command("send");
  cmd.description("Send CTC from an account");
  cmd.option(
    "-s, --seed [mnemonic]",
    "Specify mnemonic phrase to use for new account"
  );
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with mnemonic phrase to use for new account"
  );
  cmd.option("-a, --amount [amount]", "Amount to send");
  cmd.option("-t, --to [to]", "Specify recipient address");
  cmd.action(sendAction);
  return cmd;
}

async function sendAction(options: OptionValues) {
  const api = await newApi(options.url);

  // Build account
  const seed = getSeedFromOptions(options);
  const stash = initKeyringPair(seed);

  // Send transaction
  const tx = api.tx.balances.transfer(options.to, options.amount);
  const hash = await tx.signAndSend(stash);

  console.log("Transfer transaction hash: " + hash.toHex());
  process.exit(0);
}
