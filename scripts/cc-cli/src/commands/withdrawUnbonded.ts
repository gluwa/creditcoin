import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getStashSeedFromEnvOrPrompt, initKeyringPair } from "../utils/account";

export function makeWithdrawUnbondedCommand() {
  const cmd = new Command("withdraw-unbonded");
  cmd.description("Withdraw unbonded funds from a stash account");
  cmd.option("-a, --amount [amount]", "Amount to withdraw");
  cmd.action(withdrawUnbondedAction);
  return cmd;
}

async function withdrawUnbondedAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const stashSeed = await getStashSeedFromEnvOrPrompt();
  const stashAccount = initKeyringPair(stashSeed);
  const slashingSpans = await api.query.staking.slashingSpans(
    stashAccount.address
  );
  const slashingSpansCount = slashingSpans.toHuman()
    ? slashingSpans.toHuman()
    : 0;
  const withdrawUnbondTx = api.tx.staking.withdrawUnbonded(slashingSpansCount);
  const hash = await withdrawUnbondTx.signAndSend(stashAccount);

  console.log("Withdraw unbonded transaction sent with hash:", hash.toHex());
  process.exit(0);
}
