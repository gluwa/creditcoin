// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/lookup';

import type { Bytes, Compact, Enum, Null, Option, Result, Struct, Text, U256, U8aFixed, Vec, bool, i64, u128, u32, u64, u8 } from '@polkadot/types-codec';
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

  /** @name FrameSupportWeightsPerDispatchClassU64 (7) */
  interface FrameSupportWeightsPerDispatchClassU64 extends Struct {
    readonly normal: u64;
    readonly operational: u64;
    readonly mandatory: u64;
  }

  /** @name SpRuntimeDigest (11) */
  interface SpRuntimeDigest extends Struct {
    readonly logs: Vec<SpRuntimeDigestDigestItem>;
  }

  /** @name SpRuntimeDigestDigestItem (13) */
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

  /** @name FrameSystemEventRecord (16) */
  interface FrameSystemEventRecord extends Struct {
    readonly phase: FrameSystemPhase;
    readonly event: Event;
    readonly topics: Vec<H256>;
  }

  /** @name FrameSystemEvent (18) */
  interface FrameSystemEvent extends Enum {
    readonly isExtrinsicSuccess: boolean;
    readonly asExtrinsicSuccess: {
      readonly dispatchInfo: FrameSupportWeightsDispatchInfo;
    } & Struct;
    readonly isExtrinsicFailed: boolean;
    readonly asExtrinsicFailed: {
      readonly dispatchError: SpRuntimeDispatchError;
      readonly dispatchInfo: FrameSupportWeightsDispatchInfo;
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
    readonly type: 'ExtrinsicSuccess' | 'ExtrinsicFailed' | 'CodeUpdated' | 'NewAccount' | 'KilledAccount' | 'Remarked';
  }

  /** @name FrameSupportWeightsDispatchInfo (19) */
  interface FrameSupportWeightsDispatchInfo extends Struct {
    readonly weight: u64;
    readonly class: FrameSupportWeightsDispatchClass;
    readonly paysFee: FrameSupportWeightsPays;
  }

  /** @name FrameSupportWeightsDispatchClass (20) */
  interface FrameSupportWeightsDispatchClass extends Enum {
    readonly isNormal: boolean;
    readonly isOperational: boolean;
    readonly isMandatory: boolean;
    readonly type: 'Normal' | 'Operational' | 'Mandatory';
  }

  /** @name FrameSupportWeightsPays (21) */
  interface FrameSupportWeightsPays extends Enum {
    readonly isYes: boolean;
    readonly isNo: boolean;
    readonly type: 'Yes' | 'No';
  }

  /** @name SpRuntimeDispatchError (22) */
  interface SpRuntimeDispatchError extends Enum {
    readonly isOther: boolean;
    readonly isCannotLookup: boolean;
    readonly isBadOrigin: boolean;
    readonly isModule: boolean;
    readonly asModule: {
      readonly index: u8;
      readonly error: u8;
    } & Struct;
    readonly isConsumerRemaining: boolean;
    readonly isNoProviders: boolean;
    readonly isTooManyConsumers: boolean;
    readonly isToken: boolean;
    readonly asToken: SpRuntimeTokenError;
    readonly isArithmetic: boolean;
    readonly asArithmetic: SpRuntimeArithmeticError;
    readonly type: 'Other' | 'CannotLookup' | 'BadOrigin' | 'Module' | 'ConsumerRemaining' | 'NoProviders' | 'TooManyConsumers' | 'Token' | 'Arithmetic';
  }

  /** @name SpRuntimeTokenError (23) */
  interface SpRuntimeTokenError extends Enum {
    readonly isNoFunds: boolean;
    readonly isWouldDie: boolean;
    readonly isBelowMinimum: boolean;
    readonly isCannotCreate: boolean;
    readonly isUnknownAsset: boolean;
    readonly isFrozen: boolean;
    readonly isUnsupported: boolean;
    readonly type: 'NoFunds' | 'WouldDie' | 'BelowMinimum' | 'CannotCreate' | 'UnknownAsset' | 'Frozen' | 'Unsupported';
  }

  /** @name SpRuntimeArithmeticError (24) */
  interface SpRuntimeArithmeticError extends Enum {
    readonly isUnderflow: boolean;
    readonly isOverflow: boolean;
    readonly isDivisionByZero: boolean;
    readonly type: 'Underflow' | 'Overflow' | 'DivisionByZero';
  }

  /** @name PalletBalancesEvent (25) */
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
    readonly type: 'Endowed' | 'DustLost' | 'Transfer' | 'BalanceSet' | 'Reserved' | 'Unreserved' | 'ReserveRepatriated' | 'Deposit' | 'Withdraw' | 'Slashed';
  }

  /** @name FrameSupportTokensMiscBalanceStatus (26) */
  interface FrameSupportTokensMiscBalanceStatus extends Enum {
    readonly isFree: boolean;
    readonly isReserved: boolean;
    readonly type: 'Free' | 'Reserved';
  }

  /** @name PalletSudoEvent (27) */
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

  /** @name PalletCreditcoinEvent (31) */
  interface PalletCreditcoinEvent extends Enum {
    readonly isAddressRegistered: boolean;
    readonly asAddressRegistered: ITuple<[H256, PalletCreditcoinAddress]>;
    readonly isCollectCoinsRegistered: boolean;
    readonly asCollectCoinsRegistered: ITuple<[H256, PalletCreditcoinUnverifiedCollectedCoins]>;
    readonly isTransferRegistered: boolean;
    readonly asTransferRegistered: ITuple<[H256, PalletCreditcoinTransfer]>;
    readonly isTransferVerified: boolean;
    readonly asTransferVerified: H256;
    readonly isCollectedCoinsMinted: boolean;
    readonly asCollectedCoinsMinted: ITuple<[H256, PalletCreditcoinCollectedCoins]>;
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
    readonly type: 'AddressRegistered' | 'CollectCoinsRegistered' | 'TransferRegistered' | 'TransferVerified' | 'CollectedCoinsMinted' | 'TransferProcessed' | 'AskOrderAdded' | 'BidOrderAdded' | 'OfferAdded' | 'DealOrderAdded' | 'DealOrderFunded' | 'DealOrderLocked' | 'DealOrderClosed' | 'LoanExempted' | 'LegacyWalletClaimed' | 'TransferFailedVerification' | 'CollectCoinsFailedVerification' | 'CurrencyRegistered';
  }

  /** @name PalletCreditcoinAddress (33) */
  interface PalletCreditcoinAddress extends Struct {
    readonly blockchain: PalletCreditcoinPlatformBlockchain;
    readonly value: Bytes;
    readonly owner: AccountId32;
  }

  /** @name PalletCreditcoinPlatformBlockchain (34) */
  interface PalletCreditcoinPlatformBlockchain extends Enum {
    readonly isEvm: boolean;
    readonly asEvm: PalletCreditcoinPlatformEvmInfo;
    readonly type: 'Evm';
  }

  /** @name PalletCreditcoinPlatformEvmInfo (35) */
  interface PalletCreditcoinPlatformEvmInfo extends Struct {
    readonly chainId: PalletCreditcoinPlatformEvmChainId;
  }

  /** @name PalletCreditcoinPlatformEvmChainId (36) */
  interface PalletCreditcoinPlatformEvmChainId extends Compact<u64> {}

  /** @name PalletCreditcoinUnverifiedCollectedCoins (40) */
  interface PalletCreditcoinUnverifiedCollectedCoins extends Struct {
    readonly to: Bytes;
    readonly txId: Bytes;
        readonly contract: PalletCreditcoinOcwTasksCollectCoinsGCreContract;
  }

  /** @name PalletCreditcoinTransfer (42) */
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

  /** @name PalletCreditcoinPlatformTransferKind (43) */
  interface PalletCreditcoinPlatformTransferKind extends Enum {
    readonly isEvm: boolean;
    readonly asEvm: PalletCreditcoinPlatformEvmTransferKind;
    readonly type: 'Evm';
  }

  /** @name PalletCreditcoinPlatformEvmTransferKind (44) */
  interface PalletCreditcoinPlatformEvmTransferKind extends Enum {
    readonly isErc20: boolean;
    readonly isEthless: boolean;
    readonly type: 'Erc20' | 'Ethless';
  }

  /** @name PalletCreditcoinDealOrderId (45) */
  interface PalletCreditcoinDealOrderId extends ITuple<[u32, H256]> {}

  /** @name PalletCreditcoinCollectedCoins (50) */
  interface PalletCreditcoinCollectedCoins extends Struct {
    readonly to: H256;
    readonly amount: u128;
    readonly txId: Bytes;
  }

  /** @name PalletCreditcoinAskOrderId (51) */
  interface PalletCreditcoinAskOrderId extends ITuple<[u32, H256]> {}

  /** @name PalletCreditcoinAskOrder (52) */
  interface PalletCreditcoinAskOrder extends Struct {
    readonly lenderAddressId: H256;
    readonly terms: PalletCreditcoinLoanTermsAskTerms;
    readonly expirationBlock: u32;
    readonly block: u32;
    readonly lender: AccountId32;
  }

  /** @name PalletCreditcoinLoanTermsAskTerms (53) */
  interface PalletCreditcoinLoanTermsAskTerms extends PalletCreditcoinLoanTerms {}

  /** @name PalletCreditcoinLoanTerms (54) */
  interface PalletCreditcoinLoanTerms extends Struct {
    readonly amount: U256;
    readonly interestRate: PalletCreditcoinLoanTermsInterestRate;
    readonly termLength: PalletCreditcoinLoanTermsDuration;
    readonly currency: H256;
  }

  /** @name PalletCreditcoinLoanTermsInterestRate (55) */
  interface PalletCreditcoinLoanTermsInterestRate extends Struct {
    readonly ratePerPeriod: u64;
    readonly decimals: u64;
    readonly period: PalletCreditcoinLoanTermsDuration;
    readonly interestType: PalletCreditcoinLoanTermsInterestType;
  }

  /** @name PalletCreditcoinLoanTermsDuration (56) */
  interface PalletCreditcoinLoanTermsDuration extends Struct {
    readonly secs: u64;
    readonly nanos: u32;
  }

  /** @name PalletCreditcoinLoanTermsInterestType (57) */
  interface PalletCreditcoinLoanTermsInterestType extends Enum {
    readonly isSimple: boolean;
    readonly isCompound: boolean;
    readonly type: 'Simple' | 'Compound';
  }

  /** @name PalletCreditcoinBidOrderId (59) */
  interface PalletCreditcoinBidOrderId extends ITuple<[u32, H256]> {}

  /** @name PalletCreditcoinBidOrder (60) */
  interface PalletCreditcoinBidOrder extends Struct {
    readonly borrowerAddressId: H256;
    readonly terms: PalletCreditcoinLoanTermsBidTerms;
    readonly expirationBlock: u32;
    readonly block: u32;
    readonly borrower: AccountId32;
  }

  /** @name PalletCreditcoinLoanTermsBidTerms (61) */
  interface PalletCreditcoinLoanTermsBidTerms extends PalletCreditcoinLoanTerms {}

  /** @name PalletCreditcoinOfferId (62) */
  interface PalletCreditcoinOfferId extends ITuple<[u32, H256]> {}

  /** @name PalletCreditcoinOffer (63) */
  interface PalletCreditcoinOffer extends Struct {
    readonly askId: PalletCreditcoinAskOrderId;
    readonly bidId: PalletCreditcoinBidOrderId;
    readonly expirationBlock: u32;
    readonly block: u32;
    readonly lender: AccountId32;
  }

  /** @name PalletCreditcoinDealOrder (64) */
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

  /** @name PalletCreditcoinLegacySighash (67) */
  interface PalletCreditcoinLegacySighash extends U8aFixed {}

  /** @name PalletCreditcoinOcwErrorsVerificationFailureCause (69) */
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
    readonly type: 'TaskNonexistent' | 'TaskFailed' | 'TaskPending' | 'TaskUnconfirmed' | 'TaskInFuture' | 'IncorrectContract' | 'MissingReceiver' | 'MissingSender' | 'AbiMismatch' | 'IncorrectInputLength' | 'EmptyInput' | 'IncorrectInputType' | 'IncorrectAmount' | 'IncorrectNonce' | 'IncorrectReceiver' | 'IncorrectSender' | 'InvalidAddress' | 'UnsupportedMethod';
  }

  /** @name PalletCreditcoinPlatformCurrency (70) */
  interface PalletCreditcoinPlatformCurrency extends Enum {
    readonly isEvm: boolean;
    readonly asEvm: ITuple<[PalletCreditcoinPlatformEvmCurrencyType, PalletCreditcoinPlatformEvmInfo]>;
    readonly type: 'Evm';
  }

  /** @name PalletCreditcoinPlatformEvmCurrencyType (71) */
  interface PalletCreditcoinPlatformEvmCurrencyType extends Enum {
    readonly isSmartContract: boolean;
    readonly asSmartContract: ITuple<[Bytes, Vec<PalletCreditcoinPlatformEvmTransferKind>]>;
    readonly type: 'SmartContract';
  }

  /** @name PalletRewardsEvent (74) */
  interface PalletRewardsEvent extends Enum {
    readonly isRewardIssued: boolean;
    readonly asRewardIssued: ITuple<[AccountId32, u128]>;
    readonly type: 'RewardIssued';
  }

  /** @name FrameSystemPhase (75) */
  interface FrameSystemPhase extends Enum {
    readonly isApplyExtrinsic: boolean;
    readonly asApplyExtrinsic: u32;
    readonly isFinalization: boolean;
    readonly isInitialization: boolean;
    readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
  }

  /** @name FrameSystemLastRuntimeUpgradeInfo (79) */
  interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
    readonly specVersion: Compact<u32>;
    readonly specName: Text;
  }

  /** @name FrameSystemCall (82) */
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
    readonly type: 'FillBlock' | 'Remark' | 'SetHeapPages' | 'SetCode' | 'SetCodeWithoutChecks' | 'SetStorage' | 'KillStorage' | 'KillPrefix' | 'RemarkWithEvent';
  }

  /** @name FrameSystemLimitsBlockWeights (87) */
  interface FrameSystemLimitsBlockWeights extends Struct {
    readonly baseBlock: u64;
    readonly maxBlock: u64;
    readonly perClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
  }

  /** @name FrameSupportWeightsPerDispatchClassWeightsPerClass (88) */
  interface FrameSupportWeightsPerDispatchClassWeightsPerClass extends Struct {
    readonly normal: FrameSystemLimitsWeightsPerClass;
    readonly operational: FrameSystemLimitsWeightsPerClass;
    readonly mandatory: FrameSystemLimitsWeightsPerClass;
  }

  /** @name FrameSystemLimitsWeightsPerClass (89) */
  interface FrameSystemLimitsWeightsPerClass extends Struct {
    readonly baseExtrinsic: u64;
    readonly maxExtrinsic: Option<u64>;
    readonly maxTotal: Option<u64>;
    readonly reserved: Option<u64>;
  }

  /** @name FrameSystemLimitsBlockLength (90) */
  interface FrameSystemLimitsBlockLength extends Struct {
    readonly max: FrameSupportWeightsPerDispatchClassU32;
  }

  /** @name FrameSupportWeightsPerDispatchClassU32 (91) */
  interface FrameSupportWeightsPerDispatchClassU32 extends Struct {
    readonly normal: u32;
    readonly operational: u32;
    readonly mandatory: u32;
  }

  /** @name FrameSupportWeightsRuntimeDbWeight (92) */
  interface FrameSupportWeightsRuntimeDbWeight extends Struct {
    readonly read: u64;
    readonly write: u64;
  }

  /** @name SpVersionRuntimeVersion (93) */
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

  /** @name FrameSystemError (99) */
  interface FrameSystemError extends Enum {
    readonly isInvalidSpecName: boolean;
    readonly isSpecVersionNeedsToIncrease: boolean;
    readonly isFailedToExtractRuntimeVersion: boolean;
    readonly isNonDefaultComposite: boolean;
    readonly isNonZeroRefCount: boolean;
    readonly isCallFiltered: boolean;
    readonly type: 'InvalidSpecName' | 'SpecVersionNeedsToIncrease' | 'FailedToExtractRuntimeVersion' | 'NonDefaultComposite' | 'NonZeroRefCount' | 'CallFiltered';
  }

  /** @name PalletTimestampCall (100) */
  interface PalletTimestampCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly now: Compact<u64>;
    } & Struct;
    readonly type: 'Set';
  }

  /** @name PalletBalancesBalanceLock (102) */
  interface PalletBalancesBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
    readonly reasons: PalletBalancesReasons;
  }

  /** @name PalletBalancesReasons (103) */
  interface PalletBalancesReasons extends Enum {
    readonly isFee: boolean;
    readonly isMisc: boolean;
    readonly isAll: boolean;
    readonly type: 'Fee' | 'Misc' | 'All';
  }

  /** @name PalletBalancesReserveData (106) */
  interface PalletBalancesReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name PalletBalancesReleases (108) */
  interface PalletBalancesReleases extends Enum {
    readonly isV100: boolean;
    readonly isV200: boolean;
    readonly type: 'V100' | 'V200';
  }

  /** @name PalletBalancesCall (109) */
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
    readonly type: 'Transfer' | 'SetBalance' | 'ForceTransfer' | 'TransferKeepAlive' | 'TransferAll' | 'ForceUnreserve';
  }

  /** @name PalletBalancesError (114) */
  interface PalletBalancesError extends Enum {
    readonly isVestingBalance: boolean;
    readonly isLiquidityRestrictions: boolean;
    readonly isInsufficientBalance: boolean;
    readonly isExistentialDeposit: boolean;
    readonly isKeepAlive: boolean;
    readonly isExistingVestingSchedule: boolean;
    readonly isDeadAccount: boolean;
    readonly isTooManyReserves: boolean;
    readonly type: 'VestingBalance' | 'LiquidityRestrictions' | 'InsufficientBalance' | 'ExistentialDeposit' | 'KeepAlive' | 'ExistingVestingSchedule' | 'DeadAccount' | 'TooManyReserves';
  }

  /** @name PalletTransactionPaymentReleases (116) */
  interface PalletTransactionPaymentReleases extends Enum {
    readonly isV1Ancient: boolean;
    readonly isV2: boolean;
    readonly type: 'V1Ancient' | 'V2';
  }

  /** @name FrameSupportWeightsWeightToFeeCoefficient (118) */
  interface FrameSupportWeightsWeightToFeeCoefficient extends Struct {
    readonly coeffInteger: u128;
    readonly coeffFrac: Perbill;
    readonly negative: bool;
    readonly degree: u8;
  }

  /** @name PalletSudoCall (119) */
  interface PalletSudoCall extends Enum {
    readonly isSudo: boolean;
    readonly asSudo: {
      readonly call: Call;
    } & Struct;
    readonly isSudoUncheckedWeight: boolean;
    readonly asSudoUncheckedWeight: {
      readonly call: Call;
      readonly weight: u64;
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

  /** @name PalletCreditcoinCall (121) */
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
    readonly type: 'ClaimLegacyWallet' | 'RegisterAddress' | 'AddAskOrder' | 'AddBidOrder' | 'AddOffer' | 'AddDealOrder' | 'LockDealOrder' | 'FundDealOrder' | 'RegisterDealOrder' | 'CloseDealOrder' | 'RequestCollectCoins' | 'RegisterFundingTransferLegacy' | 'RegisterRepaymentTransferLegacy' | 'RegisterFundingTransfer' | 'RegisterRepaymentTransfer' | 'Exempt' | 'PersistTaskOutput' | 'FailTask' | 'AddAuthority' | 'RegisterCurrency';
  }

  /** @name SpCoreEcdsaPublic (122) */
  interface SpCoreEcdsaPublic extends U8aFixed {}

  /** @name SpCoreEcdsaSignature (124) */
  interface SpCoreEcdsaSignature extends U8aFixed {}

  /** @name SpRuntimeMultiSigner (126) */
  interface SpRuntimeMultiSigner extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Public;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Public;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaPublic;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
  }

  /** @name SpCoreEd25519Public (127) */
  interface SpCoreEd25519Public extends U8aFixed {}

  /** @name SpCoreSr25519Public (128) */
  interface SpCoreSr25519Public extends U8aFixed {}

  /** @name SpRuntimeMultiSignature (129) */
  interface SpRuntimeMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaSignature;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
  }

  /** @name SpCoreEd25519Signature (130) */
  interface SpCoreEd25519Signature extends U8aFixed {}

  /** @name SpCoreSr25519Signature (132) */
  interface SpCoreSr25519Signature extends U8aFixed {}

  /** @name PalletCreditcoinLegacyTransferKind (133) */
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

  /** @name PalletCreditcoinTaskOutput (134) */
  interface PalletCreditcoinTaskOutput extends Enum {
    readonly isVerifyTransfer: boolean;
    readonly asVerifyTransfer: ITuple<[H256, PalletCreditcoinTransfer]>;
    readonly isCollectCoins: boolean;
    readonly asCollectCoins: ITuple<[H256, PalletCreditcoinCollectedCoins]>;
    readonly type: 'VerifyTransfer' | 'CollectCoins';
  }

  /** @name PalletCreditcoinTaskId (135) */
  interface PalletCreditcoinTaskId extends Enum {
    readonly isVerifyTransfer: boolean;
    readonly asVerifyTransfer: H256;
    readonly isCollectCoins: boolean;
    readonly asCollectCoins: H256;
    readonly type: 'VerifyTransfer' | 'CollectCoins';
  }

  /** @name PalletDifficultyCall (136) */
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

  /** @name PalletSudoError (138) */
  interface PalletSudoError extends Enum {
    readonly isRequireSudo: boolean;
    readonly type: 'RequireSudo';
  }

  /** @name PalletCreditcoinTask (148) */
  interface PalletCreditcoinTask extends Enum {
    readonly isVerifyTransfer: boolean;
    readonly asVerifyTransfer: PalletCreditcoinUnverifiedTransfer;
    readonly isCollectCoins: boolean;
    readonly asCollectCoins: PalletCreditcoinUnverifiedCollectedCoins;
    readonly type: 'VerifyTransfer' | 'CollectCoins';
  }

  /** @name PalletCreditcoinUnverifiedTransfer (149) */
  interface PalletCreditcoinUnverifiedTransfer extends Struct {
    readonly transfer: PalletCreditcoinTransfer;
    readonly fromExternal: Bytes;
    readonly toExternal: Bytes;
    readonly deadline: u32;
    readonly currencyToCheck: PalletCreditcoinCurrencyOrLegacyTransferKind;
  }

  /** @name PalletCreditcoinCurrencyOrLegacyTransferKind (142) */
  interface PalletCreditcoinCurrencyOrLegacyTransferKind extends Enum {
    readonly isCurrency: boolean;
    readonly asCurrency: PalletCreditcoinPlatformCurrency;
    readonly isTransferKind: boolean;
    readonly asTransferKind: PalletCreditcoinLegacyTransferKind;
    readonly type: 'Currency' | 'TransferKind';
  }

  /** @name PalletCreditcoinError (152) */
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
    readonly isBalanceOverflow: boolean;
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
    readonly isAddressPlatformMismatch: boolean;
    readonly isAlreadyAuthority: boolean;
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
    readonly type: 'AddressAlreadyRegistered' | 'NonExistentAddress' | 'NonExistentDealOrder' | 'NonExistentAskOrder' | 'NonExistentBidOrder' | 'NonExistentOffer' | 'NonExistentTransfer' | 'TransferAlreadyRegistered' | 'CollectCoinsAlreadyRegistered' | 'BalanceOverflow' | 'TransferAccountMismatch' | 'TransferDealOrderMismatch' | 'TransferAmountMismatch' | 'TransferAlreadyProcessed' | 'TransferAmountInsufficient' | 'MalformedTransfer' | 'UnsupportedTransferKind' | 'InsufficientAuthority' | 'DuplicateId' | 'NotAddressOwner' | 'OffchainSignedTxFailed' | 'NoLocalAcctForSignedTx' | 'RepaymentOrderNonZeroGain' | 'AddressPlatformMismatch' | 'AlreadyAuthority' | 'DuplicateOffer' | 'DealNotFunded' | 'DealOrderAlreadyFunded' | 'DealOrderAlreadyClosed' | 'DealOrderAlreadyLocked' | 'DealOrderMustBeLocked' | 'DuplicateDealOrder' | 'DealOrderExpired' | 'AskOrderExpired' | 'BidOrderExpired' | 'OfferExpired' | 'AskBidMismatch' | 'SameOwner' | 'InvalidSignature' | 'NotBorrower' | 'MalformedDealOrder' | 'NotLender' | 'RepaymentOrderUnsupported' | 'NotLegacyWalletOwner' | 'LegacyWalletNotFound' | 'LegacyBalanceKeeperMissing' | 'GuidAlreadyUsed' | 'InvalidTermLength' | 'MalformedExternalAddress' | 'AddressFormatNotSupported' | 'OwnershipNotSatisfied' | 'CurrencyAlreadyRegistered' | 'DeprecatedExtrinsic' | 'CurrencyNotRegistered';
  }

  /** @name PalletDifficultyDifficultyAndTimestamp (146) */
  interface PalletDifficultyDifficultyAndTimestamp extends Struct {
    readonly difficulty: U256;
    readonly timestamp: u64;
  }

  /** @name PalletDifficultyError (148) */
  interface PalletDifficultyError extends Enum {
    readonly isZeroTargetTime: boolean;
    readonly isZeroAdjustmentPeriod: boolean;
    readonly isNegativeAdjustmentPeriod: boolean;
    readonly type: 'ZeroTargetTime' | 'ZeroAdjustmentPeriod' | 'NegativeAdjustmentPeriod';
  }

  /** @name FrameSystemExtensionsCheckSpecVersion (151) */
  type FrameSystemExtensionsCheckSpecVersion = Null;

  /** @name FrameSystemExtensionsCheckTxVersion (152) */
  type FrameSystemExtensionsCheckTxVersion = Null;

  /** @name FrameSystemExtensionsCheckGenesis (153) */
  type FrameSystemExtensionsCheckGenesis = Null;

  /** @name FrameSystemExtensionsCheckNonce (156) */
  interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

  /** @name FrameSystemExtensionsCheckWeight (157) */
  type FrameSystemExtensionsCheckWeight = Null;

  /** @name PalletTransactionPaymentChargeTransactionPayment (158) */
  interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {}

  /** @name CreditcoinNodeRuntimeRuntime (159) */
  type CreditcoinNodeRuntimeRuntime = Null;

} // declare module
