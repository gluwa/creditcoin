// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

declare module '@polkadot/types/lookup' {
  import type { Bytes, Compact, Enum, Null, Option, Result, Struct, Text, U256, U8aFixed, Vec, bool, i64, u128, u32, u64, u8 } from '@polkadot/types-codec';
  import type { ITuple } from '@polkadot/types-codec/types';
  import type { AccountId32, Call, H256, MultiAddress, Perbill } from '@polkadot/types/interfaces/runtime';
  import type { Event } from '@polkadot/types/interfaces/system';

  /** @name FrameSystemAccountInfo (3) */
  export interface FrameSystemAccountInfo extends Struct {
    readonly nonce: u32;
    readonly consumers: u32;
    readonly providers: u32;
    readonly sufficients: u32;
    readonly data: PalletBalancesAccountData;
  }

  /** @name PalletBalancesAccountData (5) */
  export interface PalletBalancesAccountData extends Struct {
    readonly free: u128;
    readonly reserved: u128;
    readonly miscFrozen: u128;
    readonly feeFrozen: u128;
  }

  /** @name FrameSupportWeightsPerDispatchClassU64 (7) */
  export interface FrameSupportWeightsPerDispatchClassU64 extends Struct {
    readonly normal: u64;
    readonly operational: u64;
    readonly mandatory: u64;
  }

  /** @name SpRuntimeDigest (11) */
  export interface SpRuntimeDigest extends Struct {
    readonly logs: Vec<SpRuntimeDigestDigestItem>;
  }

  /** @name SpRuntimeDigestDigestItem (13) */
  export interface SpRuntimeDigestDigestItem extends Enum {
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
  export interface FrameSystemEventRecord extends Struct {
    readonly phase: FrameSystemPhase;
    readonly event: Event;
    readonly topics: Vec<H256>;
  }

  /** @name FrameSystemEvent (18) */
  export interface FrameSystemEvent extends Enum {
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
  export interface FrameSupportWeightsDispatchInfo extends Struct {
    readonly weight: u64;
    readonly class: FrameSupportWeightsDispatchClass;
    readonly paysFee: FrameSupportWeightsPays;
  }

  /** @name FrameSupportWeightsDispatchClass (20) */
  export interface FrameSupportWeightsDispatchClass extends Enum {
    readonly isNormal: boolean;
    readonly isOperational: boolean;
    readonly isMandatory: boolean;
    readonly type: 'Normal' | 'Operational' | 'Mandatory';
  }

  /** @name FrameSupportWeightsPays (21) */
  export interface FrameSupportWeightsPays extends Enum {
    readonly isYes: boolean;
    readonly isNo: boolean;
    readonly type: 'Yes' | 'No';
  }

  /** @name SpRuntimeDispatchError (22) */
  export interface SpRuntimeDispatchError extends Enum {
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
  export interface SpRuntimeTokenError extends Enum {
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
  export interface SpRuntimeArithmeticError extends Enum {
    readonly isUnderflow: boolean;
    readonly isOverflow: boolean;
    readonly isDivisionByZero: boolean;
    readonly type: 'Underflow' | 'Overflow' | 'DivisionByZero';
  }

  /** @name PalletBalancesEvent (25) */
  export interface PalletBalancesEvent extends Enum {
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
  export interface FrameSupportTokensMiscBalanceStatus extends Enum {
    readonly isFree: boolean;
    readonly isReserved: boolean;
    readonly type: 'Free' | 'Reserved';
  }

  /** @name PalletSudoEvent (27) */
  export interface PalletSudoEvent extends Enum {
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
  export interface PalletCreditcoinEvent extends Enum {
    readonly isAddressRegistered: boolean;
    readonly asAddressRegistered: ITuple<[H256, PalletCreditcoinAddress]>;
    readonly isTransferRegistered: boolean;
    readonly asTransferRegistered: ITuple<[H256, PalletCreditcoinTransfer]>;
    readonly isTransferVerified: boolean;
    readonly asTransferVerified: ITuple<[H256, PalletCreditcoinTransfer]>;
    readonly isTransferProcessed: boolean;
    readonly asTransferProcessed: ITuple<[H256, PalletCreditcoinTransfer]>;
    readonly isAskOrderAdded: boolean;
    readonly asAskOrderAdded: ITuple<[PalletCreditcoinAskOrderId, PalletCreditcoinAskOrder]>;
    readonly isBidOrderAdded: boolean;
    readonly asBidOrderAdded: ITuple<[PalletCreditcoinBidOrderId, PalletCreditcoinBidOrder]>;
    readonly isOfferAdded: boolean;
    readonly asOfferAdded: ITuple<[PalletCreditcoinOfferId, PalletCreditcoinOffer]>;
    readonly isDealOrderAdded: boolean;
    readonly asDealOrderAdded: ITuple<[PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
    readonly isDealOrderFunded: boolean;
    readonly asDealOrderFunded: ITuple<[PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
    readonly isDealOrderClosed: boolean;
    readonly asDealOrderClosed: ITuple<[PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
    readonly isLoanExempted: boolean;
    readonly asLoanExempted: ITuple<[PalletCreditcoinDealOrderId, H256]>;
    readonly isLegacyWalletClaimed: boolean;
    readonly asLegacyWalletClaimed: ITuple<[AccountId32, PalletCreditcoinLegacySighash, u128]>;
    readonly type: 'AddressRegistered' | 'TransferRegistered' | 'TransferVerified' | 'TransferProcessed' | 'AskOrderAdded' | 'BidOrderAdded' | 'OfferAdded' | 'DealOrderAdded' | 'DealOrderFunded' | 'DealOrderClosed' | 'LoanExempted' | 'LegacyWalletClaimed';
  }

  /** @name PalletCreditcoinAddress (33) */
  export interface PalletCreditcoinAddress extends Struct {
    readonly blockchain: PalletCreditcoinBlockchain;
    readonly value: Bytes;
    readonly owner: AccountId32;
  }

  /** @name PalletCreditcoinBlockchain (34) */
  export interface PalletCreditcoinBlockchain extends Enum {
    readonly isEthereum: boolean;
    readonly isRinkeby: boolean;
    readonly isLuniverse: boolean;
    readonly isBitcoin: boolean;
    readonly isOther: boolean;
    readonly asOther: Bytes;
    readonly type: 'Ethereum' | 'Rinkeby' | 'Luniverse' | 'Bitcoin' | 'Other';
  }

  /** @name PalletCreditcoinTransfer (37) */
  export interface PalletCreditcoinTransfer extends Struct {
    readonly blockchain: PalletCreditcoinBlockchain;
    readonly kind: PalletCreditcoinTransferKind;
    readonly from: H256;
    readonly to: H256;
    readonly orderId: PalletCreditcoinOrderId;
    readonly amount: U256;
    readonly tx: Bytes;
    readonly block: u32;
    readonly processed: bool;
    readonly sighash: AccountId32;
  }

  /** @name PalletCreditcoinTransferKind (38) */
  export interface PalletCreditcoinTransferKind extends Enum {
    readonly isErc20: boolean;
    readonly asErc20: Bytes;
    readonly isEthless: boolean;
    readonly asEthless: Bytes;
    readonly isNative: boolean;
    readonly isOther: boolean;
    readonly asOther: Bytes;
    readonly type: 'Erc20' | 'Ethless' | 'Native' | 'Other';
  }

  /** @name PalletCreditcoinOrderId (39) */
  export interface PalletCreditcoinOrderId extends Enum {
    readonly isDeal: boolean;
    readonly asDeal: PalletCreditcoinDealOrderId;
    readonly isRepayment: boolean;
    readonly asRepayment: PalletCreditcoinRepaymentOrderId;
    readonly type: 'Deal' | 'Repayment';
  }

  /** @name PalletCreditcoinDealOrderId (40) */
  export interface PalletCreditcoinDealOrderId extends ITuple<[u32, H256]> {}

  /** @name PalletCreditcoinRepaymentOrderId (41) */
  export interface PalletCreditcoinRepaymentOrderId extends ITuple<[u32, H256]> {}

  /** @name PalletCreditcoinAskOrderId (45) */
  export interface PalletCreditcoinAskOrderId extends ITuple<[u32, H256]> {}

  /** @name PalletCreditcoinAskOrder (46) */
  export interface PalletCreditcoinAskOrder extends Struct {
    readonly blockchain: PalletCreditcoinBlockchain;
    readonly lenderAddressId: H256;
    readonly terms: PalletCreditcoinLoanTermsAskTerms;
    readonly expirationBlock: u32;
    readonly block: u32;
    readonly lender: AccountId32;
  }

  /** @name PalletCreditcoinLoanTermsAskTerms (47) */
  export interface PalletCreditcoinLoanTermsAskTerms extends PalletCreditcoinLoanTerms {}

  /** @name PalletCreditcoinLoanTerms (48) */
  export interface PalletCreditcoinLoanTerms extends Struct {
    readonly amount: U256;
    readonly interestRate: u64;
    readonly maturity: u64;
  }

  /** @name PalletCreditcoinBidOrderId (49) */
  export interface PalletCreditcoinBidOrderId extends ITuple<[u32, H256]> {}

  /** @name PalletCreditcoinBidOrder (50) */
  export interface PalletCreditcoinBidOrder extends Struct {
    readonly blockchain: PalletCreditcoinBlockchain;
    readonly borrowerAddressId: H256;
    readonly terms: PalletCreditcoinLoanTermsBidTerms;
    readonly expirationBlock: u32;
    readonly block: u32;
    readonly borrower: AccountId32;
  }

  /** @name PalletCreditcoinLoanTermsBidTerms (51) */
  export interface PalletCreditcoinLoanTermsBidTerms extends PalletCreditcoinLoanTerms {}

  /** @name PalletCreditcoinOfferId (52) */
  export interface PalletCreditcoinOfferId extends ITuple<[u32, H256]> {}

  /** @name PalletCreditcoinOffer (53) */
  export interface PalletCreditcoinOffer extends Struct {
    readonly blockchain: PalletCreditcoinBlockchain;
    readonly askId: PalletCreditcoinAskOrderId;
    readonly bidId: PalletCreditcoinBidOrderId;
    readonly expirationBlock: u32;
    readonly block: u32;
    readonly lender: AccountId32;
  }

  /** @name PalletCreditcoinDealOrder (54) */
  export interface PalletCreditcoinDealOrder extends Struct {
    readonly blockchain: PalletCreditcoinBlockchain;
    readonly offerId: PalletCreditcoinOfferId;
    readonly lenderAddressId: H256;
    readonly borrowerAddressId: H256;
    readonly terms: PalletCreditcoinLoanTerms;
    readonly expirationBlock: u32;
    readonly timestamp: u64;
    readonly fundingTransferId: Option<H256>;
    readonly repaymentTransferId: Option<H256>;
    readonly lock: Option<AccountId32>;
    readonly borrower: AccountId32;
  }

  /** @name PalletCreditcoinLegacySighash (56) */
  export interface PalletCreditcoinLegacySighash extends U8aFixed {}

  /** @name PalletRewardsEvent (58) */
  export interface PalletRewardsEvent extends Enum {
    readonly isRewardIssued: boolean;
    readonly asRewardIssued: ITuple<[AccountId32, u128]>;
    readonly type: 'RewardIssued';
  }

  /** @name FrameSystemPhase (59) */
  export interface FrameSystemPhase extends Enum {
    readonly isApplyExtrinsic: boolean;
    readonly asApplyExtrinsic: u32;
    readonly isFinalization: boolean;
    readonly isInitialization: boolean;
    readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
  }

  /** @name FrameSystemLastRuntimeUpgradeInfo (63) */
  export interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
    readonly specVersion: Compact<u32>;
    readonly specName: Text;
  }

  /** @name FrameSystemCall (66) */
  export interface FrameSystemCall extends Enum {
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

  /** @name FrameSystemLimitsBlockWeights (71) */
  export interface FrameSystemLimitsBlockWeights extends Struct {
    readonly baseBlock: u64;
    readonly maxBlock: u64;
    readonly perClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
  }

  /** @name FrameSupportWeightsPerDispatchClassWeightsPerClass (72) */
  export interface FrameSupportWeightsPerDispatchClassWeightsPerClass extends Struct {
    readonly normal: FrameSystemLimitsWeightsPerClass;
    readonly operational: FrameSystemLimitsWeightsPerClass;
    readonly mandatory: FrameSystemLimitsWeightsPerClass;
  }

  /** @name FrameSystemLimitsWeightsPerClass (73) */
  export interface FrameSystemLimitsWeightsPerClass extends Struct {
    readonly baseExtrinsic: u64;
    readonly maxExtrinsic: Option<u64>;
    readonly maxTotal: Option<u64>;
    readonly reserved: Option<u64>;
  }

  /** @name FrameSystemLimitsBlockLength (75) */
  export interface FrameSystemLimitsBlockLength extends Struct {
    readonly max: FrameSupportWeightsPerDispatchClassU32;
  }

  /** @name FrameSupportWeightsPerDispatchClassU32 (76) */
  export interface FrameSupportWeightsPerDispatchClassU32 extends Struct {
    readonly normal: u32;
    readonly operational: u32;
    readonly mandatory: u32;
  }

  /** @name FrameSupportWeightsRuntimeDbWeight (77) */
  export interface FrameSupportWeightsRuntimeDbWeight extends Struct {
    readonly read: u64;
    readonly write: u64;
  }

  /** @name SpVersionRuntimeVersion (78) */
  export interface SpVersionRuntimeVersion extends Struct {
    readonly specName: Text;
    readonly implName: Text;
    readonly authoringVersion: u32;
    readonly specVersion: u32;
    readonly implVersion: u32;
    readonly apis: Vec<ITuple<[U8aFixed, u32]>>;
    readonly transactionVersion: u32;
    readonly stateVersion: u8;
  }

  /** @name FrameSystemError (84) */
  export interface FrameSystemError extends Enum {
    readonly isInvalidSpecName: boolean;
    readonly isSpecVersionNeedsToIncrease: boolean;
    readonly isFailedToExtractRuntimeVersion: boolean;
    readonly isNonDefaultComposite: boolean;
    readonly isNonZeroRefCount: boolean;
    readonly isCallFiltered: boolean;
    readonly type: 'InvalidSpecName' | 'SpecVersionNeedsToIncrease' | 'FailedToExtractRuntimeVersion' | 'NonDefaultComposite' | 'NonZeroRefCount' | 'CallFiltered';
  }

  /** @name PalletTimestampCall (85) */
  export interface PalletTimestampCall extends Enum {
    readonly isSet: boolean;
    readonly asSet: {
      readonly now: Compact<u64>;
    } & Struct;
    readonly type: 'Set';
  }

  /** @name PalletBalancesBalanceLock (88) */
  export interface PalletBalancesBalanceLock extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
    readonly reasons: PalletBalancesReasons;
  }

  /** @name PalletBalancesReasons (89) */
  export interface PalletBalancesReasons extends Enum {
    readonly isFee: boolean;
    readonly isMisc: boolean;
    readonly isAll: boolean;
    readonly type: 'Fee' | 'Misc' | 'All';
  }

  /** @name PalletBalancesReserveData (92) */
  export interface PalletBalancesReserveData extends Struct {
    readonly id: U8aFixed;
    readonly amount: u128;
  }

  /** @name PalletBalancesReleases (94) */
  export interface PalletBalancesReleases extends Enum {
    readonly isV100: boolean;
    readonly isV200: boolean;
    readonly type: 'V100' | 'V200';
  }

  /** @name PalletBalancesCall (95) */
  export interface PalletBalancesCall extends Enum {
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

  /** @name PalletBalancesError (100) */
  export interface PalletBalancesError extends Enum {
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

  /** @name PalletTransactionPaymentReleases (102) */
  export interface PalletTransactionPaymentReleases extends Enum {
    readonly isV1Ancient: boolean;
    readonly isV2: boolean;
    readonly type: 'V1Ancient' | 'V2';
  }

  /** @name FrameSupportWeightsWeightToFeeCoefficient (104) */
  export interface FrameSupportWeightsWeightToFeeCoefficient extends Struct {
    readonly coeffInteger: u128;
    readonly coeffFrac: Perbill;
    readonly negative: bool;
    readonly degree: u8;
  }

  /** @name PalletSudoCall (105) */
  export interface PalletSudoCall extends Enum {
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

  /** @name PalletCreditcoinCall (107) */
  export interface PalletCreditcoinCall extends Enum {
    readonly isClaimLegacyWallet: boolean;
    readonly asClaimLegacyWallet: {
      readonly publicKey: SpCoreEcdsaPublic;
    } & Struct;
    readonly isRegisterAddress: boolean;
    readonly asRegisterAddress: {
      readonly blockchain: PalletCreditcoinBlockchain;
      readonly address: Bytes;
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
    readonly isRegisterTransfer: boolean;
    readonly asRegisterTransfer: {
      readonly transferKind: PalletCreditcoinTransferKind;
      readonly gain: U256;
      readonly orderId: PalletCreditcoinOrderId;
      readonly blockchainTxId: Bytes;
    } & Struct;
    readonly isExempt: boolean;
    readonly asExempt: {
      readonly dealOrderId: PalletCreditcoinDealOrderId;
      readonly transferId: H256;
    } & Struct;
    readonly isVerifyTransfer: boolean;
    readonly asVerifyTransfer: {
      readonly transfer: PalletCreditcoinTransfer;
    } & Struct;
    readonly isAddAuthority: boolean;
    readonly asAddAuthority: {
      readonly who: AccountId32;
    } & Struct;
    readonly type: 'ClaimLegacyWallet' | 'RegisterAddress' | 'AddAskOrder' | 'AddBidOrder' | 'AddOffer' | 'AddDealOrder' | 'LockDealOrder' | 'FundDealOrder' | 'RegisterDealOrder' | 'CloseDealOrder' | 'RegisterTransfer' | 'Exempt' | 'VerifyTransfer' | 'AddAuthority';
  }

  /** @name SpCoreEcdsaPublic (108) */
  export interface SpCoreEcdsaPublic extends U8aFixed {}

  /** @name SpRuntimeMultiSigner (110) */
  export interface SpRuntimeMultiSigner extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Public;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Public;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaPublic;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
  }

  /** @name SpCoreEd25519Public (111) */
  export interface SpCoreEd25519Public extends U8aFixed {}

  /** @name SpCoreSr25519Public (112) */
  export interface SpCoreSr25519Public extends U8aFixed {}

  /** @name SpRuntimeMultiSignature (113) */
  export interface SpRuntimeMultiSignature extends Enum {
    readonly isEd25519: boolean;
    readonly asEd25519: SpCoreEd25519Signature;
    readonly isSr25519: boolean;
    readonly asSr25519: SpCoreSr25519Signature;
    readonly isEcdsa: boolean;
    readonly asEcdsa: SpCoreEcdsaSignature;
    readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
  }

  /** @name SpCoreEd25519Signature (114) */
  export interface SpCoreEd25519Signature extends U8aFixed {}

  /** @name SpCoreSr25519Signature (116) */
  export interface SpCoreSr25519Signature extends U8aFixed {}

  /** @name SpCoreEcdsaSignature (117) */
  export interface SpCoreEcdsaSignature extends U8aFixed {}

  /** @name PalletDifficultyCall (119) */
  export interface PalletDifficultyCall extends Enum {
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

  /** @name PalletSudoError (121) */
  export interface PalletSudoError extends Enum {
    readonly isRequireSudo: boolean;
    readonly type: 'RequireSudo';
  }

  /** @name PalletCreditcoinUnverifiedTransfer (123) */
  export interface PalletCreditcoinUnverifiedTransfer extends Struct {
    readonly transfer: PalletCreditcoinTransfer;
    readonly fromExternal: Bytes;
    readonly toExternal: Bytes;
  }

  /** @name PalletCreditcoinError (126) */
  export interface PalletCreditcoinError extends Enum {
    readonly isAddressAlreadyRegistered: boolean;
    readonly isNonExistentAddress: boolean;
    readonly isNonExistentDealOrder: boolean;
    readonly isNonExistentAskOrder: boolean;
    readonly isNonExistentBidOrder: boolean;
    readonly isNonExistentOffer: boolean;
    readonly isNonExistentTransfer: boolean;
    readonly isTransferAlreadyRegistered: boolean;
    readonly isTransferMismatch: boolean;
    readonly isTransferAlreadyProcessed: boolean;
    readonly isTransferAmountInsufficient: boolean;
    readonly isMalformedTransfer: boolean;
    readonly isUnsupportedTransferKind: boolean;
    readonly isInsufficientAuthority: boolean;
    readonly isNonExistentRepaymentOrder: boolean;
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
    readonly isScaleDecodeError: boolean;
    readonly isUnverifiedTransferPoolFull: boolean;
    readonly isRepaymentOrderUnsupported: boolean;
    readonly isNotLegacyWalletOwner: boolean;
    readonly isLegacySighashMalformed: boolean;
    readonly isLegacyWalletNotFound: boolean;
    readonly isLegacyBalanceKeeperMissing: boolean;
    readonly isVerifyStringTooLong: boolean;
    readonly isGuidAlreadyUsed: boolean;
    readonly isInvalidMaturity: boolean;
    readonly type: 'AddressAlreadyRegistered' | 'NonExistentAddress' | 'NonExistentDealOrder' | 'NonExistentAskOrder' | 'NonExistentBidOrder' | 'NonExistentOffer' | 'NonExistentTransfer' | 'TransferAlreadyRegistered' | 'TransferMismatch' | 'TransferAlreadyProcessed' | 'TransferAmountInsufficient' | 'MalformedTransfer' | 'UnsupportedTransferKind' | 'InsufficientAuthority' | 'NonExistentRepaymentOrder' | 'DuplicateId' | 'NotAddressOwner' | 'OffchainSignedTxFailed' | 'NoLocalAcctForSignedTx' | 'RepaymentOrderNonZeroGain' | 'AddressPlatformMismatch' | 'AlreadyAuthority' | 'DuplicateOffer' | 'DealNotFunded' | 'DealOrderAlreadyFunded' | 'DealOrderAlreadyClosed' | 'DealOrderAlreadyLocked' | 'DealOrderMustBeLocked' | 'DuplicateDealOrder' | 'DealOrderExpired' | 'AskOrderExpired' | 'BidOrderExpired' | 'OfferExpired' | 'AskBidMismatch' | 'SameOwner' | 'InvalidSignature' | 'NotBorrower' | 'MalformedDealOrder' | 'NotLender' | 'ScaleDecodeError' | 'UnverifiedTransferPoolFull' | 'RepaymentOrderUnsupported' | 'NotLegacyWalletOwner' | 'LegacySighashMalformed' | 'LegacyWalletNotFound' | 'LegacyBalanceKeeperMissing' | 'VerifyStringTooLong' | 'GuidAlreadyUsed' | 'InvalidMaturity';
  }

  /** @name PalletDifficultyDifficultyAndTimestamp (128) */
  export interface PalletDifficultyDifficultyAndTimestamp extends Struct {
    readonly difficulty: U256;
    readonly timestamp: u64;
  }

  /** @name PalletDifficultyError (130) */
  export interface PalletDifficultyError extends Enum {
    readonly isZeroTargetTime: boolean;
    readonly isZeroAdjustmentPeriod: boolean;
    readonly isNegativeAdjustmentPeriod: boolean;
    readonly type: 'ZeroTargetTime' | 'ZeroAdjustmentPeriod' | 'NegativeAdjustmentPeriod';
  }

  /** @name FrameSystemExtensionsCheckSpecVersion (133) */
  export type FrameSystemExtensionsCheckSpecVersion = Null;

  /** @name FrameSystemExtensionsCheckTxVersion (134) */
  export type FrameSystemExtensionsCheckTxVersion = Null;

  /** @name FrameSystemExtensionsCheckGenesis (135) */
  export type FrameSystemExtensionsCheckGenesis = Null;

  /** @name FrameSystemExtensionsCheckNonce (138) */
  export interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

  /** @name FrameSystemExtensionsCheckWeight (139) */
  export type FrameSystemExtensionsCheckWeight = Null;

  /** @name PalletTransactionPaymentChargeTransactionPayment (140) */
  export interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {}

  /** @name CreditcoinNodeRuntimeRuntime (141) */
  export type CreditcoinNodeRuntimeRuntime = Null;

} // declare module
