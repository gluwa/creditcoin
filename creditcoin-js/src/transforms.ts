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
    PalletCreditcoinPlatformBlockchain,
    PalletCreditcoinPlatformTransferKind,
    PalletCreditcoinCollectCoinsCollectedCoins,
    PalletCreditcoinCollectCoinsUnverifiedCollectedCoins,
    PalletCreditcoinPlatformEvmInfo,
    PalletCreditcoinPlatformCurrency,
    PalletCreditcoinPlatformEvmTransferKind,
} from '@polkadot/types/lookup';
import {
    Address,
    AskOrder,
    LoanTerms,
    BidOrder,
    Offer,
    EvmInfo,
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
    Blockchain,
    Currency,
    EvmTransferKind,
} from './model';

export const createEvmInfo = (info: PalletCreditcoinPlatformEvmInfo): EvmInfo => {
    const chainId = info.chainId.toString();
    return {
        chainId: new BN(chainId),
    };
};

export const createBlockchain = (blockchain: PalletCreditcoinPlatformBlockchain): Blockchain => {
    switch (blockchain.type) {
        case 'Evm':
            const evmInfo = createEvmInfo(blockchain.asEvm);
            return {
                platform: 'Evm',
                ...evmInfo,
            };
    }
};

export const createCreditcoinBlockchain = (
    api: ApiPromise,
    blockchain: Blockchain,
): PalletCreditcoinPlatformBlockchain => {
    const toType = (): unknown => {
        switch (blockchain.platform) {
            case 'Evm':
                return { Evm: { chainId: blockchain.chainId } }; // eslint-disable-line @typescript-eslint/naming-convention
        }
    };
    return api.createType('PalletCreditcoinPlatformBlockchain', toType());
};

/* eslint-disable @typescript-eslint/naming-convention */

export const createEvmTransferKind = (evmTransferKind: PalletCreditcoinPlatformEvmTransferKind): EvmTransferKind => {
    return evmTransferKind.type;
};

export const createCurrency = (currency: PalletCreditcoinPlatformCurrency): Currency => {
    switch (currency.type) {
        case 'Evm':
            const [ctcCurrencyType, ctcEvmInfo] = currency.asEvm;
            const evmInfo = createEvmInfo(ctcEvmInfo);
            switch (ctcCurrencyType.type) {
                case 'SmartContract':
                    const [contractAddr, supported] = ctcCurrencyType.asSmartContract;
                    const supportedTransferKinds = new Set(supported.map(createEvmTransferKind));
                    const contract = contractAddr.toString();
                    return {
                        platform: 'Evm',
                        type: 'SmartContract',
                        contract,
                        supportedTransferKinds,
                        ...evmInfo,
                    };
            }
    }
};

export const createCreditcoinCurrency = (api: ApiPromise, currency: Currency): PalletCreditcoinPlatformCurrency => {
    const toType = (): unknown => {
        switch (currency.platform) {
            case 'Evm':
                switch (currency.type) {
                    case 'SmartContract':
                        return {
                            Evm: [
                                { SmartContract: [currency.contract, Array.from(currency.supportedTransferKinds)] },
                                {
                                    chainId: currency.chainId,
                                },
                            ],
                        };
                }
        }
    };
    return api.createType('PalletCreditcoinPlatformCurrency', toType());
};

/* eslint-enable @typescript-eslint/naming-convention */

export const createAddress = ({ value, blockchain, owner }: PalletCreditcoinAddress): Address => ({
    accountId: owner.toString(),
    blockchain: createBlockchain(blockchain),
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

export const createLoanTerms = ({
    amount,
    interestRate,
    termLength,
    currency,
}: PalletCreditcoinLoanTerms): LoanTerms => ({
    amount,
    interestRate: createInterestRate(interestRate),
    termLength: createDuration(termLength),
    currency: currency.toString(),
});

export const createCreditcoinLoanTerms = (
    api: ApiPromise,
    { amount, interestRate, termLength, currency }: LoanTerms,
): PalletCreditcoinLoanTerms =>
    api.createType('PalletCreditcoinLoanTerms', {
        amount,
        interestRate,
        termLength,
        currency,
    });

export const createAskOrder = ({
    terms,
    lenderAddressId,
    expirationBlock,
    block,
    lender,
}: PalletCreditcoinAskOrder): AskOrder => ({
    blockNumber: block.toNumber(),
    expirationBlock: expirationBlock.toNumber(),
    loanTerms: createLoanTerms(terms),
    lenderAddressId: lenderAddressId.toString(),
    lenderAccountId: lender.toString(),
});

export const createBidOrder = ({
    terms,
    borrowerAddressId,
    expirationBlock,
    block,
    borrower,
}: PalletCreditcoinBidOrder): BidOrder => ({
    blockNumber: block.toNumber(),
    expirationBlock: expirationBlock.toNumber(),
    loanTerms: createLoanTerms(terms),
    borrowerAddressId: borrowerAddressId.toString(),
    borrowerAccountId: borrower.toString(),
});

export const createOffer = ({ askId, bidId, expirationBlock, block, lender }: PalletCreditcoinOffer): Offer => ({
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
): PalletCreditcoinPlatformTransferKind => {
    const toType = (): unknown => {
        switch (transferKind.platform) {
            case 'Evm':
                return { Evm: transferKind.kind }; // eslint-disable-line @typescript-eslint/naming-convention
        }
    };

    return api.createType('PalletCreditcoinPlatformTransferKind', toType());
};

export const createTransferKind = (transferKind: PalletCreditcoinPlatformTransferKind): TransferKind => {
    switch (transferKind.type) {
        case 'Evm':
            return { platform: 'Evm', kind: transferKind.asEvm.type };
    }
};

export const createTransfer = (transfer: PalletCreditcoinTransfer): Transfer => {
    const { blockchain, dealOrderId, kind, from, to, amount, txId, block, isProcessed, accountId, timestamp } =
        transfer;
    return {
        blockchain: createBlockchain(blockchain),
        kind: createTransferKind(kind),
        from: from.toString(),
        to: to.toString(),
        amount,
        dealOrderId: dealOrderId.toJSON() as DealOrderId,
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
