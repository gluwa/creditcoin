import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  getCallerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import { signSendAndWatch } from "../utils/tx";

export function makeDistributeRewardsCommand() {
  const cmd = new Command("distribute-rewards");
  cmd.description("Distribute all pending rewards for all validators");
  cmd.option(
    "-v, --validator-id [stash-address]",
    "Specify the Stash address of Validator to distribute rewards for"
  );
  cmd.option("-e, --era [era]", "Specify era to distribute rewards for");
  cmd.action(distributeRewardsAction);
  return cmd;
}

async function distributeRewardsAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  if (!options.validatorId) {
    console.log("Must specify a validator to distribute rewards for");
    process.exit(1);
  }

  if (!options.era) {
    console.log("Must specify an era");
    process.exit(1);
  }

  // Any account can call the distribute_rewards extrinsic
  const callerSeed = await getCallerSeedFromEnvOrPrompt();
  const distributeTx = api.tx.staking.payoutStakers(
    options.validatorId,
    options.era
  );

  const result = await signSendAndWatch(
    distributeTx,
    api,
    initKeyringPair(callerSeed)
  );

  console.log(result.info);
  process.exit(0);
}


export function makeDistributeRewardsMultiEraCommand() {
  const cmd = new Command("distribute-eras-rewards");
  cmd.description("Distribute all pending rewards for all validators multiple eras");
  cmd.option(
    "-v, --validator-id [stash-address]",
    "Specify the Stash address of Validator to distribute rewards for"
  );
  cmd.option("-sa, --start-era [era]", "Specify the start era to distribute rewards for");
  cmd.option("-ea, --end-era [era]", "Specify the end era to distribute rewards for");
  cmd.action(distributeRewardsMultiEraAction);
  return cmd;
}

async function distributeRewardsMultiEraAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  if (!options.validatorId) {
    console.log("Must specify a validator to distribute rewards for");
    process.exit(1);
  }

  if (!options.startEra) {
    console.log("Must specify an start era");
    process.exit(1);
  }

  if (!options.endEra) {
    console.log("Must specify an end era");
    process.exit(1);
  }

  // Any account can call the distribute_rewards extrinsic
  const callerSeed = await getCallerSeedFromEnvOrPrompt();

  let txs = [];
  for (let i = options.startEra; i <= options.endEra; i++) {
    const eraDistributeTx = api.tx.staking.payoutStakers(
      options.validatorId,
      i
    );
    txs.push(eraDistributeTx);
  }

  const batchTx = api.tx.utility.batchAll(txs);

  const result = await signSendAndWatch(
    batchTx,
    api,
    initKeyringPair(callerSeed)
  );

  console.log(result.info);
  process.exit(0);
}
