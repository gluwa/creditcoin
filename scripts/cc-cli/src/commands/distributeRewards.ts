import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  getCallerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";

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

  const hash = await distributeTx.signAndSend(initKeyringPair(callerSeed));

  console.log("Payout stakers transaction sent with hash:", hash.toHex());
  process.exit(0);
}
