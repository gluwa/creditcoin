import { KeyringPair } from '@polkadot/keyring/types';

import { Guid } from 'js-guid';
import { POINT_01_CTC } from '../constants';

import { signLoanParams, DealOrderRegistered } from 'creditcoin-js/extrinsics/register-deal-order';
import { creditcoinApi } from 'creditcoin-js';
import { CreditcoinApi } from 'creditcoin-js/types';
import { testData, lendOnEth } from './common';
import { extractFee } from '../utils';
import { Wallet } from 'ethers';

describe('LockDealOrder', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let dealOrder: DealOrderRegistered;
    let lenderWallet: Wallet;
    let borrowerWallet: Wallet;

    const { blockchain, expirationBlock, loanTerms, createWallet, keyring } = testData;

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = keyring.addFromUri('//Alice');
        borrower = keyring.addFromUri('//Bob');
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        const {
            api,
            extrinsics: { fundDealOrder, registerAddress, registerDealOrder, registerFundingTransfer },
            utils: { signAccountId },
        } = ccApi;
        lenderWallet = createWallet();
        borrowerWallet = createWallet();
        const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
            registerAddress(lenderWallet.address, blockchain, signAccountId(lenderWallet, lender.address), lender),
            registerAddress(
                borrowerWallet.address,
                blockchain,
                signAccountId(borrowerWallet, borrower.address),
                borrower,
            ),
        ]);
        const askGuid = Guid.newGuid();
        const bidGuid = Guid.newGuid();
        const signedParams = signLoanParams(api, borrower, expirationBlock, askGuid, bidGuid, loanTerms);

        dealOrder = await registerDealOrder(
            lenderRegAddr.itemId,
            borrowerRegAddr.itemId,
            loanTerms,
            expirationBlock,
            askGuid,
            bidGuid,
            borrower.publicKey,
            signedParams,
            lender,
        );

        const [fundingTokenAddress, fundingTxHash] = await lendOnEth(
            lenderWallet,
            borrowerWallet,
            dealOrder.dealOrder.itemId,
            loanTerms,
        );
        const fundingEvent = await registerFundingTransfer(
            { kind: 'Ethless', contractAddress: fundingTokenAddress },
            dealOrder.dealOrder.itemId,
            fundingTxHash,
            lender,
        );
        const fundingTransferVerified = await fundingEvent.waitForVerification().catch();
        expect(fundingTransferVerified).toBeTruthy();

        await fundDealOrder(dealOrder.dealOrder.itemId, fundingEvent.transferId, lender);
    }, 600000);

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .lockDealOrder(dealOrder.dealOrder.itemId)
                .signAndSend(borrower, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    }, 60000);
});
