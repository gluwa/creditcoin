import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromEnvOrPrompt, initKeyringPair } from "../utils/account";

export function makeDistributeRewardsCommand() {
  const cmd = new Command("distribute-rewards");
  cmd.description("Distribute all pending rewards for all validators");
  cmd.option(
    "-v, --validator-id [validator-id]",
    "Specify validator to distribute rewards for"
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

  const signerSeed = await getSeedFromEnvOrPrompt(process.env.CC_SEED, "Specify caller's seed phrase");
  const distributeTx = api.tx.staking.payoutStakers(
    options.validatorId,
    options.era
  );

  const hash = await distributeTx.signAndSend(initKeyringPair(signerSeed));

  console.log("Payout stakers transaction sent with hash:", hash.toHex());
  process.exit(0);
}
