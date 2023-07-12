import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  getControllerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import { getBalance } from "../utils/balance";
import { getStatus, requireStatus } from "../utils/status";
import { requireEnoughFundsToSend, signSendAndWatch } from "../utils/tx";
import { ApiPromise, BN } from "creditcoin-js";
import { promptContinue } from "../utils/promptContinue";
import { parseAmountOrExit, requiredInput } from "../utils/parsing";

export function makeUnbondCommand() {
  const cmd = new Command("unbond");
  cmd.description("Schedule a portion of the stash to be unlocked");
  cmd.option("-a, --amount [amount]", "Amount to send");
  cmd.action(unbondAction);
  return cmd;
}

async function unbondAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const amount = parseAmountOrExit(
    requiredInput(options.amount, "Failed to unbond: Must specify an amount")
  );

  // Build account
  const controllerSeed = await getControllerSeedFromEnvOrPrompt();
  const controllerKeyring = initKeyringPair(controllerSeed);
  const controllerAddress = controllerKeyring.address;

  const controllerStatus = await getStatus(controllerAddress, api);
  if (!controllerStatus.stash) {
    console.error(
      `Cannot unbond, ${controllerAddress} is not a controller account`
    );
    process.exit(1);
  }
  const stashStatus = await getStatus(controllerStatus.stash, api);
  requireStatus(stashStatus, "bonded");

  // Check if amount specified exceeds total bonded funds
  await checkIfUnbodingMax(controllerStatus.stash, amount, api);

  // Unbond transaction
  const tx = api.tx.staking.unbond(amount.toString());
  await requireEnoughFundsToSend(tx, controllerAddress, api);

  const result = await signSendAndWatch(tx, api, controllerKeyring);

  console.log(result.info);
  process.exit(0);
}

async function checkIfUnbodingMax(
  address: string,
  unbondAmount: BN,
  api: ApiPromise
) {
  const balance = await getBalance(address, api);
  if (balance.bonded.lt(unbondAmount)) {
    console.error(
      "Warning: amount specified exceeds total bonded funds, will unbond all funds"
    );
    await promptContinue();
  }
}
