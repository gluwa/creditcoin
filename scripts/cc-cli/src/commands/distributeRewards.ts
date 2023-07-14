import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import {
  getCallerSeedFromEnvOrPrompt,
  initKeyringPair,
} from "../utils/account";
import { requireEnoughFundsToSend, signSendAndWatch } from "../utils/tx";
import {
  parseAddressOrExit,
  parseIntegerOrExit,
  requiredInput,
} from "../utils/parsing";

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

  const { validator, era } = parseOptions(options);

  // Any account can call the distribute_rewards extrinsic
  const callerSeed = await getCallerSeedFromEnvOrPrompt();
  const distributeTx = api.tx.staking.payoutStakers(validator, era);

  await requireEnoughFundsToSend(distributeTx, callerSeed, api);

  const result = await signSendAndWatch(
    distributeTx,
    api,
    initKeyringPair(callerSeed)
  );

  console.log(result.info);
  process.exit(0);
}

function parseOptions(options: OptionValues) {
  const validator = parseAddressOrExit(
    requiredInput(
      options.validatorId,
      "Failed to distribute rewards: Must specify a validator address"
    )
  );

  const era = parseIntegerOrExit(
    requiredInput(
      options.era,
      "Failed to distribute rewards: Must specify an era"
    )
  );

  return { validator, era };
}
