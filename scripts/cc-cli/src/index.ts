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
import { makeReceiveCommand } from "./commands/receive";
import { makeSendCommand } from "./commands/send";
import { makeChillCommand } from "./commands/chill";

const program = new Command();

program.description("Creditcoin Staking Tool");

// Option to set custom URL for Substrate node

program
  .addCommand(makeNewSeedCommand())
  .addCommand(makeReceiveCommand())
  .addCommand(makeSendCommand())
  .addCommand(makeBalanceCommand())
  .addCommand(makeBondCommand())
  .addCommand(makeRotateKeysCommand())
  .addCommand(makeSetKeysCommand())
  .addCommand(makeValidateCommand())
  .addCommand(makeChillCommand())
  .addCommand(makeWizardCommand());

program.commands.forEach((cmd) => {
  cmd.option(
    "-u, --url [url]",
    "URL for the Substrate node",
    "ws://localhost:9944"
  );
});

program.parse(process.argv);
