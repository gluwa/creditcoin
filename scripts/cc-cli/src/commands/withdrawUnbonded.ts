import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getValidatorStatus, requireStatus } from "../utils/validatorStatus";
import { requireEnoughFundsToSend, signSendAndWatch } from "../utils/tx";
import { initControllerKeyring } from "../utils/account";

export function makeWithdrawUnbondedCommand() {
  const cmd = new Command("withdraw-unbonded");
  cmd.description("Withdraw unbonded funds from a stash account");
  cmd.action(withdrawUnbondedAction);
  return cmd;
}

async function withdrawUnbondedAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const controller = await initControllerKeyring(options);
  const controllerStatus = await getValidatorStatus(controller.address, api);

  if (!controllerStatus.stash) {
    console.error(
      `Could not find stash account associated with the provided controller address: ${controller.address}. Please ensure the address is actually a controller.`
    );
    process.exit(1);
  }

  const status = await getValidatorStatus(controllerStatus.stash, api);
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

  await requireEnoughFundsToSend(withdrawUnbondTx, controller.address, api);

  const result = await signSendAndWatch(withdrawUnbondTx, api, controller);

  console.log(result.info);
  process.exit(0);
}
