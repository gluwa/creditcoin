import { ApiPromise } from '@polkadot/api';
import { BN } from '@polkadot/util';
import {
    PalletCreditcoinAddress,
    PalletCreditcoinAskOrder,
    PalletCreditcoinBidOrder,
    PalletCreditcoinDealOrder,
    PalletCreditcoinLoanTerms,
    PalletCreditcoinLoanTermsInterestRate,
    PalletCreditcoinLoanTermsDuration,
    PalletCreditcoinOffer,
    PalletCreditcoinTransfer,
    PalletCreditcoinTransferKind,
    PalletCreditcoinCollectCoinsCollectedCoins,
    PalletCreditcoinCollectCoinsUnverifiedCollectedCoins,
    PalletCreditcoinCollectCoinsBurnGATE,
    PalletCreditcoinCollectCoinsUnverifiedBurnGATE,
} from '@polkadot/types/lookup';
import {
    Address,
    AskOrder,
    LoanTerms,
    BidOrder,
    Offer,
    AskOrderId,
    BidOrderId,
    DealOrder,
    OfferId,
    InterestRate,
    Duration,
    DealOrderId,
    Transfer,
    TransferKind,
    UnverifiedCollectedCoins,
    CollectedCoins,
    UnverifiedSwapGATE,
    SwappedGATE,
} from './model';

export const createAddress = ({ value, blockchain, owner }: PalletCreditcoinAddress): Address => ({
    accountId: owner.toString(),
    blockchain: blockchain.type,
    externalAddress: value.toString(),
});

export const createDuration = ({ secs, nanos }: PalletCreditcoinLoanTermsDuration): Duration => ({
    secs: secs.toNumber(),
    nanos: nanos.toNumber(),
});

export const createInterestRate = ({
    ratePerPeriod,
    decimals,
    period,
    interestType,
}: PalletCreditcoinLoanTermsInterestRate): InterestRate => ({
    ratePerPeriod: ratePerPeriod.toNumber(),
    decimals: decimals.toNumber(),
    period: createDuration(period),
    interestType: interestType.type,
});

export const createLoanTerms = ({ amount, interestRate, termLength }: PalletCreditcoinLoanTerms): LoanTerms => ({
    amount,
    interestRate: createInterestRate(interestRate),
    termLength: createDuration(termLength),
});

export const createCreditcoinLoanTerms = (
    api: ApiPromise,
    { amount, interestRate, termLength }: LoanTerms,
): PalletCreditcoinLoanTerms =>
    api.createType('PalletCreditcoinLoanTerms', {
        amount,
        interestRate,
        termLength,
    });

export const createAskOrder = ({
    blockchain,
    terms,
    lenderAddressId,
    expirationBlock,
    block,
    lender,
}: PalletCreditcoinAskOrder): AskOrder => ({
    blockchain: blockchain.type,
    blockNumber: block.toNumber(),
    expirationBlock: expirationBlock.toNumber(),
    loanTerms: createLoanTerms(terms),
    lenderAddressId: lenderAddressId.toString(),
    lenderAccountId: lender.toString(),
});

export const createBidOrder = ({
    blockchain,
    terms,
    borrowerAddressId,
    expirationBlock,
    block,
    borrower,
}: PalletCreditcoinBidOrder): BidOrder => ({
    blockchain: blockchain.type,
    blockNumber: block.toNumber(),
    expirationBlock: expirationBlock.toNumber(),
    loanTerms: createLoanTerms(terms),
    borrowerAddressId: borrowerAddressId.toString(),
    borrowerAccountId: borrower.toString(),
});

export const createOffer = ({
    blockchain,
    askId,
    bidId,
    expirationBlock,
    block,
    lender,
}: PalletCreditcoinOffer): Offer => ({
    blockchain: blockchain.type,
    askOrderId: askId.toJSON() as AskOrderId,
    bidOrderId: bidId.toJSON() as BidOrderId,
    expirationBlock: expirationBlock.toNumber(),
    blockNumber: block.toNumber(),
    lenderAccountId: lender.toString(),
});

export const createDealOrder = (dealOrder: PalletCreditcoinDealOrder): DealOrder => {
    const {
        offerId,
        lenderAddressId,
        borrowerAddressId,
        terms,
        expirationBlock,
        timestamp,
        fundingTransferId,
        repaymentTransferId,
        lock,
        borrower,
        block,
    } = dealOrder;
    return {
        offerId: offerId.toJSON() as OfferId,
        lenderAddressId: lenderAddressId.toString(),
        borrowerAddressId: borrowerAddressId.toString(),
        loanTerms: createLoanTerms(terms),
        expirationBlock: expirationBlock.toNumber(),
        timestamp: new Date(timestamp.toNumber()),
        fundingTransferId: fundingTransferId.unwrapOr(undefined)?.toString(),
        repaymentTransferId: repaymentTransferId.unwrapOr(undefined)?.toString(),
        lock: lock.unwrapOr(undefined)?.toString(),
        borrower: borrower.toString(),
        block: block.unwrapOr(undefined)?.toNumber(),
    };
};

export const createCreditcoinTransferKind = (
    api: ApiPromise,
    transferKind: TransferKind,
): PalletCreditcoinTransferKind => {
    const toType = (): unknown => {
        switch (transferKind.kind) {
            case 'Erc20':
                return { Erc20: transferKind.contractAddress }; // eslint-disable-line  @typescript-eslint/naming-convention
            case 'Ethless':
                return { Ethless: transferKind.contractAddress }; // eslint-disable-line  @typescript-eslint/naming-convention
            case 'Native':
                return 'Native';
            case 'Other':
                return { Other: transferKind.value }; // eslint-disable-line  @typescript-eslint/naming-convention
        }
    };

    return api.createType('PalletCreditcoinTransferKind', toType());
};

export const createTransferKind = (transferKind: PalletCreditcoinTransferKind): TransferKind => {
    switch (transferKind.type) {
        case 'Erc20':
            return { kind: 'Erc20', contractAddress: transferKind.asErc20.toString() };
        case 'Ethless':
            return { kind: 'Ethless', contractAddress: transferKind.asEthless.toString() };
        case 'Native':
            return { kind: 'Native' };
        default:
            return { kind: 'Other', value: transferKind.asOther.toString() };
    }
};

export const createTransfer = (transfer: PalletCreditcoinTransfer): Transfer => {
    const { blockchain, kind, from, to, orderId, amount, txId, block, isProcessed, accountId, timestamp } = transfer;
    return {
        blockchain: blockchain.type,
        kind: createTransferKind(kind),
        from: from.toString(),
        to: to.toString(),
        orderId: (orderId.isDeal ? orderId.asDeal.toJSON() : orderId.asRepayment.toJSON()) as DealOrderId,
        amount,
        txHash: txId.toString(),
        blockNumber: block.toNumber(),
        processed: isProcessed.isTrue,
        accountId: accountId.toString(),
        timestamp: timestamp.isSome ? new Date(timestamp.unwrap().toNumber()) : undefined,
    };
};

export const createUnverifiedCollectedCoins = (
    collectedCoins: PalletCreditcoinCollectCoinsUnverifiedCollectedCoins,
): UnverifiedCollectedCoins => {
    const { to, txId } = collectedCoins;
    return {
        to: to.toString(),
        txHash: txId.toString(),
    };
};

export const createCollectedCoins = (collectedCoins: PalletCreditcoinCollectCoinsCollectedCoins): CollectedCoins => {
    const { to, txId, amount } = collectedCoins;
    return {
        to: to.toString(),
        txHash: txId.toString(),
        amount: amount as BN,
    };
};

export const createUnverifiedBurnGATE = (
    burnedGATE: PalletCreditcoinCollectCoinsUnverifiedBurnGATE,
): UnverifiedSwapGATE => {
    const { to, txId } = burnedGATE;
    return {
        to: to.toString(),
        txHash: txId.toString(),
    };
};

export const createBurnedGATE = (burnedGate: PalletCreditcoinCollectCoinsBurnGATE): SwappedGATE => {
    const { to, txId, amount } = burnedGate;
    return {
        to: to.toString(),
        txHash: txId.toString(),
        amount: amount as BN,
    };
};
