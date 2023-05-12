#!/usr/bin/env node

const {Command} = require('commander');

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


const program = new Command();


program.version('0.0.1').description('Creditcoin Staking Tool')

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
    .addCommand(makeWizardCommand());

program.parse(process.argv);
