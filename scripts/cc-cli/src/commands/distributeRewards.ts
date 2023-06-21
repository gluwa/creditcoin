import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { signSendAndWatch } from "../utils/tx";

export function makeDistributeRewardsCommand() {
  const cmd = new Command("distribute-rewards");
  cmd.description("Distribute all pending rewards for all validators");
  cmd.option("-s, --seed [mnemonic]", "Specify mnemonic phrase to use");
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with mnemonic phrase to use"
  );
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

  const signerSeed = getSeedFromOptions(options);

  if (!options.validatorId) {
    console.log("Must specify a validator to distribute rewards for");
    process.exit(1);
  }

  if (!options.era) {
    console.log("Must specify an era");
    process.exit(1);
  }

  const distributeTx = api.tx.staking.payoutStakers(
    options.validatorId,
    options.era
  );

  const result = await signSendAndWatch(
    distributeTx,
    api,
    initKeyringPair(signerSeed)
  );

  console.log(result.info);
  process.exit(0);
}
