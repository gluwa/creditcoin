import { JsonRpcProvider } from "@ethersproject/providers";
import { ContractFactory, Wallet } from "ethers";
import { GluwaCreditVestingToken } from "./test/integration-tests/ethereum/ctc/typechain";
import CtcArtifact from "./test/integration-tests/ethereum/ctc/contracts/GluwaCreditVestingToken.sol/GluwaCreditVestingToken.json";
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

function setArg(key: string, value: any) {
    (global as any)[key] = value;
}

function setArgIfUndef(key: string, value: any) {
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
        `ws://127.0.0.1:${process.env.CREDITCOIN_WS_PORT ? process.env.CREDITCOIN_WS_PORT : "9944"
        }`,
    ],
]);

export function arg(key: string) {
    return (global as any)[key];
}

async function setAuthorities() {
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

function retry(retries: any, executor: any): any {
    console.log(`${retries as string} retries left!`);

    if (typeof retries !== "number") {
        throw new TypeError("retries is not a number");
    }

    return new Promise(executor).catch((error) =>
        retries > 0 ? retry(retries - 1, executor) : Promise.reject(error),
    );
}

async function setup() {
    process.env.NODE_ENV = "test";

    // Makes console output look better
    console.log("");

    globalDefaults.forEach((value: any, key: string) => {
        setArgIfUndef(key, value);
    });

    if (process.env.INTEGRATION_TEST === undefined) {
        console.log("Skipping token deployment and authority setup");
        return;
    }

    await deployCtcToken((global as any).CREDITCOIN_CTC_CONTRACT_ADDRESS);
    setArg(
        "CREDITCOIN_CTC_CONTRACT_ADDRESS",
        process.env.CREDITCOIN_CTC_CONTRACT_ADDRESS,
    );
    setArgIfUndef(
        "CREDITCOIN_CTC_BURN_TX_HASH",
        process.env.CREDITCOIN_CTC_BURN_TX_HASH,
    );

    await retry(5, async (resolve: any, reject: any) => {
        await setAuthorities().then(resolve).catch(reject);
    }).catch(() => {
        console.log("Could not setup testing authorities");
        process.exit(1);
    });
}

export default setup;
