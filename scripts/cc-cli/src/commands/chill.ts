import { Command, OptionValues } from "commander";
import { newApi } from "../api";
import { getControllerSeedFromEnvOrPrompt } from "../utils/account";
import { chill } from "../utils/validate";

export function makeChillCommand() {
  const cmd = new Command("chill");
  cmd.description(
    "Signal intention to stop validating from a Controller account"
  );
  cmd.action(chillAction);
  return cmd;
}

async function chillAction(options: OptionValues) {
  const { api } = await newApi(options.url);

  const controllerSeed = await getControllerSeedFromEnvOrPrompt();

  console.log("Creating chill transaction...");

  const chillTxHash = await chill(controllerSeed, api);

  console.log("Chill transaction sent with hash:", chillTxHash.toHex());
  process.exit(0);
}
