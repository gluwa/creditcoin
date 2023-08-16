import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getBalance, logBalance } from "../utils/balance";
import {
    parseAddressOrExit,
    parseBoolean,
    requiredInput,
} from "../utils/parsing";

const fs = require('fs');


export function makeMapControllerCommand() {
    const cmd = new Command("map-controller");
    cmd.description("Map a controller account to its stash");
    cmd.action(mapControllerAction);
    return cmd;
}

async function mapControllerAction(options: OptionValues) {
    const { api } = await newApi(options.url);

    const data = fs.readFileSync('/Users/zach/Documents/creditcoin/controller_keys.txt', 'utf8')
    const keys = data.split('\n');

    let output: string[] = [];

    for (let i = 0; i < keys.length; i++) {
        const controller: string = keys[i];

        if (controller.length === 0) {
            continue;
        }

        const res = await api.query.staking.ledger(controller);

        if (res.isSome) {
            let stash = res.unwrap().stash.toString();
            output.push(`${controller},${stash}`);
        }
        if (res.isEmpty) {
            output.push(`${controller},`)
        }
    }

    let logger = fs.createWriteStream('/Users/zach/Documents/creditcoin/stash.csv', {
        flags: 'a' // 'a' means appending (old data will be preserved)
    })

    for (let j = 0; j < output.length; j++) {
        logger.write(`\n${output[j]}`)
    }

    logger.end()
    process.exit(0);
}


