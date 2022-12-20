// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/lookup';

import type {
    Bytes,
    Compact,
    Enum,
    Null,
    Option,
    Result,
    Struct,
    Text,
    U256,
    U8aFixed,
    Vec,
    bool,
    i64,
    u128,
    u32,
    u64,
    u8,
} from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, H160, H256, MultiAddress, Perbill } from '@polkadot/types/interfaces/runtime';
import type { Event } from '@polkadot/types/interfaces/system';

declare module '@polkadot/types/lookup' {
    /** @name FrameSystemAccountInfo (3) */
    interface FrameSystemAccountInfo extends Struct {
        readonly nonce: u32;
        readonly consumers: u32;
        readonly providers: u32;
        readonly sufficients: u32;
        readonly data: PalletBalancesAccountData;
    }

    /** @name PalletBalancesAccountData (5) */
    interface PalletBalancesAccountData extends Struct {
        readonly free: u128;
        readonly reserved: u128;
        readonly miscFrozen: u128;
        readonly feeFrozen: u128;
    }

    /** @name FrameSupportDispatchPerDispatchClassWeight (7) */
    interface FrameSupportDispatchPerDispatchClassWeight extends Struct {
        readonly normal: SpWeightsWeightV2Weight;
        readonly operational: SpWeightsWeightV2Weight;
        readonly mandatory: SpWeightsWeightV2Weight;
    }

    /** @name SpWeightsWeightV2Weight (8) */
    interface SpWeightsWeightV2Weight extends Struct {
        readonly refTime: Compact<u64>;
        readonly proofSize: Compact<u64>;
    }

    /** @name SpRuntimeDigest (13) */
    interface SpRuntimeDigest extends Struct {
        readonly logs: Vec<SpRuntimeDigestDigestItem>;
    }

    /** @name SpRuntimeDigestDigestItem (15) */
    interface SpRuntimeDigestDigestItem extends Enum {
        readonly isOther: boolean;
        readonly asOther: Bytes;
        readonly isConsensus: boolean;
        readonly asConsensus: ITuple<[U8aFixed, Bytes]>;
        readonly isSeal: boolean;
        readonly asSeal: ITuple<[U8aFixed, Bytes]>;
        readonly isPreRuntime: boolean;
        readonly asPreRuntime: ITuple<[U8aFixed, Bytes]>;
        readonly isRuntimeEnvironmentUpdated: boolean;
        readonly type: 'Other' | 'Consensus' | 'Seal' | 'PreRuntime' | 'RuntimeEnvironmentUpdated';
    }

    /** @name FrameSystemEventRecord (18) */
    interface FrameSystemEventRecord extends Struct {
        readonly phase: FrameSystemPhase;
        readonly event: Event;
        readonly topics: Vec<H256>;
    }

    /** @name FrameSystemEvent (20) */
    interface FrameSystemEvent extends Enum {
        readonly isExtrinsicSuccess: boolean;
        readonly asExtrinsicSuccess: {
            readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
        } & Struct;
        readonly isExtrinsicFailed: boolean;
        readonly asExtrinsicFailed: {
            readonly dispatchError: SpRuntimeDispatchError;
            readonly dispatchInfo: FrameSupportDispatchDispatchInfo;
        } & Struct;
        readonly isCodeUpdated: boolean;
        readonly isNewAccount: boolean;
        readonly asNewAccount: {
            readonly account: AccountId32;
        } & Struct;
        readonly isKilledAccount: boolean;
        readonly asKilledAccount: {
            readonly account: AccountId32;
        } & Struct;
        readonly isRemarked: boolean;
        readonly asRemarked: {
            readonly sender: AccountId32;
            readonly hash_: H256;
        } & Struct;
        readonly type:
            | 'ExtrinsicSuccess'
            | 'ExtrinsicFailed'
            | 'CodeUpdated'
            | 'NewAccount'
            | 'KilledAccount'
            | 'Remarked';
    }

    /** @name FrameSupportDispatchDispatchInfo (21) */
    interface FrameSupportDispatchDispatchInfo extends Struct {
        readonly weight: SpWeightsWeightV2Weight;
        readonly class: FrameSupportDispatchDispatchClass;
        readonly paysFee: FrameSupportDispatchPays;
    }

    /** @name FrameSupportDispatchDispatchClass (22) */
    interface FrameSupportDispatchDispatchClass extends Enum {
        readonly isNormal: boolean;
        readonly isOperational: boolean;
        readonly isMandatory: boolean;
        readonly type: 'Normal' | 'Operational' | 'Mandatory';
    }

    /** @name FrameSupportDispatchPays (23) */
    interface FrameSupportDispatchPays extends Enum {
        readonly isYes: boolean;
        readonly isNo: boolean;
        readonly type: 'Yes' | 'No';
    }

    /** @name SpRuntimeDispatchError (24) */
    interface SpRuntimeDispatchError extends Enum {
        readonly isOther: boolean;
        readonly isCannotLookup: boolean;
        readonly isBadOrigin: boolean;
        readonly isModule: boolean;
        readonly asModule: SpRuntimeModuleError;
        readonly isConsumerRemaining: boolean;
        readonly isNoProviders: boolean;
        readonly isTooManyConsumers: boolean;
        readonly isToken: boolean;
        readonly asToken: SpRuntimeTokenError;
        readonly isArithmetic: boolean;
        readonly asArithmetic: SpRuntimeArithmeticError;
        readonly isTransactional: boolean;
        readonly asTransactional: SpRuntimeTransactionalError;
        readonly isExhausted: boolean;
        readonly isCorruption: boolean;
        readonly isUnavailable: boolean;
        readonly type:
            | 'Other'
            | 'CannotLookup'
            | 'BadOrigin'
            | 'Module'
            | 'ConsumerRemaining'
            | 'NoProviders'
            | 'TooManyConsumers'
            | 'Token'
            | 'Arithmetic'
            | 'Transactional'
            | 'Exhausted'
            | 'Corruption'
            | 'Unavailable';
    }

    /** @name SpRuntimeModuleError (25) */
    interface SpRuntimeModuleError extends Struct {
        readonly index: u8;
        readonly error: U8aFixed;
    }

    /** @name SpRuntimeTokenError (26) */
    interface SpRuntimeTokenError extends Enum {
        readonly isNoFunds: boolean;
        readonly isWouldDie: boolean;
        readonly isBelowMinimum: boolean;
        readonly isCannotCreate: boolean;
        readonly isUnknownAsset: boolean;
        readonly isFrozen: boolean;
        readonly isUnsupported: boolean;
        readonly type:
            | 'NoFunds'
            | 'WouldDie'
            | 'BelowMinimum'
            | 'CannotCreate'
            | 'UnknownAsset'
            | 'Frozen'
            | 'Unsupported';
    }

    /** @name SpRuntimeArithmeticError (27) */
    interface SpRuntimeArithmeticError extends Enum {
        readonly isUnderflow: boolean;
        readonly isOverflow: boolean;
        readonly isDivisionByZero: boolean;
        readonly type: 'Underflow' | 'Overflow' | 'DivisionByZero';
    }

    /** @name SpRuntimeTransactionalError (28) */
    interface SpRuntimeTransactionalError extends Enum {
        readonly isLimitReached: boolean;
        readonly isNoLayer: boolean;
        readonly type: 'LimitReached' | 'NoLayer';
    }

    /** @name PalletBalancesEvent (29) */
    interface PalletBalancesEvent extends Enum {
        readonly isEndowed: boolean;
        readonly asEndowed: {
            readonly account: AccountId32;
            readonly freeBalance: u128;
        } & Struct;
        readonly isDustLost: boolean;
        readonly asDustLost: {
            readonly account: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isTransfer: boolean;
        readonly asTransfer: {
            readonly from: AccountId32;
            readonly to: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isBalanceSet: boolean;
        readonly asBalanceSet: {
            readonly who: AccountId32;
            readonly free: u128;
            readonly reserved: u128;
        } & Struct;
        readonly isReserved: boolean;
        readonly asReserved: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isUnreserved: boolean;
        readonly asUnreserved: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isReserveRepatriated: boolean;
        readonly asReserveRepatriated: {
            readonly from: AccountId32;
            readonly to: AccountId32;
            readonly amount: u128;
            readonly destinationStatus: FrameSupportTokensMiscBalanceStatus;
        } & Struct;
        readonly isDeposit: boolean;
        readonly asDeposit: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isWithdraw: boolean;
        readonly asWithdraw: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isSlashed: boolean;
        readonly asSlashed: {
            readonly who: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly type:
            | 'Endowed'
            | 'DustLost'
            | 'Transfer'
            | 'BalanceSet'
            | 'Reserved'
            | 'Unreserved'
            | 'ReserveRepatriated'
            | 'Deposit'
            | 'Withdraw'
            | 'Slashed';
    }

    /** @name FrameSupportTokensMiscBalanceStatus (30) */
    interface FrameSupportTokensMiscBalanceStatus extends Enum {
        readonly isFree: boolean;
        readonly isReserved: boolean;
        readonly type: 'Free' | 'Reserved';
    }

    /** @name PalletTransactionPaymentEvent (31) */
    interface PalletTransactionPaymentEvent extends Enum {
        readonly isTransactionFeePaid: boolean;
        readonly asTransactionFeePaid: {
            readonly who: AccountId32;
            readonly actualFee: u128;
            readonly tip: u128;
        } & Struct;
        readonly type: 'TransactionFeePaid';
    }

    /** @name PalletSudoEvent (32) */
    interface PalletSudoEvent extends Enum {
        readonly isSudid: boolean;
        readonly asSudid: {
            readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isKeyChanged: boolean;
        readonly asKeyChanged: {
            readonly oldSudoer: Option<AccountId32>;
        } & Struct;
        readonly isSudoAsDone: boolean;
        readonly asSudoAsDone: {
            readonly sudoResult: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly type: 'Sudid' | 'KeyChanged' | 'SudoAsDone';
    }

    /** @name PalletCreditcoinEvent (36) */
    interface PalletCreditcoinEvent extends Enum {
        readonly isAddressRegistered: boolean;
        readonly asAddressRegistered: ITuple<[H256, PalletCreditcoinAddress]>;
        readonly isCollectCoinsRegistered: boolean;
        readonly asCollectCoinsRegistered: ITuple<[H256, PalletCreditcoinCollectCoinsUnverifiedCollectedCoins]>;
        readonly isTransferRegistered: boolean;
        readonly asTransferRegistered: ITuple<[H256, PalletCreditcoinTransfer]>;
        readonly isTransferVerified: boolean;
        readonly asTransferVerified: H256;
        readonly isCollectedCoinsMinted: boolean;
        readonly asCollectedCoinsMinted: ITuple<[H256, PalletCreditcoinCollectCoinsCollectedCoins]>;
        readonly isTransferProcessed: boolean;
        readonly asTransferProcessed: H256;
        readonly isAskOrderAdded: boolean;
        readonly asAskOrderAdded: ITuple<[PalletCreditcoinAskOrderId, PalletCreditcoinAskOrder]>;
        readonly isBidOrderAdded: boolean;
        readonly asBidOrderAdded: ITuple<[PalletCreditcoinBidOrderId, PalletCreditcoinBidOrder]>;
        readonly isOfferAdded: boolean;
        readonly asOfferAdded: ITuple<[PalletCreditcoinOfferId, PalletCreditcoinOffer]>;
        readonly isDealOrderAdded: boolean;
        readonly asDealOrderAdded: ITuple<[PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
        readonly isDealOrderFunded: boolean;
        readonly asDealOrderFunded: PalletCreditcoinDealOrderId;
        readonly isDealOrderLocked: boolean;
        readonly asDealOrderLocked: PalletCreditcoinDealOrderId;
        readonly isDealOrderClosed: boolean;
        readonly asDealOrderClosed: PalletCreditcoinDealOrderId;
        readonly isLoanExempted: boolean;
        readonly asLoanExempted: PalletCreditcoinDealOrderId;
        readonly isLegacyWalletClaimed: boolean;
        readonly asLegacyWalletClaimed: ITuple<[AccountId32, PalletCreditcoinLegacySighash, u128]>;
        readonly isTransferFailedVerification: boolean;
        readonly asTransferFailedVerification: ITuple<[H256, PalletCreditcoinOcwErrorsVerificationFailureCause]>;
        readonly isCollectCoinsFailedVerification: boolean;
        readonly asCollectCoinsFailedVerification: ITuple<[H256, PalletCreditcoinOcwErrorsVerificationFailureCause]>;
        readonly isCurrencyRegistered: boolean;
        readonly asCurrencyRegistered: ITuple<[H256, PalletCreditcoinPlatformCurrency]>;
        readonly type:
            | 'AddressRegistered'
            | 'CollectCoinsRegistered'
            | 'TransferRegistered'
            | 'TransferVerified'
            | 'CollectedCoinsMinted'
            | 'TransferProcessed'
            | 'AskOrderAdded'
            | 'BidOrderAdded'
            | 'OfferAdded'
            | 'DealOrderAdded'
            | 'DealOrderFunded'
            | 'DealOrderLocked'
            | 'DealOrderClosed'
            | 'LoanExempted'
            | 'LegacyWalletClaimed'
            | 'TransferFailedVerification'
            | 'CollectCoinsFailedVerification'
            | 'CurrencyRegistered';
    }

    /** @name PalletCreditcoinAddress (38) */
    interface PalletCreditcoinAddress extends Struct {
        readonly blockchain: PalletCreditcoinPlatformBlockchain;
        readonly value: Bytes;
        readonly owner: AccountId32;
    }

    /** @name PalletCreditcoinPlatformBlockchain (39) */
    interface PalletCreditcoinPlatformBlockchain extends Enum {
        readonly isEvm: boolean;
        readonly asEvm: PalletCreditcoinPlatformEvmInfo;
        readonly type: 'Evm';
    }

    /** @name PalletCreditcoinPlatformEvmInfo (40) */
    interface PalletCreditcoinPlatformEvmInfo extends Struct {
        readonly chainId: PalletCreditcoinPlatformEvmChainId;
    }

    /** @name PalletCreditcoinPlatformEvmChainId (41) */
    interface PalletCreditcoinPlatformEvmChainId extends Compact<u64> {}

    /** @name PalletCreditcoinCollectCoinsUnverifiedCollectedCoins (44) */
    interface PalletCreditcoinCollectCoinsUnverifiedCollectedCoins extends Struct {
        readonly to: Bytes;
        readonly txId: Bytes;
        readonly contract: PalletCreditcoinOcwTasksCollectCoinsGCreContract;
    }

    /** @name PalletCreditcoinOcwTasksCollectCoinsGCreContract (45) */
    interface PalletCreditcoinOcwTasksCollectCoinsGCreContract extends Struct {
        readonly address: H160;
        readonly chain: PalletCreditcoinPlatformBlockchain;
    }

    /** @name PalletCreditcoinTransfer (49) */
    interface PalletCreditcoinTransfer extends Struct {
        readonly blockchain: PalletCreditcoinPlatformBlockchain;
        readonly kind: PalletCreditcoinPlatformTransferKind;
        readonly from: H256;
        readonly to: H256;
        readonly dealOrderId: PalletCreditcoinDealOrderId;
        readonly amount: U256;
        readonly txId: Bytes;
        readonly block: u32;
        readonly isProcessed: bool;
        readonly accountId: AccountId32;
        readonly timestamp: Option<u64>;
    }

    /** @name PalletCreditcoinPlatformTransferKind (50) */
    interface PalletCreditcoinPlatformTransferKind extends Enum {
        readonly isEvm: boolean;
        readonly asEvm: PalletCreditcoinPlatformEvmTransferKind;
        readonly type: 'Evm';
    }

    /** @name PalletCreditcoinPlatformEvmTransferKind (51) */
    interface PalletCreditcoinPlatformEvmTransferKind extends Enum {
        readonly isErc20: boolean;
        readonly isEthless: boolean;
        readonly type: 'Erc20' | 'Ethless';
    }

    /** @name PalletCreditcoinDealOrderId (52) */
    interface PalletCreditcoinDealOrderId extends ITuple<[u32, H256]> {}

    /** @name PalletCreditcoinCollectCoinsCollectedCoins (57) */
    interface PalletCreditcoinCollectCoinsCollectedCoins extends Struct {
        readonly to: H256;
        readonly amount: u128;
        readonly txId: Bytes;
    }

    /** @name PalletCreditcoinAskOrderId (58) */
    interface PalletCreditcoinAskOrderId extends ITuple<[u32, H256]> {}

    /** @name PalletCreditcoinAskOrder (59) */
    interface PalletCreditcoinAskOrder extends Struct {
        readonly lenderAddressId: H256;
        readonly terms: PalletCreditcoinLoanTermsAskTerms;
        readonly expirationBlock: u32;
        readonly block: u32;
        readonly lender: AccountId32;
    }

    /** @name PalletCreditcoinLoanTermsAskTerms (60) */
    interface PalletCreditcoinLoanTermsAskTerms extends PalletCreditcoinLoanTerms {}

    /** @name PalletCreditcoinLoanTerms (61) */
    interface PalletCreditcoinLoanTerms extends Struct {
        readonly amount: U256;
        readonly interestRate: PalletCreditcoinLoanTermsInterestRate;
        readonly termLength: PalletCreditcoinLoanTermsDuration;
        readonly currency: H256;
    }

    /** @name PalletCreditcoinLoanTermsInterestRate (62) */
    interface PalletCreditcoinLoanTermsInterestRate extends Struct {
        readonly ratePerPeriod: u64;
        readonly decimals: u64;
        readonly period: PalletCreditcoinLoanTermsDuration;
        readonly interestType: PalletCreditcoinLoanTermsInterestType;
    }

    /** @name PalletCreditcoinLoanTermsDuration (63) */
    interface PalletCreditcoinLoanTermsDuration extends Struct {
        readonly secs: u64;
        readonly nanos: u32;
    }

    /** @name PalletCreditcoinLoanTermsInterestType (64) */
    interface PalletCreditcoinLoanTermsInterestType extends Enum {
        readonly isSimple: boolean;
        readonly isCompound: boolean;
        readonly type: 'Simple' | 'Compound';
    }

    /** @name PalletCreditcoinBidOrderId (66) */
    interface PalletCreditcoinBidOrderId extends ITuple<[u32, H256]> {}

    /** @name PalletCreditcoinBidOrder (67) */
    interface PalletCreditcoinBidOrder extends Struct {
        readonly borrowerAddressId: H256;
        readonly terms: PalletCreditcoinLoanTermsBidTerms;
        readonly expirationBlock: u32;
        readonly block: u32;
        readonly borrower: AccountId32;
    }

    /** @name PalletCreditcoinLoanTermsBidTerms (68) */
    interface PalletCreditcoinLoanTermsBidTerms extends PalletCreditcoinLoanTerms {}

    /** @name PalletCreditcoinOfferId (69) */
    interface PalletCreditcoinOfferId extends ITuple<[u32, H256]> {}

    /** @name PalletCreditcoinOffer (70) */
    interface PalletCreditcoinOffer extends Struct {
        readonly askId: PalletCreditcoinAskOrderId;
        readonly bidId: PalletCreditcoinBidOrderId;
        readonly expirationBlock: u32;
        readonly block: u32;
        readonly lender: AccountId32;
    }

    /** @name PalletCreditcoinDealOrder (71) */
    interface PalletCreditcoinDealOrder extends Struct {
        readonly offerId: PalletCreditcoinOfferId;
        readonly lenderAddressId: H256;
        readonly borrowerAddressId: H256;
        readonly terms: PalletCreditcoinLoanTerms;
        readonly expirationBlock: u32;
        readonly timestamp: u64;
        readonly block: Option<u32>;
        readonly fundingTransferId: Option<H256>;
        readonly repaymentTransferId: Option<H256>;
        readonly lock: Option<AccountId32>;
        readonly borrower: AccountId32;
    }

    /** @name PalletCreditcoinLegacySighash (74) */
    interface PalletCreditcoinLegacySighash extends U8aFixed {}

    /** @name PalletCreditcoinOcwErrorsVerificationFailureCause (76) */
    interface PalletCreditcoinOcwErrorsVerificationFailureCause extends Enum {
        readonly isTaskNonexistent: boolean;
        readonly isTaskFailed: boolean;
        readonly isTaskPending: boolean;
        readonly isTaskUnconfirmed: boolean;
        readonly isTaskInFuture: boolean;
        readonly isIncorrectContract: boolean;
        readonly isMissingReceiver: boolean;
        readonly isMissingSender: boolean;
        readonly isAbiMismatch: boolean;
        readonly isIncorrectInputLength: boolean;
        readonly isEmptyInput: boolean;
        readonly isIncorrectInputType: boolean;
        readonly isIncorrectAmount: boolean;
        readonly isIncorrectNonce: boolean;
        readonly isIncorrectReceiver: boolean;
        readonly isIncorrectSender: boolean;
        readonly isInvalidAddress: boolean;
        readonly isUnsupportedMethod: boolean;
        readonly isTransactionNotFound: boolean;
        readonly type:
            | 'TaskNonexistent'
            | 'TaskFailed'
            | 'TaskPending'
            | 'TaskUnconfirmed'
            | 'TaskInFuture'
            | 'IncorrectContract'
            | 'MissingReceiver'
            | 'MissingSender'
            | 'AbiMismatch'
            | 'IncorrectInputLength'
            | 'EmptyInput'
            | 'IncorrectInputType'
            | 'IncorrectAmount'
            | 'IncorrectNonce'
            | 'IncorrectReceiver'
            | 'IncorrectSender'
            | 'InvalidAddress'
            | 'UnsupportedMethod'
            | 'TransactionNotFound';
    }

    /** @name PalletCreditcoinPlatformCurrency (77) */
    interface PalletCreditcoinPlatformCurrency extends Enum {
        readonly isEvm: boolean;
        readonly asEvm: ITuple<[PalletCreditcoinPlatformEvmCurrencyType, PalletCreditcoinPlatformEvmInfo]>;
        readonly type: 'Evm';
    }

    /** @name PalletCreditcoinPlatformEvmCurrencyType (78) */
    interface PalletCreditcoinPlatformEvmCurrencyType extends Enum {
        readonly isSmartContract: boolean;
        readonly asSmartContract: ITuple<[Bytes, Vec<PalletCreditcoinPlatformEvmTransferKind>]>;
        readonly type: 'SmartContract';
    }

    /** @name PalletRewardsEvent (81) */
    interface PalletRewardsEvent extends Enum {
        readonly isRewardIssued: boolean;
        readonly asRewardIssued: ITuple<[AccountId32, u128]>;
        readonly type: 'RewardIssued';
    }

    /** @name PalletSchedulerEvent (82) */
    interface PalletSchedulerEvent extends Enum {
        readonly isScheduled: boolean;
        readonly asScheduled: {
            readonly when: u32;
            readonly index: u32;
        } & Struct;
        readonly isCanceled: boolean;
        readonly asCanceled: {
            readonly when: u32;
            readonly index: u32;
        } & Struct;
        readonly isDispatched: boolean;
        readonly asDispatched: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isCallUnavailable: boolean;
        readonly asCallUnavailable: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
        } & Struct;
        readonly isPeriodicFailed: boolean;
        readonly asPeriodicFailed: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
        } & Struct;
        readonly isPermanentlyOverweight: boolean;
        readonly asPermanentlyOverweight: {
            readonly task: ITuple<[u32, u32]>;
            readonly id: Option<U8aFixed>;
        } & Struct;
        readonly type:
            | 'Scheduled'
            | 'Canceled'
            | 'Dispatched'
            | 'CallUnavailable'
            | 'PeriodicFailed'
            | 'PermanentlyOverweight';
    }

    /** @name PalletOffchainTaskSchedulerEvent (85) */
    type PalletOffchainTaskSchedulerEvent = Null;

    /** @name FrameSystemPhase (86) */
    interface FrameSystemPhase extends Enum {
        readonly isApplyExtrinsic: boolean;
        readonly asApplyExtrinsic: u32;
        readonly isFinalization: boolean;
        readonly isInitialization: boolean;
        readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
    }

    /** @name FrameSystemLastRuntimeUpgradeInfo (89) */
    interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
        readonly specVersion: Compact<u32>;
        readonly specName: Text;
    }

    /** @name FrameSystemCall (92) */
    interface FrameSystemCall extends Enum {
        readonly isFillBlock: boolean;
        readonly asFillBlock: {
            readonly ratio: Perbill;
        } & Struct;
        readonly isRemark: boolean;
        readonly asRemark: {
            readonly remark: Bytes;
        } & Struct;
        readonly isSetHeapPages: boolean;
        readonly asSetHeapPages: {
            readonly pages: u64;
        } & Struct;
        readonly isSetCode: boolean;
        readonly asSetCode: {
            readonly code: Bytes;
        } & Struct;
        readonly isSetCodeWithoutChecks: boolean;
        readonly asSetCodeWithoutChecks: {
            readonly code: Bytes;
        } & Struct;
        readonly isSetStorage: boolean;
        readonly asSetStorage: {
            readonly items: Vec<ITuple<[Bytes, Bytes]>>;
        } & Struct;
        readonly isKillStorage: boolean;
        readonly asKillStorage: {
            readonly keys_: Vec<Bytes>;
        } & Struct;
        readonly isKillPrefix: boolean;
        readonly asKillPrefix: {
            readonly prefix: Bytes;
            readonly subkeys: u32;
        } & Struct;
        readonly isRemarkWithEvent: boolean;
        readonly asRemarkWithEvent: {
            readonly remark: Bytes;
        } & Struct;
        readonly type:
            | 'FillBlock'
            | 'Remark'
            | 'SetHeapPages'
            | 'SetCode'
            | 'SetCodeWithoutChecks'
            | 'SetStorage'
            | 'KillStorage'
            | 'KillPrefix'
            | 'RemarkWithEvent';
    }

    /** @name FrameSystemLimitsBlockWeights (97) */
    interface FrameSystemLimitsBlockWeights extends Struct {
        readonly baseBlock: SpWeightsWeightV2Weight;
        readonly maxBlock: SpWeightsWeightV2Weight;
        readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    }

    /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (98) */
    interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
        readonly normal: FrameSystemLimitsWeightsPerClass;
        readonly operational: FrameSystemLimitsWeightsPerClass;
        readonly mandatory: FrameSystemLimitsWeightsPerClass;
    }

    /** @name FrameSystemLimitsWeightsPerClass (99) */
    interface FrameSystemLimitsWeightsPerClass extends Struct {
        readonly baseExtrinsic: SpWeightsWeightV2Weight;
        readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
        readonly maxTotal: Option<SpWeightsWeightV2Weight>;
        readonly reserved: Option<SpWeightsWeightV2Weight>;
    }

    /** @name FrameSystemLimitsBlockLength (101) */
    interface FrameSystemLimitsBlockLength extends Struct {
        readonly max: FrameSupportDispatchPerDispatchClassU32;
    }

    /** @name FrameSupportDispatchPerDispatchClassU32 (102) */
    interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
        readonly normal: u32;
        readonly operational: u32;
        readonly mandatory: u32;
    }

    /** @name SpWeightsRuntimeDbWeight (103) */
    interface SpWeightsRuntimeDbWeight extends Struct {
        readonly read: u64;
        readonly write: u64;
    }

    /** @name SpVersionRuntimeVersion (104) */
    interface SpVersionRuntimeVersion extends Struct {
        readonly specName: Text;
        readonly implName: Text;
        readonly authoringVersion: u32;
        readonly specVersion: u32;
        readonly implVersion: u32;
        readonly apis: Vec<ITuple<[U8aFixed, u32]>>;
        readonly transactionVersion: u32;
        readonly stateVersion: u8;
    }

    /** @name FrameSystemError (110) */
    interface FrameSystemError extends Enum {
        readonly isInvalidSpecName: boolean;
        readonly isSpecVersionNeedsToIncrease: boolean;
        readonly isFailedToExtractRuntimeVersion: boolean;
        readonly isNonDefaultComposite: boolean;
        readonly isNonZeroRefCount: boolean;
        readonly isCallFiltered: boolean;
        readonly type:
            | 'InvalidSpecName'
            | 'SpecVersionNeedsToIncrease'
            | 'FailedToExtractRuntimeVersion'
            | 'NonDefaultComposite'
            | 'NonZeroRefCount'
            | 'CallFiltered';
    }

    /** @name PalletTimestampCall (111) */
    interface PalletTimestampCall extends Enum {
        readonly isSet: boolean;
        readonly asSet: {
            readonly now: Compact<u64>;
        } & Struct;
        readonly type: 'Set';
    }

    /** @name PalletBalancesBalanceLock (113) */
    interface PalletBalancesBalanceLock extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
        readonly reasons: PalletBalancesReasons;
    }

    /** @name PalletBalancesReasons (114) */
    interface PalletBalancesReasons extends Enum {
        readonly isFee: boolean;
        readonly isMisc: boolean;
        readonly isAll: boolean;
        readonly type: 'Fee' | 'Misc' | 'All';
    }

    /** @name PalletBalancesReserveData (117) */
    interface PalletBalancesReserveData extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
    }

    /** @name PalletBalancesReleases (119) */
    interface PalletBalancesReleases extends Enum {
        readonly isV100: boolean;
        readonly isV200: boolean;
        readonly type: 'V100' | 'V200';
    }

    /** @name PalletBalancesCall (120) */
    interface PalletBalancesCall extends Enum {
        readonly isTransfer: boolean;
        readonly asTransfer: {
            readonly dest: MultiAddress;
            readonly value: Compact<u128>;
        } & Struct;
        readonly isSetBalance: boolean;
        readonly asSetBalance: {
            readonly who: MultiAddress;
            readonly newFree: Compact<u128>;
            readonly newReserved: Compact<u128>;
        } & Struct;
        readonly isForceTransfer: boolean;
        readonly asForceTransfer: {
            readonly source: MultiAddress;
            readonly dest: MultiAddress;
            readonly value: Compact<u128>;
        } & Struct;
        readonly isTransferKeepAlive: boolean;
        readonly asTransferKeepAlive: {
            readonly dest: MultiAddress;
            readonly value: Compact<u128>;
        } & Struct;
        readonly isTransferAll: boolean;
        readonly asTransferAll: {
            readonly dest: MultiAddress;
            readonly keepAlive: bool;
        } & Struct;
        readonly isForceUnreserve: boolean;
        readonly asForceUnreserve: {
            readonly who: MultiAddress;
            readonly amount: u128;
        } & Struct;
        readonly type:
            | 'Transfer'
            | 'SetBalance'
            | 'ForceTransfer'
            | 'TransferKeepAlive'
            | 'TransferAll'
            | 'ForceUnreserve';
    }

    /** @name PalletBalancesError (124) */
    interface PalletBalancesError extends Enum {
        readonly isVestingBalance: boolean;
        readonly isLiquidityRestrictions: boolean;
        readonly isInsufficientBalance: boolean;
        readonly isExistentialDeposit: boolean;
        readonly isKeepAlive: boolean;
        readonly isExistingVestingSchedule: boolean;
        readonly isDeadAccount: boolean;
        readonly isTooManyReserves: boolean;
        readonly type:
            | 'VestingBalance'
            | 'LiquidityRestrictions'
            | 'InsufficientBalance'
            | 'ExistentialDeposit'
            | 'KeepAlive'
            | 'ExistingVestingSchedule'
            | 'DeadAccount'
            | 'TooManyReserves';
    }

    /** @name PalletTransactionPaymentReleases (126) */
    interface PalletTransactionPaymentReleases extends Enum {
        readonly isV1Ancient: boolean;
        readonly isV2: boolean;
        readonly type: 'V1Ancient' | 'V2';
    }

    /** @name PalletSudoCall (127) */
    interface PalletSudoCall extends Enum {
        readonly isSudo: boolean;
        readonly asSudo: {
            readonly call: Call;
        } & Struct;
        readonly isSudoUncheckedWeight: boolean;
        readonly asSudoUncheckedWeight: {
            readonly call: Call;
            readonly weight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly isSetKey: boolean;
        readonly asSetKey: {
            readonly new_: MultiAddress;
        } & Struct;
        readonly isSudoAs: boolean;
        readonly asSudoAs: {
            readonly who: MultiAddress;
            readonly call: Call;
        } & Struct;
        readonly type: 'Sudo' | 'SudoUncheckedWeight' | 'SetKey' | 'SudoAs';
    }

    /** @name PalletCreditcoinCall (129) */
    interface PalletCreditcoinCall extends Enum {
        readonly isClaimLegacyWallet: boolean;
        readonly asClaimLegacyWallet: {
            readonly publicKey: SpCoreEcdsaPublic;
        } & Struct;
        readonly isRegisterAddress: boolean;
        readonly asRegisterAddress: {
            readonly blockchain: PalletCreditcoinPlatformBlockchain;
            readonly address: Bytes;
            readonly ownershipProof: SpCoreEcdsaSignature;
        } & Struct;
        readonly isAddAskOrder: boolean;
        readonly asAddAskOrder: {
            readonly addressId: H256;
            readonly terms: PalletCreditcoinLoanTerms;
            readonly expirationBlock: u32;
            readonly guid: Bytes;
        } & Struct;
        readonly isAddBidOrder: boolean;
        readonly asAddBidOrder: {
            readonly addressId: H256;
            readonly terms: PalletCreditcoinLoanTerms;
            readonly expirationBlock: u32;
            readonly guid: Bytes;
        } & Struct;
        readonly isAddOffer: boolean;
        readonly asAddOffer: {
            readonly askOrderId: PalletCreditcoinAskOrderId;
            readonly bidOrderId: PalletCreditcoinBidOrderId;
            readonly expirationBlock: u32;
        } & Struct;
        readonly isAddDealOrder: boolean;
        readonly asAddDealOrder: {
            readonly offerId: PalletCreditcoinOfferId;
            readonly expirationBlock: u32;
        } & Struct;
        readonly isLockDealOrder: boolean;
        readonly asLockDealOrder: {
            readonly dealOrderId: PalletCreditcoinDealOrderId;
        } & Struct;
        readonly isFundDealOrder: boolean;
        readonly asFundDealOrder: {
            readonly dealOrderId: PalletCreditcoinDealOrderId;
            readonly transferId: H256;
        } & Struct;
        readonly isRegisterDealOrder: boolean;
        readonly asRegisterDealOrder: {
            readonly lenderAddressId: H256;
            readonly borrowerAddressId: H256;
            readonly terms: PalletCreditcoinLoanTerms;
            readonly expirationBlock: u32;
            readonly askGuid: Bytes;
            readonly bidGuid: Bytes;
            readonly borrowerKey: SpRuntimeMultiSigner;
            readonly borrowerSignature: SpRuntimeMultiSignature;
        } & Struct;
        readonly isCloseDealOrder: boolean;
        readonly asCloseDealOrder: {
            readonly dealOrderId: PalletCreditcoinDealOrderId;
            readonly transferId: H256;
        } & Struct;
        readonly isRequestCollectCoins: boolean;
        readonly asRequestCollectCoins: {
            readonly evmAddress: Bytes;
            readonly txId: Bytes;
        } & Struct;
        readonly isRegisterFundingTransferLegacy: boolean;
        readonly asRegisterFundingTransferLegacy: {
            readonly transferKind: PalletCreditcoinLegacyTransferKind;
            readonly dealOrderId: PalletCreditcoinDealOrderId;
            readonly blockchainTxId: Bytes;
        } & Struct;
        readonly isRegisterRepaymentTransferLegacy: boolean;
        readonly asRegisterRepaymentTransferLegacy: {
            readonly transferKind: PalletCreditcoinLegacyTransferKind;
            readonly repaymentAmount: U256;
            readonly dealOrderId: PalletCreditcoinDealOrderId;
            readonly blockchainTxId: Bytes;
        } & Struct;
        readonly isRegisterFundingTransfer: boolean;
        readonly asRegisterFundingTransfer: {
            readonly transferKind: PalletCreditcoinPlatformTransferKind;
            readonly dealOrderId: PalletCreditcoinDealOrderId;
            readonly blockchainTxId: Bytes;
        } & Struct;
        readonly isRegisterRepaymentTransfer: boolean;
        readonly asRegisterRepaymentTransfer: {
            readonly transferKind: PalletCreditcoinPlatformTransferKind;
            readonly repaymentAmount: U256;
            readonly dealOrderId: PalletCreditcoinDealOrderId;
            readonly blockchainTxId: Bytes;
        } & Struct;
        readonly isExempt: boolean;
        readonly asExempt: {
            readonly dealOrderId: PalletCreditcoinDealOrderId;
        } & Struct;
        readonly isPersistTaskOutput: boolean;
        readonly asPersistTaskOutput: {
            readonly deadline: u32;
            readonly taskOutput: PalletCreditcoinTaskOutput;
        } & Struct;
        readonly isFailTask: boolean;
        readonly asFailTask: {
            readonly deadline: u32;
            readonly taskId: PalletCreditcoinTaskId;
            readonly cause: PalletCreditcoinOcwErrorsVerificationFailureCause;
        } & Struct;
        readonly isAddAuthority: boolean;
        readonly asAddAuthority: {
            readonly who: AccountId32;
        } & Struct;
        readonly isRegisterCurrency: boolean;
        readonly asRegisterCurrency: {
            readonly currency: PalletCreditcoinPlatformCurrency;
        } & Struct;
        readonly isSetCollectCoinsContract: boolean;
        readonly asSetCollectCoinsContract: {
            readonly contract: PalletCreditcoinOcwTasksCollectCoinsGCreContract;
        } & Struct;
        readonly isRemoveAuthority: boolean;
        readonly asRemoveAuthority: {
            readonly who: AccountId32;
        } & Struct;
        readonly type:
            | 'ClaimLegacyWallet'
            | 'RegisterAddress'
            | 'AddAskOrder'
            | 'AddBidOrder'
            | 'AddOffer'
            | 'AddDealOrder'
            | 'LockDealOrder'
            | 'FundDealOrder'
            | 'RegisterDealOrder'
            | 'CloseDealOrder'
            | 'RequestCollectCoins'
            | 'RegisterFundingTransferLegacy'
            | 'RegisterRepaymentTransferLegacy'
            | 'RegisterFundingTransfer'
            | 'RegisterRepaymentTransfer'
            | 'Exempt'
            | 'PersistTaskOutput'
            | 'FailTask'
            | 'AddAuthority'
            | 'RegisterCurrency'
            | 'SetCollectCoinsContract'
            | 'RemoveAuthority';
    }

    /** @name SpCoreEcdsaPublic (130) */
    interface SpCoreEcdsaPublic extends U8aFixed {}

    /** @name SpCoreEcdsaSignature (132) */
    interface SpCoreEcdsaSignature extends U8aFixed {}

    /** @name SpRuntimeMultiSigner (134) */
    interface SpRuntimeMultiSigner extends Enum {
        readonly isEd25519: boolean;
        readonly asEd25519: SpCoreEd25519Public;
        readonly isSr25519: boolean;
        readonly asSr25519: SpCoreSr25519Public;
        readonly isEcdsa: boolean;
        readonly asEcdsa: SpCoreEcdsaPublic;
        readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
    }

    /** @name SpCoreEd25519Public (135) */
    interface SpCoreEd25519Public extends U8aFixed {}

    /** @name SpCoreSr25519Public (136) */
    interface SpCoreSr25519Public extends U8aFixed {}

    /** @name SpRuntimeMultiSignature (137) */
    interface SpRuntimeMultiSignature extends Enum {
        readonly isEd25519: boolean;
        readonly asEd25519: SpCoreEd25519Signature;
        readonly isSr25519: boolean;
        readonly asSr25519: SpCoreSr25519Signature;
        readonly isEcdsa: boolean;
        readonly asEcdsa: SpCoreEcdsaSignature;
        readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
    }

    /** @name SpCoreEd25519Signature (138) */
    interface SpCoreEd25519Signature extends U8aFixed {}

    /** @name SpCoreSr25519Signature (140) */
    interface SpCoreSr25519Signature extends U8aFixed {}

    /** @name PalletCreditcoinLegacyTransferKind (141) */
    interface PalletCreditcoinLegacyTransferKind extends Enum {
        readonly isErc20: boolean;
        readonly asErc20: Bytes;
        readonly isEthless: boolean;
        readonly asEthless: Bytes;
        readonly isNative: boolean;
        readonly isOther: boolean;
        readonly asOther: Bytes;
        readonly type: 'Erc20' | 'Ethless' | 'Native' | 'Other';
    }

    /** @name PalletCreditcoinTaskOutput (142) */
    interface PalletCreditcoinTaskOutput extends Enum {
        readonly isVerifyTransfer: boolean;
        readonly asVerifyTransfer: ITuple<[H256, PalletCreditcoinTransfer]>;
        readonly isCollectCoins: boolean;
        readonly asCollectCoins: ITuple<[H256, PalletCreditcoinCollectCoinsCollectedCoins]>;
        readonly type: 'VerifyTransfer' | 'CollectCoins';
    }

    /** @name PalletCreditcoinTaskId (143) */
    interface PalletCreditcoinTaskId extends Enum {
        readonly isVerifyTransfer: boolean;
        readonly asVerifyTransfer: H256;
        readonly isCollectCoins: boolean;
        readonly asCollectCoins: H256;
        readonly type: 'VerifyTransfer' | 'CollectCoins';
    }

    /** @name PalletDifficultyCall (144) */
    interface PalletDifficultyCall extends Enum {
        readonly isSetTargetBlockTime: boolean;
        readonly asSetTargetBlockTime: {
            readonly targetTime: u64;
        } & Struct;
        readonly isSetAdjustmentPeriod: boolean;
        readonly asSetAdjustmentPeriod: {
            readonly period: i64;
        } & Struct;
        readonly type: 'SetTargetBlockTime' | 'SetAdjustmentPeriod';
    }

    /** @name PalletSchedulerCall (146) */
    interface PalletSchedulerCall extends Enum {
        readonly isSchedule: boolean;
        readonly asSchedule: {
            readonly when: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly isCancel: boolean;
        readonly asCancel: {
            readonly when: u32;
            readonly index: u32;
        } & Struct;
        readonly isScheduleNamed: boolean;
        readonly asScheduleNamed: {
            readonly id: U8aFixed;
            readonly when: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly isCancelNamed: boolean;
        readonly asCancelNamed: {
            readonly id: U8aFixed;
        } & Struct;
        readonly isScheduleAfter: boolean;
        readonly asScheduleAfter: {
            readonly after: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly isScheduleNamedAfter: boolean;
        readonly asScheduleNamedAfter: {
            readonly id: U8aFixed;
            readonly after: u32;
            readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
            readonly priority: u8;
            readonly call: Call;
        } & Struct;
        readonly type: 'Schedule' | 'Cancel' | 'ScheduleNamed' | 'CancelNamed' | 'ScheduleAfter' | 'ScheduleNamedAfter';
    }

    /** @name PalletSudoError (148) */
    interface PalletSudoError extends Enum {
        readonly isRequireSudo: boolean;
        readonly type: 'RequireSudo';
    }

    /** @name PalletCreditcoinTask (150) */
    interface PalletCreditcoinTask extends Enum {
        readonly isVerifyTransfer: boolean;
        readonly asVerifyTransfer: PalletCreditcoinTransferUnverifiedTransfer;
        readonly isCollectCoins: boolean;
        readonly asCollectCoins: PalletCreditcoinCollectCoinsUnverifiedCollectedCoins;
        readonly type: 'VerifyTransfer' | 'CollectCoins';
    }

    /** @name PalletCreditcoinTransferUnverifiedTransfer (151) */
    interface PalletCreditcoinTransferUnverifiedTransfer extends Struct {
        readonly transfer: PalletCreditcoinTransfer;
        readonly fromExternal: Bytes;
        readonly toExternal: Bytes;
        readonly deadline: u32;
        readonly currencyToCheck: PalletCreditcoinCurrencyOrLegacyTransferKind;
    }

    /** @name PalletCreditcoinCurrencyOrLegacyTransferKind (152) */
    interface PalletCreditcoinCurrencyOrLegacyTransferKind extends Enum {
        readonly isCurrency: boolean;
        readonly asCurrency: PalletCreditcoinPlatformCurrency;
        readonly isTransferKind: boolean;
        readonly asTransferKind: PalletCreditcoinLegacyTransferKind;
        readonly type: 'Currency' | 'TransferKind';
    }

    /** @name PalletCreditcoinError (154) */
    interface PalletCreditcoinError extends Enum {
        readonly isAddressAlreadyRegistered: boolean;
        readonly isNonExistentAddress: boolean;
        readonly isNonExistentDealOrder: boolean;
        readonly isNonExistentAskOrder: boolean;
        readonly isNonExistentBidOrder: boolean;
        readonly isNonExistentOffer: boolean;
        readonly isNonExistentTransfer: boolean;
        readonly isTransferAlreadyRegistered: boolean;
        readonly isCollectCoinsAlreadyRegistered: boolean;
        readonly isTransferAccountMismatch: boolean;
        readonly isTransferDealOrderMismatch: boolean;
        readonly isTransferAmountMismatch: boolean;
        readonly isTransferAlreadyProcessed: boolean;
        readonly isTransferAmountInsufficient: boolean;
        readonly isMalformedTransfer: boolean;
        readonly isUnsupportedTransferKind: boolean;
        readonly isInsufficientAuthority: boolean;
        readonly isDuplicateId: boolean;
        readonly isNotAddressOwner: boolean;
        readonly isOffchainSignedTxFailed: boolean;
        readonly isNoLocalAcctForSignedTx: boolean;
        readonly isRepaymentOrderNonZeroGain: boolean;
        readonly isAddressBlockchainMismatch: boolean;
        readonly isAlreadyAuthority: boolean;
        readonly isNotAnAuthority: boolean;
        readonly isDuplicateOffer: boolean;
        readonly isDealNotFunded: boolean;
        readonly isDealOrderAlreadyFunded: boolean;
        readonly isDealOrderAlreadyClosed: boolean;
        readonly isDealOrderAlreadyLocked: boolean;
        readonly isDealOrderMustBeLocked: boolean;
        readonly isDuplicateDealOrder: boolean;
        readonly isDealOrderExpired: boolean;
        readonly isAskOrderExpired: boolean;
        readonly isBidOrderExpired: boolean;
        readonly isOfferExpired: boolean;
        readonly isAskBidMismatch: boolean;
        readonly isSameOwner: boolean;
        readonly isInvalidSignature: boolean;
        readonly isNotBorrower: boolean;
        readonly isMalformedDealOrder: boolean;
        readonly isNotLender: boolean;
        readonly isRepaymentOrderUnsupported: boolean;
        readonly isNotLegacyWalletOwner: boolean;
        readonly isLegacyWalletNotFound: boolean;
        readonly isLegacyBalanceKeeperMissing: boolean;
        readonly isGuidAlreadyUsed: boolean;
        readonly isInvalidTermLength: boolean;
        readonly isMalformedExternalAddress: boolean;
        readonly isAddressFormatNotSupported: boolean;
        readonly isOwnershipNotSatisfied: boolean;
        readonly isCurrencyAlreadyRegistered: boolean;
        readonly isDeprecatedExtrinsic: boolean;
        readonly isCurrencyNotRegistered: boolean;
        readonly type:
            | 'AddressAlreadyRegistered'
            | 'NonExistentAddress'
            | 'NonExistentDealOrder'
            | 'NonExistentAskOrder'
            | 'NonExistentBidOrder'
            | 'NonExistentOffer'
            | 'NonExistentTransfer'
            | 'TransferAlreadyRegistered'
            | 'CollectCoinsAlreadyRegistered'
            | 'TransferAccountMismatch'
            | 'TransferDealOrderMismatch'
            | 'TransferAmountMismatch'
            | 'TransferAlreadyProcessed'
            | 'TransferAmountInsufficient'
            | 'MalformedTransfer'
            | 'UnsupportedTransferKind'
            | 'InsufficientAuthority'
            | 'DuplicateId'
            | 'NotAddressOwner'
            | 'OffchainSignedTxFailed'
            | 'NoLocalAcctForSignedTx'
            | 'RepaymentOrderNonZeroGain'
            | 'AddressBlockchainMismatch'
            | 'AlreadyAuthority'
            | 'NotAnAuthority'
            | 'DuplicateOffer'
            | 'DealNotFunded'
            | 'DealOrderAlreadyFunded'
            | 'DealOrderAlreadyClosed'
            | 'DealOrderAlreadyLocked'
            | 'DealOrderMustBeLocked'
            | 'DuplicateDealOrder'
            | 'DealOrderExpired'
            | 'AskOrderExpired'
            | 'BidOrderExpired'
            | 'OfferExpired'
            | 'AskBidMismatch'
            | 'SameOwner'
            | 'InvalidSignature'
            | 'NotBorrower'
            | 'MalformedDealOrder'
            | 'NotLender'
            | 'RepaymentOrderUnsupported'
            | 'NotLegacyWalletOwner'
            | 'LegacyWalletNotFound'
            | 'LegacyBalanceKeeperMissing'
            | 'GuidAlreadyUsed'
            | 'InvalidTermLength'
            | 'MalformedExternalAddress'
            | 'AddressFormatNotSupported'
            | 'OwnershipNotSatisfied'
            | 'CurrencyAlreadyRegistered'
            | 'DeprecatedExtrinsic'
            | 'CurrencyNotRegistered';
    }

    /** @name PalletDifficultyDifficultyAndTimestamp (156) */
    interface PalletDifficultyDifficultyAndTimestamp extends Struct {
        readonly difficulty: U256;
        readonly timestamp: u64;
    }

    /** @name PalletDifficultyError (158) */
    interface PalletDifficultyError extends Enum {
        readonly isZeroTargetTime: boolean;
        readonly isZeroAdjustmentPeriod: boolean;
        readonly isNegativeAdjustmentPeriod: boolean;
        readonly type: 'ZeroTargetTime' | 'ZeroAdjustmentPeriod' | 'NegativeAdjustmentPeriod';
    }

    /** @name PalletSchedulerScheduled (161) */
    interface PalletSchedulerScheduled extends Struct {
        readonly maybeId: Option<U8aFixed>;
        readonly priority: u8;
        readonly call: FrameSupportPreimagesBounded;
        readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
        readonly origin: CreditcoinNodeRuntimeOriginCaller;
    }

    /** @name FrameSupportPreimagesBounded (162) */
    interface FrameSupportPreimagesBounded extends Enum {
        readonly isLegacy: boolean;
        readonly asLegacy: {
            readonly hash_: H256;
        } & Struct;
        readonly isInline: boolean;
        readonly asInline: Bytes;
        readonly isLookup: boolean;
        readonly asLookup: {
            readonly hash_: H256;
            readonly len: u32;
        } & Struct;
        readonly type: 'Legacy' | 'Inline' | 'Lookup';
    }

    /** @name CreditcoinNodeRuntimeOriginCaller (164) */
    interface CreditcoinNodeRuntimeOriginCaller extends Enum {
        readonly isSystem: boolean;
        readonly asSystem: FrameSupportDispatchRawOrigin;
        readonly isVoid: boolean;
        readonly type: 'System' | 'Void';
    }

    /** @name FrameSupportDispatchRawOrigin (165) */
    interface FrameSupportDispatchRawOrigin extends Enum {
        readonly isRoot: boolean;
        readonly isSigned: boolean;
        readonly asSigned: AccountId32;
        readonly isNone: boolean;
        readonly type: 'Root' | 'Signed' | 'None';
    }

    /** @name SpCoreVoid (166) */
    type SpCoreVoid = Null;

    /** @name PalletSchedulerError (168) */
    interface PalletSchedulerError extends Enum {
        readonly isFailedToSchedule: boolean;
        readonly isNotFound: boolean;
        readonly isTargetBlockNumberInPast: boolean;
        readonly isRescheduleNoChange: boolean;
        readonly isNamed: boolean;
        readonly type: 'FailedToSchedule' | 'NotFound' | 'TargetBlockNumberInPast' | 'RescheduleNoChange' | 'Named';
    }

    /** @name PalletOffchainTaskSchedulerError (169) */
    interface PalletOffchainTaskSchedulerError extends Enum {
        readonly isOffchainSignedTxFailed: boolean;
        readonly isNoLocalAcctForSignedTx: boolean;
        readonly type: 'OffchainSignedTxFailed' | 'NoLocalAcctForSignedTx';
    }

    /** @name FrameSystemExtensionsCheckNonZeroSender (172) */
    type FrameSystemExtensionsCheckNonZeroSender = Null;

    /** @name FrameSystemExtensionsCheckSpecVersion (173) */
    type FrameSystemExtensionsCheckSpecVersion = Null;

    /** @name FrameSystemExtensionsCheckTxVersion (174) */
    type FrameSystemExtensionsCheckTxVersion = Null;

    /** @name FrameSystemExtensionsCheckGenesis (175) */
    type FrameSystemExtensionsCheckGenesis = Null;

    /** @name FrameSystemExtensionsCheckNonce (178) */
    interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

    /** @name FrameSystemExtensionsCheckWeight (179) */
    type FrameSystemExtensionsCheckWeight = Null;

    /** @name PalletTransactionPaymentChargeTransactionPayment (180) */
    interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {}

    /** @name CreditcoinNodeRuntimeRuntime (181) */
    type CreditcoinNodeRuntimeRuntime = Null;
} // declare module
