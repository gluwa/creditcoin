import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { BN } from '@polkadot/util';
import { Guid } from 'js-guid';
import {
    AccountId,
    AddressId,
    AskOrderId,
    BidOrderId,
    Blockchain,
    DealOrderAdded,
    DealOrderClosed,
    DealOrderFunded,
    DealOrderId,
    DealOrderLocked,
    LoanTerms,
    OfferId,
    TransferId,
    TransferKind,
    TransferProcessed,
} from './model';
import { AddressRegistered } from './extrinsics/register-address';
import { AskOrderAdded } from './extrinsics/add-ask-order';
import { BidOrderAdded } from './extrinsics/add-bid-order';
import { OfferAdded } from './extrinsics/add-offer';
import { DealOrderRegistered } from './extrinsics/register-deal-order';
import { TransferEvent } from './extrinsics/register-transfers';
import { LoanExempted } from './extrinsics/exempt';
import { Wallet } from 'ethers';
import { CollectCoinsEvent } from './extrinsics/request-collect-coins';

export type TxCallback = (result: SubmittableResult) => void;
export type TxFailureCallback = (error?: Error) => void;
export type ExtrinsicFailed = string;

export interface Extrinsics {
    registerAddress: (
        externalAddress: string,
        blockchain: Blockchain,
        ownershipProof: string,
        signer: KeyringPair,
    ) => Promise<AddressRegistered>;
    addAskOrder: (
        lenderAddressId: AddressId,
        loanTerms: LoanTerms,
        expirationBlock: number,
        guid: Guid,
        signer: KeyringPair,
    ) => Promise<AskOrderAdded>;
    addBidOrder: (
        borrowerAddressId: AddressId,
        loanTerms: LoanTerms,
        expirationBlock: number,
        guid: Guid,
        signer: KeyringPair,
    ) => Promise<BidOrderAdded>;
    addOffer: (
        askOrderId: AskOrderId,
        bidOrderId: BidOrderId,
        expirationBlock: number,
        signer: KeyringPair,
    ) => Promise<OfferAdded>;
    addDealOrder: (offerId: OfferId, expirationBlock: number, signer: KeyringPair) => Promise<DealOrderAdded>;
    registerDealOrder: (
        lenderAddressId: AddressId,
        borrowerAddressId: AddressId,
        loanTerms: LoanTerms,
        expBlock: number,
        askGuid: Guid,
        bidGuid: Guid,
        borrowerKey: Uint8Array,
        signedParams: Uint8Array,
        lender: KeyringPair,
    ) => Promise<DealOrderRegistered>;
    registerFundingTransfer: (
        transferKind: TransferKind,
        dealOrderId: DealOrderId,
        txHash: string,
        lender: KeyringPair,
    ) => Promise<TransferEvent>;
    fundDealOrder: (
        dealOrderId: DealOrderId,
        transferId: TransferId,
        lender: KeyringPair,
    ) => Promise<[DealOrderFunded, TransferProcessed]>;
    lockDealOrder: (dealOrderId: DealOrderId, borrower: KeyringPair) => Promise<DealOrderLocked>;
    registerRepaymentTransfer: (
        transferKind: TransferKind,
        repaymentAmount: BN,
        dealOrderId: DealOrderId,
        txHash: string,
        borrower: KeyringPair,
    ) => Promise<TransferEvent>;
    closeDealOrder: (
        dealOrderId: DealOrderId,
        transferId: TransferId,
        borrower: KeyringPair,
    ) => Promise<[DealOrderClosed, TransferProcessed]>;
    exemptLoan: (dealOrderId: DealOrderId, lender: KeyringPair) => Promise<LoanExempted>;
    requestCollectCoins: (evmAddress: string, collector: KeyringPair, txHash: string) => Promise<CollectCoinsEvent>;
}

export interface CreditcoinApi {
    api: ApiPromise;
    extrinsics: Extrinsics;
    utils: { signAccountId: (signer: Wallet, accountId: AccountId) => string };
}
