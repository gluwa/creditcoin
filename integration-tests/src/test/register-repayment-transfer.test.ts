import { creditcoinApi, KeyringPair, LoanTerms, TransferKind, Guid, Wallet, POINT_01_CTC } from 'creditcoin-js';

import { signLoanParams, DealOrderRegistered } from 'creditcoin-js/lib/extrinsics/register-deal-order';
import { Blockchain } from 'creditcoin-js/lib/model';
import { CreditcoinApi } from 'creditcoin-js/lib/types';
import { createCreditcoinTransferKind } from 'creditcoin-js/lib/transforms';
import { testData, lendOnEth, tryRegisterAddress, loanTermsWithCurrency } from 'creditcoin-js/lib/testUtils';
import { ethConnection, testCurrency } from 'creditcoin-js/lib/examples/ethereum';

import { extractFee } from '../utils';

const ethless: TransferKind = {
    platform: 'Evm',
    kind: 'Ethless',
};

describe('RegisterRepaymentTransfer', (): void => {
    let ccApi: CreditcoinApi;
    let borrower: KeyringPair;
    let lender: KeyringPair;
    let dealOrder: DealOrderRegistered;
    let repaymentTxHash: string;
    let lenderWallet: Wallet;
    let borrowerWallet: Wallet;
    let loanTerms: LoanTerms;

    const { blockchain, expirationBlock, createWallet, keyring } = testData(
        (global as any).CREDITCOIN_ETHEREUM_CHAIN as Blockchain,
        (global as any).CREDITCOIN_CREATE_WALLET,
    );

    beforeAll(async () => {
        ccApi = await creditcoinApi((global as any).CREDITCOIN_API_URL);
        lender = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'lender');
        borrower = (global as any).CREDITCOIN_CREATE_SIGNER(keyring, 'borrower');
    });

    afterAll(async () => {
        await ccApi.api.disconnect();
    });

    beforeEach(async () => {
        const {
            api,
            extrinsics: { fundDealOrder, lockDealOrder, registerDealOrder, registerFundingTransfer },
            utils: { signAccountId },
        } = ccApi;
        lenderWallet = createWallet('lender');
        borrowerWallet = createWallet('borrower');
        const [lenderRegAddr, borrowerRegAddr] = await Promise.all([
            tryRegisterAddress(
                ccApi,
                lenderWallet.address,
                blockchain,
                signAccountId(lenderWallet, lender.address),
                lender,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            ),
            tryRegisterAddress(
                ccApi,
                borrowerWallet.address,
                blockchain,
                signAccountId(borrowerWallet, borrower.address),
                borrower,
                (global as any).CREDITCOIN_REUSE_EXISTING_ADDRESSES,
            ),
        ]);
        const eth = await ethConnection(
            (global as any).CREDITCOIN_ETHEREUM_NODE_URL,
            (global as any).CREDITCOIN_ETHEREUM_DECREASE_MINING_INTERVAL,
            (global as any).CREDITCOIN_ETHEREUM_USE_HARDHAT_WALLET ? undefined : lenderWallet,
        );
        const currency = testCurrency(eth.testTokenAddress);
        loanTerms = await loanTermsWithCurrency(ccApi, currency);
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

        const fundingTxHash = await lendOnEth(lenderWallet, borrowerWallet, dealOrder.dealOrder.itemId, loanTerms, eth);
        const fundingEvent = await registerFundingTransfer(ethless, dealOrder.dealOrder.itemId, fundingTxHash, lender);
        const fundingTransferVerified = await fundingEvent.waitForVerification().catch();
        expect(fundingTransferVerified).toBeTruthy();

        await fundDealOrder(dealOrder.dealOrder.itemId, fundingEvent.transferId, lender);
        await lockDealOrder(dealOrder.dealOrder.itemId, borrower);
        // borrower repays the money on Ethereum
        repaymentTxHash = await lendOnEth(borrowerWallet, lenderWallet, dealOrder.dealOrder.itemId, loanTerms, eth);
    }, 18000000);

    it('fee is min 0.01 CTC', async (): Promise<void> => {
        const { api } = ccApi;
        const ccTransferKind = createCreditcoinTransferKind(api, ethless);

        return new Promise((resolve, reject): void => {
            const unsubscribe = api.tx.creditcoin
                .registerRepaymentTransfer(
                    ccTransferKind,
                    loanTerms.amount,
                    dealOrder.dealOrder.itemId,
                    repaymentTxHash,
                )
                .signAndSend(borrower, { nonce: -1 }, async ({ dispatchError, events, status }) => {
                    await extractFee(resolve, reject, unsubscribe, api, dispatchError, events, status);
                })
                .catch((error) => reject(error));
        }).then((fee) => {
            expect(fee).toBeGreaterThanOrEqual(POINT_01_CTC);
        });
    }, 18000000);
});
