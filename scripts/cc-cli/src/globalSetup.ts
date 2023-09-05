import { JsonRpcProvider } from "@ethersproject/providers";
import { ContractFactory, Signer, Wallet } from "ethers";
import { GluwaCreditVestingToken } from "./test/integration-tests/ethereum/ctc/typechain";
import CtcArtifact from "./test/integration-tests/ethereum/ctc/contracts/GluwaCreditVestingToken.sol/GluwaCreditVestingToken.json";
import { Keyring, KeyringPair } from "creditcoin-js";

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

async function deployCtcToken(existingAddress: string | undefined) {
  const provider = new JsonRpcProvider(
    (global as any).CREDITCOIN_ETHEREUM_NODE_URL,
  );
  const deployer = new Wallet(
    (global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY,
    provider,
  );

  const factory = new ContractFactory(
    CtcArtifact.abi,
    CtcArtifact.bytecode,
    deployer,
  );

  let ctcToken: GluwaCreditVestingToken;

  if (existingAddress !== undefined) {
    ctcToken = factory.attach(existingAddress) as GluwaCreditVestingToken;
  } else {
    const deployerAddress = await deployer.getAddress();
    ctcToken = (await factory.deploy(
      deployerAddress,
      deployerAddress,
    )) as GluwaCreditVestingToken;
  }

  process.env.CREDITCOIN_CTC_CONTRACT_ADDRESS = ctcToken.address;

  const tx = await ctcToken.burn(1);
  const txHash = tx.hash;
  await tx.wait();

  console.log("Burn Tx Hash", txHash);
  process.env.CREDITCOIN_CTC_BURN_TX_HASH = txHash;
}

function setGlobalDefault(key: string, value: any) {
  (global as any)[key] = value;
}

function setGlobalDefaultIfUndefined(key: string, value: any) {
  if ((global as any)[key] === undefined) {
    setGlobalDefault(key, value);
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
]);

async function setup() {
  process.env.NODE_ENV = "test";

  // Makes console output look better
  console.log("");

  globalDefaults.forEach((value: any, key: string) => {
    setGlobalDefaultIfUndefined(key, value);
  });

  await deployCtcToken((global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS);
  setGlobalDefault(
    "CREDITCOIN_CTC_CONTRACT_ADDRESS",
    process.env.CREDITCOIN_CTC_CONTRACT_ADDRESS,
  );
  setGlobalDefaultIfUndefined(
    "CREDITCOIN_CTC_BURN_TX_HASH",
    process.env.CREDITCOIN_CTC_BURN_TX_HASH,
  );
}

export default setup;
