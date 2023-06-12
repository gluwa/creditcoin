import { Command, OptionValues } from "commander";
import { getSeedFromOptions, initKeyringPair } from "../utils/account";
import { newApi } from "../api";
import { initEthereumWallet } from "../utils/wallet";

import { Blockchain } from "creditcoin-js/lib/model";
import { main as deployCtcContract } from "creditcoin-js/lib/ctc-deploy";
import { tryRegisterAddress } from "creditcoin-js/lib/testUtils";

export function makeBurnGATECommand() {
    let cmd = new Command("burn-gate")
        .description("Register Ethereum wallet with Creditcoin and collect CTC by burning GATE")
        .option(
            "--debug",
            "WARNING: DO NOT USE! Only for testing NPoS! Will deploy a smart contract!"
        ).option(
            "-t, --burn-tx-hash [hex-string]",
            "G-CRE burn transaction hash on Ethereum"
        ).option(
            "-k, --private-key [string]",
            "Private key of your Ethereum wallet"
        ).option(
            "-s, --seed [mnemonic]",
            "Specify mnemonic phrase to use for Creditcoin account"
        ).option(
            "-f, --file [file-name]",
            "Specify file with mnemonic phrase to use for Creditcoin account"
        ).action(burnGATEAction);

    return cmd;
}

async function burnGATEAction(options: OptionValues) {
    const ccApi = await newApi(options.url);
    const {
        api,
        extrinsics: { requestBurnGate },
        utils: { signAccountId },
    } = ccApi;

    if (!options.privateKey) {
        console.log("You must specify privateKey");
        process.exit(1);
    }

    if (options.burnTxHash && options.debug) {
        console.log(
            "--debug is incompatible with --burn-tx-hash. Use only --debug!"
        );
        process.exit(1);
    }

    if (!options.burnTxHash && !options.debug) {
        console.log("You must specify G-CRE burn transaction hash");
        process.exit(1);
    }

    const seed = getSeedFromOptions(options);
    const pair = initKeyringPair(seed);

    // this assumes we're running against a local blockchain network + hardhat
    // will deploy the smart contract & configure creditcoin-node with it before it
    // can collect coins
    if (options.debug) {
        await deployCtcContract(
            undefined, // no pre-existing contract address
            "http://127.0.0.1:8545", // address of gluwa/hardhat-dev
            // must be Private key for Account #0: from gluwa/hardhat-dev (10000 ETH)
            options.privateKey
        );
        const ctcContractAddress = process.env.CREDITCOIN_CTC_CONTRACT_ADDRESS;
        options.burnTxHash = process.env.CREDITCOIN_CTC_BURN_TX_HASH;

        // configure the contract used for CollectCoins
        const contract = api.createType(
            "PalletCreditcoinOcwTasksCollectCoinsGATEContract",
            {
                address: ctcContractAddress,
                chain: "Ethereum" as Blockchain,
            }
        );

        // warning: Creditcoin account must be allowed sudo calls
        await api.tx.sudo
            .sudo(api.tx.creditcoin.setCollectCoinsContract(contract))
            .signAndSend(pair, { nonce: -1 });
    }

    const ethereumWallet = initEthereumWallet(options.privateKey);
    console.log(`👛 Ethereum wallet address: ${ethereumWallet.address}`);

    console.log("... trying to register an account in Creditcoin network");
    const creditcoinAddress = await tryRegisterAddress(
        ccApi,
        ethereumWallet.address,
        // at some point we'll support other networks
        "Ethereum" as Blockchain,
        signAccountId(ethereumWallet, pair.address),
        pair,
        true
    );
    console.log(
        `📒 AddressRegistered! Creditcoin accountId: ${creditcoinAddress.item.accountId}`
    );

    // these must be the same, see integration-tests/src/test/collect-coins.test.ts
    if (
        creditcoinAddress.item.externalAddress.toLowerCase() !==
        ethereumWallet.address.toLowerCase()
    ) {
        throw new Error(
            `Creditcoin.externalAddress (${creditcoinAddress.item.externalAddress}) different than Ethereum address (${ethereumWallet.address})`
        );
    }
    console.log(`✅ Creditcoin & Ethereum addresses match!`);

    // TODO: what happens if the user tries again ? collect-coins is designed to fail
    // so we must stop before executing it again !!!!

    // TODO: call .burn to make it easier for new users
    const burnTxHash = options.burnTxHash;

    console.log("... trying to collect CTC");
    const burnGATEEvent = await requestBurnGate(
        creditcoinAddress.item.externalAddress,
        pair,
        burnTxHash
    );

    const collectCoinsVerified = await burnGATEEvent
        .waitForVerification(800_000)
        .catch();
    if (!collectCoinsVerified) {
        throw new Error("Waiting for verification failed");
    }

    console.log(`🪙 CCTC collected!`);

    process.exit(0);
}