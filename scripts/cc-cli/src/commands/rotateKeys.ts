import { Command, OptionValues } from "commander";
import { newApi } from "../api";

export function makeRotateKeysCommand() {
  const cmd = new Command("rotate-keys");
  cmd.description("Rotate session keys for a specified node");
  cmd.action(rotateKeysAction);
  return cmd;
}

async function rotateKeysAction(options: OptionValues) {
  const { api } = await newApi(options.url);
  const newKeys = await api.rpc.author.rotateKeys();
  console.log("New keys: " + newKeys.toString());
  process.exit(0);
}
