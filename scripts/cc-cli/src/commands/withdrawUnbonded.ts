import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { signSendAndWatch } from "../utils/tx";

export function makeWithdrawUnbondedCommand() {
  const cmd = new Command("withdraw-unbonded");
  cmd.description("Withdraw unbonded funds from a stash account");
  cmd.option(
    "-s, --seed [mnemonic]",
    "Specify mnemonic phrase to withdraw from"
  );
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with mnemonic phrase to withdraw from"
  );
  cmd.option("-a, --amount [amount]", "Amount to withdraw");
  cmd.action(withdrawUnbondedAction);
  return cmd;
}

async function withdrawUnbondedAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const stashSeed = getSeedFromOptions(options);
  const stashAccount = initKeyringPair(stashSeed);
  const slashingSpans = await api.query.staking.slashingSpans(
    stashAccount.address
  );
  const slashingSpansCount = slashingSpans.toHuman()
    ? slashingSpans.toHuman()
    : 0;
  const withdrawUnbondTx = api.tx.staking.withdrawUnbonded(slashingSpansCount);
  const result = await signSendAndWatch(withdrawUnbondTx, api, stashAccount);

  console.log(result.info);
  process.exit(0);
}
