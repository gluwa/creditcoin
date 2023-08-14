#!/usr/bin/env node

import { Command } from "commander";

// Subcommands
import { makeWizardCommand } from "./commands/wizard";
import { makeNewSeedCommand } from "./commands/newSeed";
import { makeBalanceCommand } from "./commands/balance";
import { makeValidateCommand } from "./commands/validate";
import { makeBondCommand } from "./commands/bond";
import { makeRotateKeysCommand } from "./commands/rotateKeys";
import { makeSetKeysCommand } from "./commands/setKeys";
import { makeShowAddressCommand } from "./commands/showAddress";
import { makeSendCommand } from "./commands/send";
import { makeChillCommand } from "./commands/chill";
import { makeDistributeRewardsCommand } from "./commands/distributeRewards";
import { makeUnbondCommand } from "./commands/unbond";
import { makeStatusCommand } from "./commands/status";
import { makeWithdrawUnbondedCommand } from "./commands/withdrawUnbonded";

const program = new Command();

program.description("Creditcoin Staking Tool");

// Option to set custom URL for Substrate node

// IMPORTANT: keep this list ordered alphabetically b/c
// it determines the order in which commands are printed in help text
program
  .addCommand(makeBalanceCommand())
  .addCommand(makeBondCommand())
  .addCommand(makeChillCommand())
  .addCommand(makeDistributeRewardsCommand())
  .addCommand(makeNewSeedCommand())
  .addCommand(makeRotateKeysCommand())
  .addCommand(makeSendCommand())
  .addCommand(makeSetKeysCommand())
  .addCommand(makeShowAddressCommand())
  .addCommand(makeStatusCommand())
  .addCommand(makeUnbondCommand())
  .addCommand(makeValidateCommand())
  .addCommand(makeWithdrawUnbondedCommand())
  .addCommand(makeWizardCommand());

// Set global options
program.commands.forEach((cmd) => {
  cmd.option("--no-input", "Disable interactive prompts");
  cmd.option("--ecdsa", "Use ECDSA keys instead of sr25519 seed phrase");
  cmd.option(
    "-u, --url [url]",
    "URL for the Substrate node",
    "ws://localhost:9944"
  );
});

program.parse(process.argv);
