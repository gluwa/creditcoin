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
import { ApiPromise } from "creditcoin-js";

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

  await checkEraToBeValid(era, api);

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

async function checkEraToBeValid(era: number, api: ApiPromise) {
  const currentEra = (await api.query.staking.currentEra()).value.toNumber();
  const historyDepth = api.consts.staking.historyDepth.toNumber();
  const minEra = currentEra - historyDepth;
  if (era < minEra) {
    console.log(`Era ${era} is too old to distribute rewards for`);
    process.exit(1);
  } else if (era >= currentEra) {
    console.log(
      `Era ${era} is in the future or has not ended, can only distribute rewards for past eras`
    );
    process.exit(1);
  }
}
