import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  getControllerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import { getBalance, parseCTCString } from "../utils/balance";
import { getStatus, requireStatus } from "../utils/status";
import { ApiPromise, BN } from "creditcoin-js";
import { promptContinue } from "../utils/promptContinue";

export function makeUnbondCommand() {
  const cmd = new Command("unbond");
  cmd.description("Schedule a portion of the stash to be unlocked");
  cmd.option("-a, --amount [amount]", "Amount to send");
  cmd.action(unbondAction);
  return cmd;
}

async function unbondAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  // Check options
  checkAmount(options);

  const amount = parseCTCString(options.amount);

  // Build account
  const controllerSeed = await getControllerSeedFromEnvOrPrompt();
  const controllerKeyring = initKeyringPair(controllerSeed);
  const controllerAddress = controllerKeyring.address;

  const controllerStatus = await getStatus(controllerAddress, api);
  if (!controllerStatus.stash) {
    console.error(`Cannot unbond, ${controllerAddress} is not staked`);
    process.exit(1);
  }
  const stashStatus = await getStatus(controllerStatus.stash, api);
  requireStatus(stashStatus, "bonded");

  // Check if amount specified exceeds total bonded funds
  await checkIfUnbodingMax(controllerAddress, amount, api);

  // Unbond transaction
  const tx = api.tx.staking.unbond(amount.toString());

  const hash = await tx.signAndSend(controllerKeyring);

  console.log("Unbond transaction hash: " + hash.toHex());
  process.exit(0);
}

function checkAmount(options: OptionValues) {
  if (!options.amount) {
    console.log("Must specify amount to send");
    process.exit(1);
  }
}

async function checkIfUnbodingMax(
  address: string,
  unbondAmount: BN,
  api: ApiPromise
) {
  const balance = await getBalance(address, api);
  if (balance.miscFrozen.lt(unbondAmount)) {
    console.error(
      "Warning: amount specified exceeds total bonded funds, will unbond all funds"
    );
    await promptContinue();
  }
}
