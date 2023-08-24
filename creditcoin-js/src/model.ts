import { BN } from '@polkadot/util';

export type AddressId = string;

export type AccountId = string;

export type ExternalAddress = string;

export type Blockchain = 'Ethereum' | 'Rinkeby' | 'Luniverse' | 'Bitcoin' | 'Other';

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
};

export type TupleId = [number, string];
export type AskOrderId = TupleId;
export type BidOrderId = TupleId;

type AskOrBidOrderBase = {
    loanTerms: LoanTerms;
    expirationBlock: number;
    blockNumber: number;
    blockchain: Blockchain;
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
    blockchain: Blockchain;
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

export type Erc20 = { kind: 'Erc20'; contractAddress: ExternalAddress };
export type Ethless = { kind: 'Ethless'; contractAddress: ExternalAddress };
export type Other = { kind: 'Other'; value: ExternalAddress };
export type Native = { kind: 'Native' };
export type TransferKind = Erc20 | Ethless | Native | Other;

export type TransferId = string;

export type Transfer = {
    blockchain: Blockchain;
    kind: TransferKind;
    from: AddressId;
    to: AddressId;
    orderId: DealOrderId;
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

export type SignatureType = 'PersonalSign' | 'EthSign';

export type Signature = string;
export type PersonalSign = { kind: 'PersonalSign'; signature: Signature };
export type EthSign = { kind: 'EthSign'; signature: Signature };
export type OwnershipProof = PersonalSign | EthSign;
