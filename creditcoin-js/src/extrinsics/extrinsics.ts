import { ApiPromise } from '@polkadot/api';
import { BN } from '@polkadot/util';
import { Guid } from 'js-guid';
import { addAskOrderAsync } from './add-ask-order';
import { addBidOrderAsync } from './add-bid-order';
import { addDealOrderAsync } from './add-deal-order';
import { addOfferAsync } from './add-offer';
import { fundDealOrderAsync } from './fund-deal-order';
import { registerAddressAsync } from './register-address';
import { registerDealOrderAsync } from './register-deal-order';
import { registerFundingTransferAsync, registerRepaymentTransferAsync } from './register-transfers';
import {
    Blockchain,
    AddressId,
    LoanTerms,
    AskOrderId,
    BidOrderId,
    OfferId,
    TransferKind,
    DealOrderId,
    TransferId,
    ExternalAddress,
    OwnershipProof,
} from '../model';
import { KeyringPair } from '@polkadot/keyring/types';
import { lockDealOrderAsync } from './lock-deal-order';
import { closeDealOrderAsync } from './close-deal-order';
import { exemptLoanAsync } from './exempt';
import { requestCollectCoinsAsync } from './request-collect-coins';
import { registerAddressV2Async } from './register-address-v2';
import { requestSwapGATEAsync } from './request-swap-gate';

export const extrinsics = (api: ApiPromise) => {
    const registerAddress = (
        externalAddress: string,
        blockchain: Blockchain,
        ownershipProof: string,
        signer: KeyringPair,
    ) => registerAddressAsync(api, externalAddress, blockchain, ownershipProof, signer);

    const addAskOrder = (
        lenderAddressId: AddressId,
        loanTerms: LoanTerms,
        expirationBlock: number,
        guid: Guid,
        signer: KeyringPair,
    ) => addAskOrderAsync(api, lenderAddressId, loanTerms, expirationBlock, guid, signer);

    const addBidOrder = (
        borrowerAddressId: AddressId,
        loanTerms: LoanTerms,
        expirationBlock: number,
        guid: Guid,
        signer: KeyringPair,
    ) => addBidOrderAsync(api, borrowerAddressId, loanTerms, expirationBlock, guid, signer);

    const addOffer = (askOrderId: AskOrderId, bidOrderId: BidOrderId, expirationBlock: number, signer: KeyringPair) =>
        addOfferAsync(api, askOrderId, bidOrderId, expirationBlock, signer);

    const addDealOrder = (offerId: OfferId, expirationBlock: number, signer: KeyringPair) =>
        addDealOrderAsync(api, offerId, expirationBlock, signer);

    const registerDealOrder = (
        lenderAddressId: AddressId,
        borrowerAddressId: AddressId,
        loanTerms: LoanTerms,
        expBlock: number,
        askGuid: Guid,
        bidGuid: Guid,
        borrowerKey: Uint8Array,
        signedParams: Uint8Array,
        lender: KeyringPair,
    ) =>
        registerDealOrderAsync(
            api,
            lenderAddressId,
            borrowerAddressId,
            loanTerms,
            expBlock,
            askGuid,
            bidGuid,
            borrowerKey,
            signedParams,
            lender,
        );

    const registerFundingTransfer = (
        transferKind: TransferKind,
        dealOrderId: DealOrderId,
        txHash: string,
        lender: KeyringPair,
    ) => registerFundingTransferAsync(api, transferKind, dealOrderId, txHash, lender);

    const fundDealOrder = (dealOrderId: DealOrderId, transferId: TransferId, lender: KeyringPair) =>
        fundDealOrderAsync(api, dealOrderId, transferId, lender);

    const lockDealOrder = (dealOrderId: DealOrderId, borrower: KeyringPair) =>
        lockDealOrderAsync(api, dealOrderId, borrower);

    const registerRepaymentTransfer = (
        transferKind: TransferKind,
        repaymentAmount: BN,
        dealOrderId: DealOrderId,
        txHash: string,
        borrower: KeyringPair,
    ) => registerRepaymentTransferAsync(api, transferKind, repaymentAmount, dealOrderId, txHash, borrower);

    const closeDealOrder = (dealOrderId: DealOrderId, transferId: TransferId, borrower: KeyringPair) =>
        closeDealOrderAsync(api, dealOrderId, transferId, borrower);

    const exemptLoan = (dealOrderId: DealOrderId, lender: KeyringPair) => exemptLoanAsync(api, dealOrderId, lender);

    const requestCollectCoins = (evmAddress: ExternalAddress, collector: KeyringPair, txHash: string) =>
        requestCollectCoinsAsync(api, evmAddress, collector, txHash);

    const registerAddressV2 = (
        externalAddress: string,
        blockchain: Blockchain,
        ownershipProof: OwnershipProof,
        signer: KeyringPair,
    ) => registerAddressV2Async(api, externalAddress, blockchain, ownershipProof, signer);

    const requestSwapGATE = (evmAddress: ExternalAddress, collector: KeyringPair, txHash: string) =>
        requestSwapGATEAsync(api, evmAddress, collector, txHash);

    return {
        registerAddress,
        registerAddressV2,
        addAskOrder,
        addBidOrder,
        addOffer,
        addDealOrder,
        registerDealOrder,
        registerFundingTransfer,
        fundDealOrder,
        lockDealOrder,
        registerRepaymentTransfer,
        closeDealOrder,
        exemptLoan,
        requestCollectCoins,
        requestSwapGATE,
    };
};
