import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { signSendAndWatch } from "../utils/tx";
import {
  getControllerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";

export function makeWithdrawUnbondedCommand() {
  const cmd = new Command("withdraw-unbonded");
  cmd.description("Withdraw unbonded funds from a stash account");
  cmd.option("-a, --amount [amount]", "Amount to withdraw");
  cmd.action(withdrawUnbondedAction);
  return cmd;
}

async function withdrawUnbondedAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const controllerSeed = await getControllerSeedFromEnvOrPrompt();
  const controller = initKeyringPair(controllerSeed);
  const slashingSpans = await api.query.staking.slashingSpans(
    controller.address
  );
  const slashingSpansCount = slashingSpans.toHuman()
    ? slashingSpans.toHuman()
    : 0;
  const withdrawUnbondTx = api.tx.staking.withdrawUnbonded(slashingSpansCount);
  const result = await signSendAndWatch(withdrawUnbondTx, api, stashAccount);

  console.log(result.info);
  process.exit(0);
}
