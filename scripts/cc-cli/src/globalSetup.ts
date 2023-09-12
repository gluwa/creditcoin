import { ApiPromise, Keyring, KeyringPair, WsProvider } from "creditcoin-js";
import { setupAuthority } from "creditcoin-js/lib/examples/setup-authority";

const createSigner = (
  keyring: Keyring,
  who: "lender" | "borrower" | "sudo",
): KeyringPair => {
  switch (who) {
    case "lender":
      return keyring.addFromUri("//Alice");
    case "borrower":
      return keyring.addFromUri("//Bob");
    case "sudo":
      return keyring.addFromUri("//Alice");
    default:
      throw new Error(`Unexpected value "${who}"`); // eslint-disable-line
  }
};

export function setArg(key: string, value: any) {
  (global as any)[key] = value;
}

export function setArgIfUndef(key: string, value: any) {
  if ((global as any)[key] === undefined) {
    setArg(key, value);
  }
}

const globalDefaults = new Map<string, any>([
  ["CREDITCOIN_EXECUTE_SETUP_AUTHORITY", true],
  ["CREDITCOIN_CREATE_SIGNER", createSigner],
  ["CREDITCOIN_ETHEREUM_NODE_URL", "http://127.0.0.1:8545"],
  [
    "CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY",
    "0xabf82ff96b463e9d82b83cb9bb450fe87e6166d4db6d7021d0c71d7e960d5abe",
  ],
  [
    "CREDITCOIN_API_URL",
    `ws://127.0.0.1:${
      process.env.CREDITCOIN_WS_PORT ? process.env.CREDITCOIN_WS_PORT : "9944"
    }`,
  ],
]);

export function arg(key: string) {
  return (global as any)[key];
}

export async function setAuthorities() {
  const api = await ApiPromise.create({
    provider: new WsProvider((global as any).CREDITCOIN_API_URL),
    throwOnConnect: true,
  });
  if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
    const keyring = new Keyring({ type: "sr25519" });
    const sudo = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, "sudo");
    await setupAuthority(api, sudo);
  }
  await api.disconnect();
}

export function retry(retries: any, executor: any): any {
  console.log(`${retries as string} retries left!`);

  if (typeof retries !== "number") {
    throw new TypeError("retries is not a number");
  }

  return new Promise(executor).catch((error) =>
    retries > 0 ? retry(retries - 1, executor) : Promise.reject(error),
  );
}

function setup() {
  process.env.NODE_ENV = "test";

  // Makes console output look better
  console.log("");

  globalDefaults.forEach((value: any, key: string) => {
    setArgIfUndef(key, value);
  });
}

export default setup;
