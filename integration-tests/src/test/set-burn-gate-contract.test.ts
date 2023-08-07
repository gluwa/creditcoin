import { deployGATEToken } from '../ctc-deploy';
import { JsonRpcProvider } from '@ethersproject/providers';
import { Wallet } from 'ethers';
import { KeyringPair, creditcoinApi } from 'creditcoin-js';
import { Blockchain } from 'creditcoin-js/lib/model';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { testData } from 'creditcoin-js/lib/testUtils';

import { extractFee, testIf } from '../utils';

describe('Test set GATE contract', (): void => {
    const provider = new JsonRpcProvider((global as any).CREDITCOIN_ETHEREUM_NODE_URL);
    const deployer = new Wallet((global as any).CREDITCOIN_CTC_DEPLOYER_PRIVATE_KEY, provider);
    let gateToken: any;
    let ccApi: CreditcoinApi;
    let sudoSigner: KeyringPair;

    const testingData = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );
    const { keyring } = testingData;

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeAll(async () => {
        gateToken = await deployGATEToken(deployer);

        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        if ((global as any).CREDITCOIN_EXECUTE_SETUP_AUTHORITY) {
            sudoSigner = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
        }
    });

    test('Burn and mint gate in the deployer wallet', async () => {
        const mintTx = await gateToken.mint(deployer.address, 1000);
        await mintTx.wait(2);
        const balance = await gateToken.balanceOf(deployer.address);
        expect(balance.eq(1000)).toBe(true);

        const burnTx = await gateToken.burn(200);
        await burnTx.wait(2);
        const finalBalance = await gateToken.balanceOf(deployer.address);
        expect(finalBalance.eq(800)).toBe(true);
    });
});
