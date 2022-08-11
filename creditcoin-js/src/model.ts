import { BN } from '@polkadot/util';

export type AddressId = string;

export type AccountId = string;

export type ExternalAddress = string;

export const CHAINS: Record<string, Blockchain> = {
    ethereum: {
        platform: 'Evm',
        chainId: new BN(1),
    },
    rinkeby: {
        platform: 'Evm',
        chainId: new BN(4),
    },
    luniverse: {
        platform: 'Evm',
        chainId: new BN(59496427),
    },
    luniverseTestnet: {
        platform: 'Evm',
        chainId: new BN(949790),
    },
    hardhat: {
        platform: 'Evm',
        chainId: new BN(31337),
    },
};

// Will eventually be something like Platform<Evm, Other> = ({ platform: 'Evm' } & Evm) | ({ platform: 'Other' } & Other);
type Platform<Evm> = { platform: 'Evm' } & Evm;

export type EvmChainId = BN;

export type EvmInfo = {
    chainId: EvmChainId;
};

export type Blockchain = Platform<EvmInfo>;

export type EvmTransferKind = 'Ethless' | 'Erc20';

export type TransferKind = Platform<{ kind: EvmTransferKind }>;

export type EvmSmartContractCurrency = {
    type: 'SmartContract';
    contract: ExternalAddress;
    supportedTransferKinds: Set<EvmTransferKind>;
};

// Eventually will be EvmSmartContractCurrency | EvmOtherTypeOfCurrency
export type EvmCurrency = EvmSmartContractCurrency;

export type Currency = EvmCurrency & Blockchain;
export type CurrencyId = string;

export type Address = {
    accountId: AccountId;
    blockchain: Blockchain;
    externalAddress: ExternalAddress;
};

export type Duration = {
    secs: number;
    nanos: number;
};

export type InterestType = 'Simple' | 'Compound';

export type InterestRate = {
    ratePerPeriod: number;
    decimals: number;
    period: Duration;
    interestType: InterestType;
};

export type LoanTerms = {
    amount: BN;
    interestRate: InterestRate;
    termLength: Duration;
    currency: CurrencyId;
};

export type TupleId = [number, string];
export type AskOrderId = TupleId;
export type BidOrderId = TupleId;

type AskOrBidOrderBase = {
    loanTerms: LoanTerms;
    expirationBlock: number;
    blockNumber: number;
};

export type AskOrder = AskOrBidOrderBase & {
    lenderAddressId: AddressId;
    lenderAccountId: AccountId;
};

export type BidOrder = AskOrBidOrderBase & {
    borrowerAddressId: AddressId;
    borrowerAccountId: AccountId;
};

export type OfferId = TupleId;

export type Offer = {
    askOrderId: AskOrderId;
    bidOrderId: BidOrderId;
    expirationBlock: number;
    blockNumber: number;
    lenderAccountId: AccountId;
};

export type DealOrderId = TupleId;

export type DealOrder = {
    offerId: OfferId;
    lenderAddressId: AddressId;
    borrowerAddressId: AddressId;
    loanTerms: LoanTerms;
    expirationBlock: number;
    timestamp: Date;
    fundingTransferId?: string;
    repaymentTransferId?: string;
    lock?: string;
    borrower: AccountId;
    block?: number;
};

type EventReturnIdType<Id> = {
    itemId: Id;
};

type EventReturnDataType<Data> = {
    item: Data;
};

export type EventReturnJoinType<Id, Data> = EventReturnIdType<Id> & EventReturnDataType<Data>;
export type EventReturnType<Id, Data> = EventReturnIdType<Id> | EventReturnJoinType<Id, Data>;

export type DealOrderAdded = EventReturnJoinType<DealOrderId, DealOrder>;
export type DealOrderFunded = EventReturnIdType<DealOrderId>;
export type DealOrderLocked = EventReturnIdType<DealOrderId>;
export type DealOrderClosed = EventReturnIdType<DealOrderId>;

export type TransferId = string;

export type Transfer = {
    blockchain: Blockchain;
    kind: TransferKind;
    from: AddressId;
    to: AddressId;
    dealOrderId: DealOrderId;
    amount: BN;
    txHash: string;
    blockNumber: number;
    processed: boolean;
    accountId: AccountId;
    timestamp?: Date;
};

export type TransferProcessed = EventReturnJoinType<TransferId, Transfer>;

export type CollectedCoinsId = string;

export type CollectedCoins = {
    to: ExternalAddress;
    txHash: string;
    amount: BN;
};

export type UnverifiedCollectedCoins = {
    to: ExternalAddress;
    txHash: string;
};
