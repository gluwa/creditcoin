import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getStatus, requireStatus } from "../utils/status";
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

  const status = await getStatus(controller.address, api);
  requireStatus(
    status,
    "canWithdraw",
    "Cannot perform action, there are no unlocked funds to withdraw"
  );

  const slashingSpans = await api.query.staking.slashingSpans(
    controller.address
  );
  const slashingSpansCount = slashingSpans.isSome
    ? slashingSpans.unwrap().lastNonzeroSlash
    : 0;
  const withdrawUnbondTx = api.tx.staking.withdrawUnbonded(slashingSpansCount);
  const hash = await withdrawUnbondTx.signAndSend(controller);

  console.log("Withdraw unbonded transaction sent with hash:", hash.toHex());
  process.exit(0);
}
