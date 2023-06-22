import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getSeedFromOptions } from "../utils/account";
import { chill } from "../utils/validate";

export function makeChillCommand() {
  const cmd = new Command("chill");
  cmd.description(
    "Signal intention to stop validating from a Controller account"
  );
  cmd.option(
    "-s, --seed [seed phrase]",
    "Specify seed phrase of controller account"
  );
  cmd.option(
    "-f, --file [file-name]",
    "Specify file with seed phrase of controller account"
  );
  cmd.action(chillAction);
  return cmd;
}

async function chillAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const controllerSeed = getSeedFromOptions(options);

  console.log("Creating chill transaction...");

  const result = await chill(controllerSeed, api);

  console.log(result.info);
  process.exit(0);
}
