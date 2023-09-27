import { KeyringPair, creditcoinApi, Keyring } from 'creditcoin-js';
import { Blockchain } from 'creditcoin-js/lib/model';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { testData, tryRegisterAddress } from 'creditcoin-js/lib/testUtils';

import { deployGATEToken } from 'creditcoin-js/lib/ctc-deploy';
import { JsonRpcProvider } from '@ethersproject/providers';
import { Wallet } from 'ethers';
import { mnemonicGenerate } from '@polkadot/util-crypto';
import { signAccountId } from 'creditcoin-js/lib/utils';
import { GATEContract } from 'creditcoin-js/lib/extrinsics/request-collect-coins-v2';
import { testIf } from '../utils';
import { collectCoinsV2Example } from 'creditcoin-js/lib/examples/collect-coins-v2';

describe('Test GATE Token', (): void => {
    let ccApi: CreditcoinApi;
    let sudoSigner: KeyringPair;

    // Needed to interact with the ethererum private node
    const testingData = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );
    const { keyring } = testingData;
    const provider = new JsonRpcProvider((global as any).CREDITCOIN_ETHEREUM_NODE_URL);
    const deployer = new Wallet((global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY, provider);
    const burnAmount = 200;

    // Holds the reference to the deployed GATE contract
    let gateToken: any;
    const gateKeyring = new Keyring({ type: 'ed25519', ss58Format: 3 });
    const gateFaucet = gateKeyring.addFromUri(mnemonicGenerate(12));

    beforeAll(async () => {
        gateToken = await deployGATEToken(deployer, undefined);

        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
            sudoSigner = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
        }

        const { api } = ccApi;

        await api.tx.sudo
            .sudo(api.tx.balances.setBalance(gateFaucet.address, 1000, 0))
            .signAndSend(sudoSigner, { nonce: -1 });

        // Set the on chain location for the burn contract to be the address of the deployer wallet
        const contract = api.createType('PalletCreditcoinOcwTasksCollectCoinsDeployedContract', {
            address: gateToken.address,
            chain: testingData.blockchain,
        });
        await api.tx.sudo.sudo(api.tx.creditcoin.setGateContract(contract)).signAndSend(sudoSigner, { nonce: -1 });
    }, 900_000);

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    testIf(
        (global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY,
        '000 - End to end',
        async () => {
            const {
                api,
                extrinsics: { requestCollectCoinsV2 },
            } = ccApi;

            const mintTx = await gateToken.mint(deployer.address, 2500);
            await mintTx.wait(3);
            const balance = await gateToken.balanceOf(deployer.address);
            expect(balance.eq(2500)).toBe(true);

            const burnTx = await gateToken.burn(burnAmount);
            await burnTx.wait(3);

            // We are using the same deployer address as GCRE so the address may already be registered
            await tryRegisterAddress(
                ccApi,
                deployer.address,
                testingData.blockchain,
                signAccountId(api, deployer, sudoSigner.address),
                sudoSigner,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            );
            const gateContract = GATEContract(deployer.address, burnTx.hash);

            // Test #1: The extrinsic should erorr when the faucet address has not been set
            await expect(requestCollectCoinsV2(gateContract, sudoSigner)).rejects.toThrow(
                'creditcoin.BurnGATEFaucetNotSet',
            );

            await api.tx.sudo
                .sudo(api.tx.creditcoin.setGateFaucet(gateFaucet.address))
                .signAndSend(sudoSigner, { nonce: -1 });

            const swapGATE = await requestCollectCoinsV2(gateContract, sudoSigner);
            const swapGATEVerified = await swapGATE.waitForVerification(900_000);

            // Test #2: This is a successful transfer and should proceed normally
            expect(swapGATEVerified).toBeTruthy();

            // Test #3: GATE -> CTC should be swapped in a 2:1 ratio
            expect(swapGATEVerified.amount.toNumber()).toEqual(burnAmount / 2);

            // Test #4: You cannot resubmit previously used burn transactions
            await expect(requestCollectCoinsV2(gateContract, sudoSigner)).rejects.toThrow(
                'creditcoin.CollectCoinsAlreadyRegistered: The coin collection has already been registered',
            );
        },
        900_000,
    );

    // This test must run after the end to end test
    // We are relying on the gate contract already being set and the faucet account being set prior to this test running
    // This is an acceptable assumption since we run tests with --runInBand
    testIf(
        (global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY,
        '001 - collectCoinsV2Example',
        async () => {
            const { api } = ccApi;

            const mintTx = await gateToken.mint(deployer.address, 2500);
            await mintTx.wait(3);

            const burnTx = await gateToken.burn(burnAmount);
            await burnTx.wait(3);

            // We are using the same deployer address as GCRE so the address may already be registered
            await tryRegisterAddress(
                ccApi,
                deployer.address,
                testingData.blockchain,
                signAccountId(api, deployer, sudoSigner.address),
                sudoSigner,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            );

            const gateContract = GATEContract(deployer.address, burnTx.hash);
            const swapGATEVerified = await collectCoinsV2Example(ccApi, gateContract, sudoSigner);

            expect(swapGATEVerified).toBeTruthy();
            expect(swapGATEVerified.amount.toNumber()).toEqual(burnAmount / 2);
        },
        900_000,
    );
});
