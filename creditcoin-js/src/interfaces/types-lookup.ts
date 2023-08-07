// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/lookup';

import type { Data } from '@polkadot/types';
import type {
    BTreeMap,
    Bytes,
    Compact,
    Enum,
    Null,
    Option,
    Result,
    Set,
    Struct,
    Text,
    U256,
    U8aFixed,
    Vec,
    bool,
    u128,
    u16,
    u32,
    u64,
    u8,
} from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { OpaqueMultiaddr, OpaquePeerId } from '@polkadot/types/interfaces/imOnline';
import type { AccountId32, Call, H160, H256, MultiAddress, Perbill, Percent } from '@polkadot/types/interfaces/runtime';
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
        readonly asArithmetic: SpArithmeticArithmeticError;
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

    /** @name SpArithmeticArithmeticError (27) */
    interface SpArithmeticArithmeticError extends Enum {
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

    /** @name PalletPosSwitchEvent (29) */
    interface PalletPosSwitchEvent extends Enum {
        readonly isSwitched: boolean;
        readonly type: 'Switched';
    }

    /** @name PalletBalancesEvent (30) */
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

    /** @name FrameSupportTokensMiscBalanceStatus (31) */
    interface FrameSupportTokensMiscBalanceStatus extends Enum {
        readonly isFree: boolean;
        readonly isReserved: boolean;
        readonly type: 'Free' | 'Reserved';
    }

    /** @name PalletStakingPalletEvent (32) */
    interface PalletStakingPalletEvent extends Enum {
        readonly isEraPaid: boolean;
        readonly asEraPaid: {
            readonly eraIndex: u32;
            readonly validatorPayout: u128;
            readonly remainder: u128;
        } & Struct;
        readonly isRewarded: boolean;
        readonly asRewarded: {
            readonly stash: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isSlashed: boolean;
        readonly asSlashed: {
            readonly staker: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isSlashReported: boolean;
        readonly asSlashReported: {
            readonly validator: AccountId32;
            readonly fraction: Perbill;
            readonly slashEra: u32;
        } & Struct;
        readonly isOldSlashingReportDiscarded: boolean;
        readonly asOldSlashingReportDiscarded: {
            readonly sessionIndex: u32;
        } & Struct;
        readonly isStakersElected: boolean;
        readonly isBonded: boolean;
        readonly asBonded: {
            readonly stash: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isUnbonded: boolean;
        readonly asUnbonded: {
            readonly stash: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isWithdrawn: boolean;
        readonly asWithdrawn: {
            readonly stash: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isKicked: boolean;
        readonly asKicked: {
            readonly nominator: AccountId32;
            readonly stash: AccountId32;
        } & Struct;
        readonly isStakingElectionFailed: boolean;
        readonly isChilled: boolean;
        readonly asChilled: {
            readonly stash: AccountId32;
        } & Struct;
        readonly isPayoutStarted: boolean;
        readonly asPayoutStarted: {
            readonly eraIndex: u32;
            readonly validatorStash: AccountId32;
        } & Struct;
        readonly isValidatorPrefsSet: boolean;
        readonly asValidatorPrefsSet: {
            readonly stash: AccountId32;
            readonly prefs: PalletStakingValidatorPrefs;
        } & Struct;
        readonly isForceEra: boolean;
        readonly asForceEra: {
            readonly mode: PalletStakingForcing;
        } & Struct;
        readonly type:
            | 'EraPaid'
            | 'Rewarded'
            | 'Slashed'
            | 'SlashReported'
            | 'OldSlashingReportDiscarded'
            | 'StakersElected'
            | 'Bonded'
            | 'Unbonded'
            | 'Withdrawn'
            | 'Kicked'
            | 'StakingElectionFailed'
            | 'Chilled'
            | 'PayoutStarted'
            | 'ValidatorPrefsSet'
            | 'ForceEra';
    }

    /** @name PalletStakingValidatorPrefs (34) */
    interface PalletStakingValidatorPrefs extends Struct {
        readonly commission: Compact<Perbill>;
        readonly blocked: bool;
    }

    /** @name PalletStakingForcing (37) */
    interface PalletStakingForcing extends Enum {
        readonly isNotForcing: boolean;
        readonly isForceNew: boolean;
        readonly isForceNone: boolean;
        readonly isForceAlways: boolean;
        readonly type: 'NotForcing' | 'ForceNew' | 'ForceNone' | 'ForceAlways';
    }

    /** @name PalletOffencesEvent (38) */
    interface PalletOffencesEvent extends Enum {
        readonly isOffence: boolean;
        readonly asOffence: {
            readonly kind: U8aFixed;
            readonly timeslot: Bytes;
        } & Struct;
        readonly type: 'Offence';
    }

    /** @name PalletSessionEvent (40) */
    interface PalletSessionEvent extends Enum {
        readonly isNewSession: boolean;
        readonly asNewSession: {
            readonly sessionIndex: u32;
        } & Struct;
        readonly type: 'NewSession';
    }

    /** @name PalletGrandpaEvent (41) */
    interface PalletGrandpaEvent extends Enum {
        readonly isNewAuthorities: boolean;
        readonly asNewAuthorities: {
            readonly authoritySet: Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>>;
        } & Struct;
        readonly isPaused: boolean;
        readonly isResumed: boolean;
        readonly type: 'NewAuthorities' | 'Paused' | 'Resumed';
    }

    /** @name SpConsensusGrandpaAppPublic (44) */
    interface SpConsensusGrandpaAppPublic extends SpCoreEd25519Public {}

    /** @name SpCoreEd25519Public (45) */
    interface SpCoreEd25519Public extends U8aFixed {}

    /** @name PalletImOnlineEvent (46) */
    interface PalletImOnlineEvent extends Enum {
        readonly isHeartbeatReceived: boolean;
        readonly asHeartbeatReceived: {
            readonly authorityId: PalletImOnlineSr25519AppSr25519Public;
        } & Struct;
        readonly isAllGood: boolean;
        readonly isSomeOffline: boolean;
        readonly asSomeOffline: {
            readonly offline: Vec<ITuple<[AccountId32, PalletStakingExposure]>>;
        } & Struct;
        readonly type: 'HeartbeatReceived' | 'AllGood' | 'SomeOffline';
    }

    /** @name PalletImOnlineSr25519AppSr25519Public (47) */
    interface PalletImOnlineSr25519AppSr25519Public extends SpCoreSr25519Public {}

    /** @name SpCoreSr25519Public (48) */
    interface SpCoreSr25519Public extends U8aFixed {}

    /** @name PalletStakingExposure (51) */
    interface PalletStakingExposure extends Struct {
        readonly total: Compact<u128>;
        readonly own: Compact<u128>;
        readonly others: Vec<PalletStakingIndividualExposure>;
    }

    /** @name PalletStakingIndividualExposure (54) */
    interface PalletStakingIndividualExposure extends Struct {
        readonly who: AccountId32;
        readonly value: Compact<u128>;
    }

    /** @name PalletBagsListEvent (55) */
    interface PalletBagsListEvent extends Enum {
        readonly isRebagged: boolean;
        readonly asRebagged: {
            readonly who: AccountId32;
            readonly from: u64;
            readonly to: u64;
        } & Struct;
        readonly isScoreUpdated: boolean;
        readonly asScoreUpdated: {
            readonly who: AccountId32;
            readonly newScore: u64;
        } & Struct;
        readonly type: 'Rebagged' | 'ScoreUpdated';
    }

    /** @name PalletTransactionPaymentEvent (56) */
    interface PalletTransactionPaymentEvent extends Enum {
        readonly isTransactionFeePaid: boolean;
        readonly asTransactionFeePaid: {
            readonly who: AccountId32;
            readonly actualFee: u128;
            readonly tip: u128;
        } & Struct;
        readonly type: 'TransactionFeePaid';
    }

    /** @name PalletSudoEvent (57) */
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

    /** @name PalletCreditcoinEvent (61) */
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
            | 'CollectCoinsFailedVerification';
    }

    /** @name PalletCreditcoinAddress (63) */
    interface PalletCreditcoinAddress extends Struct {
        readonly blockchain: PalletCreditcoinBlockchain;
        readonly value: Bytes;
        readonly owner: AccountId32;
    }

    /** @name PalletCreditcoinBlockchain (64) */
    interface PalletCreditcoinBlockchain extends Enum {
        readonly isEthereum: boolean;
        readonly isRinkeby: boolean;
        readonly isLuniverse: boolean;
        readonly isBitcoin: boolean;
        readonly isOther: boolean;
        readonly asOther: Bytes;
        readonly type: 'Ethereum' | 'Rinkeby' | 'Luniverse' | 'Bitcoin' | 'Other';
    }

    /** @name PalletCreditcoinCollectCoinsUnverifiedCollectedCoins (67) */
    interface PalletCreditcoinCollectCoinsUnverifiedCollectedCoins extends Struct {
        readonly to: Bytes;
        readonly txId: Bytes;
        readonly contract: PalletCreditcoinOcwTasksCollectCoinsDeployedContract;
        readonly contractType: PalletCreditcoinCollectCoinsContractType;
    }

    /** @name PalletCreditcoinOcwTasksCollectCoinsDeployedContract (68) */
    interface PalletCreditcoinOcwTasksCollectCoinsDeployedContract extends Struct {
        readonly address: H160;
        readonly chain: PalletCreditcoinBlockchain;
    }

    /** @name PalletCreditcoinCollectCoinsContractType (71) */
    interface PalletCreditcoinCollectCoinsContractType extends Enum {
        readonly isGcre: boolean;
        readonly isGate: boolean;
        readonly type: 'Gcre' | 'Gate';
    }

    /** @name PalletCreditcoinTransfer (73) */
    interface PalletCreditcoinTransfer extends Struct {
        readonly blockchain: PalletCreditcoinBlockchain;
        readonly kind: PalletCreditcoinTransferKind;
        readonly from: H256;
        readonly to: H256;
        readonly orderId: PalletCreditcoinOrderId;
        readonly amount: U256;
        readonly txId: Bytes;
        readonly block: u32;
        readonly isProcessed: bool;
        readonly accountId: AccountId32;
        readonly timestamp: Option<u64>;
    }

    /** @name PalletCreditcoinTransferKind (74) */
    interface PalletCreditcoinTransferKind extends Enum {
        readonly isErc20: boolean;
        readonly asErc20: Bytes;
        readonly isEthless: boolean;
        readonly asEthless: Bytes;
        readonly isNative: boolean;
        readonly isOther: boolean;
        readonly asOther: Bytes;
        readonly type: 'Erc20' | 'Ethless' | 'Native' | 'Other';
    }

    /** @name PalletCreditcoinOrderId (75) */
    interface PalletCreditcoinOrderId extends Enum {
        readonly isDeal: boolean;
        readonly asDeal: PalletCreditcoinDealOrderId;
        readonly isRepayment: boolean;
        readonly asRepayment: PalletCreditcoinRepaymentOrderId;
        readonly type: 'Deal' | 'Repayment';
    }

    /** @name PalletCreditcoinDealOrderId (76) */
    interface PalletCreditcoinDealOrderId extends ITuple<[u32, H256]> {}

    /** @name PalletCreditcoinRepaymentOrderId (77) */
    interface PalletCreditcoinRepaymentOrderId extends ITuple<[u32, H256]> {}

    /** @name PalletCreditcoinCollectCoinsCollectedCoins (81) */
    interface PalletCreditcoinCollectCoinsCollectedCoins extends Struct {
        readonly to: H256;
        readonly amount: u128;
        readonly txId: Bytes;
        readonly contractType: PalletCreditcoinCollectCoinsContractType;
    }

    /** @name PalletCreditcoinAskOrderId (82) */
    interface PalletCreditcoinAskOrderId extends ITuple<[u32, H256]> {}

    /** @name PalletCreditcoinAskOrder (83) */
    interface PalletCreditcoinAskOrder extends Struct {
        readonly blockchain: PalletCreditcoinBlockchain;
        readonly lenderAddressId: H256;
        readonly terms: PalletCreditcoinLoanTermsAskTerms;
        readonly expirationBlock: u32;
        readonly block: u32;
        readonly lender: AccountId32;
    }

    /** @name PalletCreditcoinLoanTermsAskTerms (84) */
    interface PalletCreditcoinLoanTermsAskTerms extends PalletCreditcoinLoanTerms {}

    /** @name PalletCreditcoinLoanTerms (85) */
    interface PalletCreditcoinLoanTerms extends Struct {
        readonly amount: U256;
        readonly interestRate: PalletCreditcoinLoanTermsInterestRate;
        readonly termLength: PalletCreditcoinLoanTermsDuration;
    }

    /** @name PalletCreditcoinLoanTermsInterestRate (86) */
    interface PalletCreditcoinLoanTermsInterestRate extends Struct {
        readonly ratePerPeriod: u64;
        readonly decimals: u64;
        readonly period: PalletCreditcoinLoanTermsDuration;
        readonly interestType: PalletCreditcoinLoanTermsInterestType;
    }

    /** @name PalletCreditcoinLoanTermsDuration (87) */
    interface PalletCreditcoinLoanTermsDuration extends Struct {
        readonly secs: u64;
        readonly nanos: u32;
    }

    /** @name PalletCreditcoinLoanTermsInterestType (88) */
    interface PalletCreditcoinLoanTermsInterestType extends Enum {
        readonly isSimple: boolean;
        readonly isCompound: boolean;
        readonly type: 'Simple' | 'Compound';
    }

    /** @name PalletCreditcoinBidOrderId (89) */
    interface PalletCreditcoinBidOrderId extends ITuple<[u32, H256]> {}

    /** @name PalletCreditcoinBidOrder (90) */
    interface PalletCreditcoinBidOrder extends Struct {
        readonly blockchain: PalletCreditcoinBlockchain;
        readonly borrowerAddressId: H256;
        readonly terms: PalletCreditcoinLoanTermsBidTerms;
        readonly expirationBlock: u32;
        readonly block: u32;
        readonly borrower: AccountId32;
    }

    /** @name PalletCreditcoinLoanTermsBidTerms (91) */
    interface PalletCreditcoinLoanTermsBidTerms extends PalletCreditcoinLoanTerms {}

    /** @name PalletCreditcoinOfferId (92) */
    interface PalletCreditcoinOfferId extends ITuple<[u32, H256]> {}

    /** @name PalletCreditcoinOffer (93) */
    interface PalletCreditcoinOffer extends Struct {
        readonly blockchain: PalletCreditcoinBlockchain;
        readonly askId: PalletCreditcoinAskOrderId;
        readonly bidId: PalletCreditcoinBidOrderId;
        readonly expirationBlock: u32;
        readonly block: u32;
        readonly lender: AccountId32;
    }

    /** @name PalletCreditcoinDealOrder (94) */
    interface PalletCreditcoinDealOrder extends Struct {
        readonly blockchain: PalletCreditcoinBlockchain;
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

    /** @name PalletCreditcoinLegacySighash (97) */
    interface PalletCreditcoinLegacySighash extends U8aFixed {}

    /** @name PalletCreditcoinOcwErrorsVerificationFailureCause (99) */
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
        readonly isInsufficientFaucetBalance: boolean;
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
            | 'TransactionNotFound'
            | 'InsufficientFaucetBalance';
    }

    /** @name PalletRewardsEvent (100) */
    interface PalletRewardsEvent extends Enum {
        readonly isRewardIssued: boolean;
        readonly asRewardIssued: ITuple<[AccountId32, u128]>;
        readonly type: 'RewardIssued';
    }

    /** @name PalletSchedulerEvent (101) */
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

    /** @name PalletOffchainTaskSchedulerEvent (104) */
    type PalletOffchainTaskSchedulerEvent = Null;

    /** @name PalletIdentityEvent (105) */
    interface PalletIdentityEvent extends Enum {
        readonly isIdentitySet: boolean;
        readonly asIdentitySet: {
            readonly who: AccountId32;
        } & Struct;
        readonly isIdentityCleared: boolean;
        readonly asIdentityCleared: {
            readonly who: AccountId32;
            readonly deposit: u128;
        } & Struct;
        readonly isIdentityKilled: boolean;
        readonly asIdentityKilled: {
            readonly who: AccountId32;
            readonly deposit: u128;
        } & Struct;
        readonly isJudgementRequested: boolean;
        readonly asJudgementRequested: {
            readonly who: AccountId32;
            readonly registrarIndex: u32;
        } & Struct;
        readonly isJudgementUnrequested: boolean;
        readonly asJudgementUnrequested: {
            readonly who: AccountId32;
            readonly registrarIndex: u32;
        } & Struct;
        readonly isJudgementGiven: boolean;
        readonly asJudgementGiven: {
            readonly target: AccountId32;
            readonly registrarIndex: u32;
        } & Struct;
        readonly isRegistrarAdded: boolean;
        readonly asRegistrarAdded: {
            readonly registrarIndex: u32;
        } & Struct;
        readonly isSubIdentityAdded: boolean;
        readonly asSubIdentityAdded: {
            readonly sub: AccountId32;
            readonly main: AccountId32;
            readonly deposit: u128;
        } & Struct;
        readonly isSubIdentityRemoved: boolean;
        readonly asSubIdentityRemoved: {
            readonly sub: AccountId32;
            readonly main: AccountId32;
            readonly deposit: u128;
        } & Struct;
        readonly isSubIdentityRevoked: boolean;
        readonly asSubIdentityRevoked: {
            readonly sub: AccountId32;
            readonly main: AccountId32;
            readonly deposit: u128;
        } & Struct;
        readonly type:
            | 'IdentitySet'
            | 'IdentityCleared'
            | 'IdentityKilled'
            | 'JudgementRequested'
            | 'JudgementUnrequested'
            | 'JudgementGiven'
            | 'RegistrarAdded'
            | 'SubIdentityAdded'
            | 'SubIdentityRemoved'
            | 'SubIdentityRevoked';
    }

    /** @name PalletUtilityEvent (106) */
    interface PalletUtilityEvent extends Enum {
        readonly isBatchInterrupted: boolean;
        readonly asBatchInterrupted: {
            readonly index: u32;
            readonly error: SpRuntimeDispatchError;
        } & Struct;
        readonly isBatchCompleted: boolean;
        readonly isBatchCompletedWithErrors: boolean;
        readonly isItemCompleted: boolean;
        readonly isItemFailed: boolean;
        readonly asItemFailed: {
            readonly error: SpRuntimeDispatchError;
        } & Struct;
        readonly isDispatchedAs: boolean;
        readonly asDispatchedAs: {
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly type:
            | 'BatchInterrupted'
            | 'BatchCompleted'
            | 'BatchCompletedWithErrors'
            | 'ItemCompleted'
            | 'ItemFailed'
            | 'DispatchedAs';
    }

    /** @name PalletProxyEvent (107) */
    interface PalletProxyEvent extends Enum {
        readonly isProxyExecuted: boolean;
        readonly asProxyExecuted: {
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isPureCreated: boolean;
        readonly asPureCreated: {
            readonly pure: AccountId32;
            readonly who: AccountId32;
            readonly proxyType: Null;
            readonly disambiguationIndex: u16;
        } & Struct;
        readonly isAnnounced: boolean;
        readonly asAnnounced: {
            readonly real: AccountId32;
            readonly proxy: AccountId32;
            readonly callHash: H256;
        } & Struct;
        readonly isProxyAdded: boolean;
        readonly asProxyAdded: {
            readonly delegator: AccountId32;
            readonly delegatee: AccountId32;
            readonly proxyType: Null;
            readonly delay: u32;
        } & Struct;
        readonly isProxyRemoved: boolean;
        readonly asProxyRemoved: {
            readonly delegator: AccountId32;
            readonly delegatee: AccountId32;
            readonly proxyType: Null;
            readonly delay: u32;
        } & Struct;
        readonly type: 'ProxyExecuted' | 'PureCreated' | 'Announced' | 'ProxyAdded' | 'ProxyRemoved';
    }

    /** @name PalletFastUnstakeEvent (109) */
    interface PalletFastUnstakeEvent extends Enum {
        readonly isUnstaked: boolean;
        readonly asUnstaked: {
            readonly stash: AccountId32;
            readonly result: Result<Null, SpRuntimeDispatchError>;
        } & Struct;
        readonly isSlashed: boolean;
        readonly asSlashed: {
            readonly stash: AccountId32;
            readonly amount: u128;
        } & Struct;
        readonly isInternalError: boolean;
        readonly isBatchChecked: boolean;
        readonly asBatchChecked: {
            readonly eras: Vec<u32>;
        } & Struct;
        readonly isBatchFinished: boolean;
        readonly asBatchFinished: {
            readonly size_: u32;
        } & Struct;
        readonly type: 'Unstaked' | 'Slashed' | 'InternalError' | 'BatchChecked' | 'BatchFinished';
    }

    /** @name PalletNominationPoolsEvent (111) */
    interface PalletNominationPoolsEvent extends Enum {
        readonly isCreated: boolean;
        readonly asCreated: {
            readonly depositor: AccountId32;
            readonly poolId: u32;
        } & Struct;
        readonly isBonded: boolean;
        readonly asBonded: {
            readonly member: AccountId32;
            readonly poolId: u32;
            readonly bonded: u128;
            readonly joined: bool;
        } & Struct;
        readonly isPaidOut: boolean;
        readonly asPaidOut: {
            readonly member: AccountId32;
            readonly poolId: u32;
            readonly payout: u128;
        } & Struct;
        readonly isUnbonded: boolean;
        readonly asUnbonded: {
            readonly member: AccountId32;
            readonly poolId: u32;
            readonly balance: u128;
            readonly points: u128;
            readonly era: u32;
        } & Struct;
        readonly isWithdrawn: boolean;
        readonly asWithdrawn: {
            readonly member: AccountId32;
            readonly poolId: u32;
            readonly balance: u128;
            readonly points: u128;
        } & Struct;
        readonly isDestroyed: boolean;
        readonly asDestroyed: {
            readonly poolId: u32;
        } & Struct;
        readonly isStateChanged: boolean;
        readonly asStateChanged: {
            readonly poolId: u32;
            readonly newState: PalletNominationPoolsPoolState;
        } & Struct;
        readonly isMemberRemoved: boolean;
        readonly asMemberRemoved: {
            readonly poolId: u32;
            readonly member: AccountId32;
        } & Struct;
        readonly isRolesUpdated: boolean;
        readonly asRolesUpdated: {
            readonly root: Option<AccountId32>;
            readonly bouncer: Option<AccountId32>;
            readonly nominator: Option<AccountId32>;
        } & Struct;
        readonly isPoolSlashed: boolean;
        readonly asPoolSlashed: {
            readonly poolId: u32;
            readonly balance: u128;
        } & Struct;
        readonly isUnbondingPoolSlashed: boolean;
        readonly asUnbondingPoolSlashed: {
            readonly poolId: u32;
            readonly era: u32;
            readonly balance: u128;
        } & Struct;
        readonly isPoolCommissionUpdated: boolean;
        readonly asPoolCommissionUpdated: {
            readonly poolId: u32;
            readonly current: Option<ITuple<[Perbill, AccountId32]>>;
        } & Struct;
        readonly isPoolMaxCommissionUpdated: boolean;
        readonly asPoolMaxCommissionUpdated: {
            readonly poolId: u32;
            readonly maxCommission: Perbill;
        } & Struct;
        readonly isPoolCommissionChangeRateUpdated: boolean;
        readonly asPoolCommissionChangeRateUpdated: {
            readonly poolId: u32;
            readonly changeRate: PalletNominationPoolsCommissionChangeRate;
        } & Struct;
        readonly isPoolCommissionClaimed: boolean;
        readonly asPoolCommissionClaimed: {
            readonly poolId: u32;
            readonly commission: u128;
        } & Struct;
        readonly type:
            | 'Created'
            | 'Bonded'
            | 'PaidOut'
            | 'Unbonded'
            | 'Withdrawn'
            | 'Destroyed'
            | 'StateChanged'
            | 'MemberRemoved'
            | 'RolesUpdated'
            | 'PoolSlashed'
            | 'UnbondingPoolSlashed'
            | 'PoolCommissionUpdated'
            | 'PoolMaxCommissionUpdated'
            | 'PoolCommissionChangeRateUpdated'
            | 'PoolCommissionClaimed';
    }

    /** @name PalletNominationPoolsPoolState (112) */
    interface PalletNominationPoolsPoolState extends Enum {
        readonly isOpen: boolean;
        readonly isBlocked: boolean;
        readonly isDestroying: boolean;
        readonly type: 'Open' | 'Blocked' | 'Destroying';
    }

    /** @name PalletNominationPoolsCommissionChangeRate (115) */
    interface PalletNominationPoolsCommissionChangeRate extends Struct {
        readonly maxIncrease: Perbill;
        readonly minDelay: u32;
    }

    /** @name FrameSystemPhase (116) */
    interface FrameSystemPhase extends Enum {
        readonly isApplyExtrinsic: boolean;
        readonly asApplyExtrinsic: u32;
        readonly isFinalization: boolean;
        readonly isInitialization: boolean;
        readonly type: 'ApplyExtrinsic' | 'Finalization' | 'Initialization';
    }

    /** @name FrameSystemLastRuntimeUpgradeInfo (119) */
    interface FrameSystemLastRuntimeUpgradeInfo extends Struct {
        readonly specVersion: Compact<u32>;
        readonly specName: Text;
    }

    /** @name FrameSystemCall (122) */
    interface FrameSystemCall extends Enum {
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
            | 'Remark'
            | 'SetHeapPages'
            | 'SetCode'
            | 'SetCodeWithoutChecks'
            | 'SetStorage'
            | 'KillStorage'
            | 'KillPrefix'
            | 'RemarkWithEvent';
    }

    /** @name FrameSystemLimitsBlockWeights (126) */
    interface FrameSystemLimitsBlockWeights extends Struct {
        readonly baseBlock: SpWeightsWeightV2Weight;
        readonly maxBlock: SpWeightsWeightV2Weight;
        readonly perClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    }

    /** @name FrameSupportDispatchPerDispatchClassWeightsPerClass (127) */
    interface FrameSupportDispatchPerDispatchClassWeightsPerClass extends Struct {
        readonly normal: FrameSystemLimitsWeightsPerClass;
        readonly operational: FrameSystemLimitsWeightsPerClass;
        readonly mandatory: FrameSystemLimitsWeightsPerClass;
    }

    /** @name FrameSystemLimitsWeightsPerClass (128) */
    interface FrameSystemLimitsWeightsPerClass extends Struct {
        readonly baseExtrinsic: SpWeightsWeightV2Weight;
        readonly maxExtrinsic: Option<SpWeightsWeightV2Weight>;
        readonly maxTotal: Option<SpWeightsWeightV2Weight>;
        readonly reserved: Option<SpWeightsWeightV2Weight>;
    }

    /** @name FrameSystemLimitsBlockLength (130) */
    interface FrameSystemLimitsBlockLength extends Struct {
        readonly max: FrameSupportDispatchPerDispatchClassU32;
    }

    /** @name FrameSupportDispatchPerDispatchClassU32 (131) */
    interface FrameSupportDispatchPerDispatchClassU32 extends Struct {
        readonly normal: u32;
        readonly operational: u32;
        readonly mandatory: u32;
    }

    /** @name SpWeightsRuntimeDbWeight (132) */
    interface SpWeightsRuntimeDbWeight extends Struct {
        readonly read: u64;
        readonly write: u64;
    }

    /** @name SpVersionRuntimeVersion (133) */
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

    /** @name FrameSystemError (138) */
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

    /** @name PalletTimestampCall (139) */
    interface PalletTimestampCall extends Enum {
        readonly isSet: boolean;
        readonly asSet: {
            readonly now: Compact<u64>;
        } & Struct;
        readonly type: 'Set';
    }

    /** @name PalletPosSwitchCall (140) */
    interface PalletPosSwitchCall extends Enum {
        readonly isSwitchToPos: boolean;
        readonly asSwitchToPos: {
            readonly initialValidators: Vec<PalletPosSwitchInitialValidator>;
        } & Struct;
        readonly type: 'SwitchToPos';
    }

    /** @name PalletPosSwitchInitialValidator (142) */
    interface PalletPosSwitchInitialValidator extends Struct {
        readonly stash: AccountId32;
        readonly controller: AccountId32;
        readonly bonded: u128;
        readonly controllerBalance: u128;
        readonly babe: SpConsensusBabeAppPublic;
        readonly grandpa: SpConsensusGrandpaAppPublic;
        readonly imOnline: PalletImOnlineSr25519AppSr25519Public;
        readonly invulnerable: bool;
    }

    /** @name SpConsensusBabeAppPublic (143) */
    interface SpConsensusBabeAppPublic extends SpCoreSr25519Public {}

    /** @name PalletPosSwitchError (144) */
    interface PalletPosSwitchError extends Enum {
        readonly isAlreadySwitched: boolean;
        readonly type: 'AlreadySwitched';
    }

    /** @name SpConsensusBabeDigestsNextConfigDescriptor (149) */
    interface SpConsensusBabeDigestsNextConfigDescriptor extends Enum {
        readonly isV1: boolean;
        readonly asV1: {
            readonly c: ITuple<[u64, u64]>;
            readonly allowedSlots: SpConsensusBabeAllowedSlots;
        } & Struct;
        readonly type: 'V1';
    }

    /** @name SpConsensusBabeAllowedSlots (151) */
    interface SpConsensusBabeAllowedSlots extends Enum {
        readonly isPrimarySlots: boolean;
        readonly isPrimaryAndSecondaryPlainSlots: boolean;
        readonly isPrimaryAndSecondaryVRFSlots: boolean;
        readonly type: 'PrimarySlots' | 'PrimaryAndSecondaryPlainSlots' | 'PrimaryAndSecondaryVRFSlots';
    }

    /** @name SpConsensusBabeDigestsPreDigest (155) */
    interface SpConsensusBabeDigestsPreDigest extends Enum {
        readonly isPrimary: boolean;
        readonly asPrimary: SpConsensusBabeDigestsPrimaryPreDigest;
        readonly isSecondaryPlain: boolean;
        readonly asSecondaryPlain: SpConsensusBabeDigestsSecondaryPlainPreDigest;
        readonly isSecondaryVRF: boolean;
        readonly asSecondaryVRF: SpConsensusBabeDigestsSecondaryVRFPreDigest;
        readonly type: 'Primary' | 'SecondaryPlain' | 'SecondaryVRF';
    }

    /** @name SpConsensusBabeDigestsPrimaryPreDigest (156) */
    interface SpConsensusBabeDigestsPrimaryPreDigest extends Struct {
        readonly authorityIndex: u32;
        readonly slot: u64;
        readonly vrfOutput: U8aFixed;
        readonly vrfProof: U8aFixed;
    }

    /** @name SpConsensusBabeDigestsSecondaryPlainPreDigest (158) */
    interface SpConsensusBabeDigestsSecondaryPlainPreDigest extends Struct {
        readonly authorityIndex: u32;
        readonly slot: u64;
    }

    /** @name SpConsensusBabeDigestsSecondaryVRFPreDigest (159) */
    interface SpConsensusBabeDigestsSecondaryVRFPreDigest extends Struct {
        readonly authorityIndex: u32;
        readonly slot: u64;
        readonly vrfOutput: U8aFixed;
        readonly vrfProof: U8aFixed;
    }

    /** @name SpConsensusBabeBabeEpochConfiguration (160) */
    interface SpConsensusBabeBabeEpochConfiguration extends Struct {
        readonly c: ITuple<[u64, u64]>;
        readonly allowedSlots: SpConsensusBabeAllowedSlots;
    }

    /** @name PalletBabeCall (164) */
    interface PalletBabeCall extends Enum {
        readonly isReportEquivocation: boolean;
        readonly asReportEquivocation: {
            readonly equivocationProof: SpConsensusSlotsEquivocationProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isReportEquivocationUnsigned: boolean;
        readonly asReportEquivocationUnsigned: {
            readonly equivocationProof: SpConsensusSlotsEquivocationProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isPlanConfigChange: boolean;
        readonly asPlanConfigChange: {
            readonly config: SpConsensusBabeDigestsNextConfigDescriptor;
        } & Struct;
        readonly type: 'ReportEquivocation' | 'ReportEquivocationUnsigned' | 'PlanConfigChange';
    }

    /** @name SpConsensusSlotsEquivocationProof (165) */
    interface SpConsensusSlotsEquivocationProof extends Struct {
        readonly offender: SpConsensusBabeAppPublic;
        readonly slot: u64;
        readonly firstHeader: SpRuntimeHeader;
        readonly secondHeader: SpRuntimeHeader;
    }

    /** @name SpRuntimeHeader (166) */
    interface SpRuntimeHeader extends Struct {
        readonly parentHash: H256;
        readonly number: Compact<u32>;
        readonly stateRoot: H256;
        readonly extrinsicsRoot: H256;
        readonly digest: SpRuntimeDigest;
    }

    /** @name SpRuntimeBlakeTwo256 (167) */
    type SpRuntimeBlakeTwo256 = Null;

    /** @name SpSessionMembershipProof (168) */
    interface SpSessionMembershipProof extends Struct {
        readonly session: u32;
        readonly trieNodes: Vec<Bytes>;
        readonly validatorCount: u32;
    }

    /** @name PalletBabeError (169) */
    interface PalletBabeError extends Enum {
        readonly isInvalidEquivocationProof: boolean;
        readonly isInvalidKeyOwnershipProof: boolean;
        readonly isDuplicateOffenceReport: boolean;
        readonly isInvalidConfiguration: boolean;
        readonly type:
            | 'InvalidEquivocationProof'
            | 'InvalidKeyOwnershipProof'
            | 'DuplicateOffenceReport'
            | 'InvalidConfiguration';
    }

    /** @name PalletBalancesBalanceLock (171) */
    interface PalletBalancesBalanceLock extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
        readonly reasons: PalletBalancesReasons;
    }

    /** @name PalletBalancesReasons (172) */
    interface PalletBalancesReasons extends Enum {
        readonly isFee: boolean;
        readonly isMisc: boolean;
        readonly isAll: boolean;
        readonly type: 'Fee' | 'Misc' | 'All';
    }

    /** @name PalletBalancesReserveData (175) */
    interface PalletBalancesReserveData extends Struct {
        readonly id: U8aFixed;
        readonly amount: u128;
    }

    /** @name PalletBalancesCall (177) */
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

    /** @name PalletBalancesError (180) */
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

    /** @name PalletStakingStakingLedger (182) */
    interface PalletStakingStakingLedger extends Struct {
        readonly stash: AccountId32;
        readonly total: Compact<u128>;
        readonly active: Compact<u128>;
        readonly unlocking: Vec<PalletStakingUnlockChunk>;
        readonly claimedRewards: Vec<u32>;
    }

    /** @name PalletStakingUnlockChunk (184) */
    interface PalletStakingUnlockChunk extends Struct {
        readonly value: Compact<u128>;
        readonly era: Compact<u32>;
    }

    /** @name PalletStakingRewardDestination (187) */
    interface PalletStakingRewardDestination extends Enum {
        readonly isStaked: boolean;
        readonly isStash: boolean;
        readonly isController: boolean;
        readonly isAccount: boolean;
        readonly asAccount: AccountId32;
        readonly isNone: boolean;
        readonly type: 'Staked' | 'Stash' | 'Controller' | 'Account' | 'None';
    }

    /** @name PalletStakingNominations (188) */
    interface PalletStakingNominations extends Struct {
        readonly targets: Vec<AccountId32>;
        readonly submittedIn: u32;
        readonly suppressed: bool;
    }

    /** @name PalletStakingActiveEraInfo (190) */
    interface PalletStakingActiveEraInfo extends Struct {
        readonly index: u32;
        readonly start: Option<u64>;
    }

    /** @name PalletStakingEraRewardPoints (192) */
    interface PalletStakingEraRewardPoints extends Struct {
        readonly total: u32;
        readonly individual: BTreeMap<AccountId32, u32>;
    }

    /** @name PalletStakingUnappliedSlash (197) */
    interface PalletStakingUnappliedSlash extends Struct {
        readonly validator: AccountId32;
        readonly own: u128;
        readonly others: Vec<ITuple<[AccountId32, u128]>>;
        readonly reporters: Vec<AccountId32>;
        readonly payout: u128;
    }

    /** @name PalletStakingSlashingSlashingSpans (201) */
    interface PalletStakingSlashingSlashingSpans extends Struct {
        readonly spanIndex: u32;
        readonly lastStart: u32;
        readonly lastNonzeroSlash: u32;
        readonly prior: Vec<u32>;
    }

    /** @name PalletStakingSlashingSpanRecord (202) */
    interface PalletStakingSlashingSpanRecord extends Struct {
        readonly slashed: u128;
        readonly paidOut: u128;
    }

    /** @name PalletStakingPalletCall (206) */
    interface PalletStakingPalletCall extends Enum {
        readonly isBond: boolean;
        readonly asBond: {
            readonly controller: MultiAddress;
            readonly value: Compact<u128>;
            readonly payee: PalletStakingRewardDestination;
        } & Struct;
        readonly isBondExtra: boolean;
        readonly asBondExtra: {
            readonly maxAdditional: Compact<u128>;
        } & Struct;
        readonly isUnbond: boolean;
        readonly asUnbond: {
            readonly value: Compact<u128>;
        } & Struct;
        readonly isWithdrawUnbonded: boolean;
        readonly asWithdrawUnbonded: {
            readonly numSlashingSpans: u32;
        } & Struct;
        readonly isValidate: boolean;
        readonly asValidate: {
            readonly prefs: PalletStakingValidatorPrefs;
        } & Struct;
        readonly isNominate: boolean;
        readonly asNominate: {
            readonly targets: Vec<MultiAddress>;
        } & Struct;
        readonly isChill: boolean;
        readonly isSetPayee: boolean;
        readonly asSetPayee: {
            readonly payee: PalletStakingRewardDestination;
        } & Struct;
        readonly isSetController: boolean;
        readonly asSetController: {
            readonly controller: MultiAddress;
        } & Struct;
        readonly isSetValidatorCount: boolean;
        readonly asSetValidatorCount: {
            readonly new_: Compact<u32>;
        } & Struct;
        readonly isIncreaseValidatorCount: boolean;
        readonly asIncreaseValidatorCount: {
            readonly additional: Compact<u32>;
        } & Struct;
        readonly isScaleValidatorCount: boolean;
        readonly asScaleValidatorCount: {
            readonly factor: Percent;
        } & Struct;
        readonly isForceNoEras: boolean;
        readonly isForceNewEra: boolean;
        readonly isSetInvulnerables: boolean;
        readonly asSetInvulnerables: {
            readonly invulnerables: Vec<AccountId32>;
        } & Struct;
        readonly isForceUnstake: boolean;
        readonly asForceUnstake: {
            readonly stash: AccountId32;
            readonly numSlashingSpans: u32;
        } & Struct;
        readonly isForceNewEraAlways: boolean;
        readonly isCancelDeferredSlash: boolean;
        readonly asCancelDeferredSlash: {
            readonly era: u32;
            readonly slashIndices: Vec<u32>;
        } & Struct;
        readonly isPayoutStakers: boolean;
        readonly asPayoutStakers: {
            readonly validatorStash: AccountId32;
            readonly era: u32;
        } & Struct;
        readonly isRebond: boolean;
        readonly asRebond: {
            readonly value: Compact<u128>;
        } & Struct;
        readonly isReapStash: boolean;
        readonly asReapStash: {
            readonly stash: AccountId32;
            readonly numSlashingSpans: u32;
        } & Struct;
        readonly isKick: boolean;
        readonly asKick: {
            readonly who: Vec<MultiAddress>;
        } & Struct;
        readonly isSetStakingConfigs: boolean;
        readonly asSetStakingConfigs: {
            readonly minNominatorBond: PalletStakingPalletConfigOpU128;
            readonly minValidatorBond: PalletStakingPalletConfigOpU128;
            readonly maxNominatorCount: PalletStakingPalletConfigOpU32;
            readonly maxValidatorCount: PalletStakingPalletConfigOpU32;
            readonly chillThreshold: PalletStakingPalletConfigOpPercent;
            readonly minCommission: PalletStakingPalletConfigOpPerbill;
        } & Struct;
        readonly isChillOther: boolean;
        readonly asChillOther: {
            readonly controller: AccountId32;
        } & Struct;
        readonly isForceApplyMinCommission: boolean;
        readonly asForceApplyMinCommission: {
            readonly validatorStash: AccountId32;
        } & Struct;
        readonly isSetMinCommission: boolean;
        readonly asSetMinCommission: {
            readonly new_: Perbill;
        } & Struct;
        readonly type:
            | 'Bond'
            | 'BondExtra'
            | 'Unbond'
            | 'WithdrawUnbonded'
            | 'Validate'
            | 'Nominate'
            | 'Chill'
            | 'SetPayee'
            | 'SetController'
            | 'SetValidatorCount'
            | 'IncreaseValidatorCount'
            | 'ScaleValidatorCount'
            | 'ForceNoEras'
            | 'ForceNewEra'
            | 'SetInvulnerables'
            | 'ForceUnstake'
            | 'ForceNewEraAlways'
            | 'CancelDeferredSlash'
            | 'PayoutStakers'
            | 'Rebond'
            | 'ReapStash'
            | 'Kick'
            | 'SetStakingConfigs'
            | 'ChillOther'
            | 'ForceApplyMinCommission'
            | 'SetMinCommission';
    }

    /** @name PalletStakingPalletConfigOpU128 (208) */
    interface PalletStakingPalletConfigOpU128 extends Enum {
        readonly isNoop: boolean;
        readonly isSet: boolean;
        readonly asSet: u128;
        readonly isRemove: boolean;
        readonly type: 'Noop' | 'Set' | 'Remove';
    }

    /** @name PalletStakingPalletConfigOpU32 (209) */
    interface PalletStakingPalletConfigOpU32 extends Enum {
        readonly isNoop: boolean;
        readonly isSet: boolean;
        readonly asSet: u32;
        readonly isRemove: boolean;
        readonly type: 'Noop' | 'Set' | 'Remove';
    }

    /** @name PalletStakingPalletConfigOpPercent (210) */
    interface PalletStakingPalletConfigOpPercent extends Enum {
        readonly isNoop: boolean;
        readonly isSet: boolean;
        readonly asSet: Percent;
        readonly isRemove: boolean;
        readonly type: 'Noop' | 'Set' | 'Remove';
    }

    /** @name PalletStakingPalletConfigOpPerbill (211) */
    interface PalletStakingPalletConfigOpPerbill extends Enum {
        readonly isNoop: boolean;
        readonly isSet: boolean;
        readonly asSet: Perbill;
        readonly isRemove: boolean;
        readonly type: 'Noop' | 'Set' | 'Remove';
    }

    /** @name PalletStakingPalletError (212) */
    interface PalletStakingPalletError extends Enum {
        readonly isNotController: boolean;
        readonly isNotStash: boolean;
        readonly isAlreadyBonded: boolean;
        readonly isAlreadyPaired: boolean;
        readonly isEmptyTargets: boolean;
        readonly isDuplicateIndex: boolean;
        readonly isInvalidSlashIndex: boolean;
        readonly isInsufficientBond: boolean;
        readonly isNoMoreChunks: boolean;
        readonly isNoUnlockChunk: boolean;
        readonly isFundedTarget: boolean;
        readonly isInvalidEraToReward: boolean;
        readonly isInvalidNumberOfNominations: boolean;
        readonly isNotSortedAndUnique: boolean;
        readonly isAlreadyClaimed: boolean;
        readonly isIncorrectHistoryDepth: boolean;
        readonly isIncorrectSlashingSpans: boolean;
        readonly isBadState: boolean;
        readonly isTooManyTargets: boolean;
        readonly isBadTarget: boolean;
        readonly isCannotChillOther: boolean;
        readonly isTooManyNominators: boolean;
        readonly isTooManyValidators: boolean;
        readonly isCommissionTooLow: boolean;
        readonly isBoundNotMet: boolean;
        readonly type:
            | 'NotController'
            | 'NotStash'
            | 'AlreadyBonded'
            | 'AlreadyPaired'
            | 'EmptyTargets'
            | 'DuplicateIndex'
            | 'InvalidSlashIndex'
            | 'InsufficientBond'
            | 'NoMoreChunks'
            | 'NoUnlockChunk'
            | 'FundedTarget'
            | 'InvalidEraToReward'
            | 'InvalidNumberOfNominations'
            | 'NotSortedAndUnique'
            | 'AlreadyClaimed'
            | 'IncorrectHistoryDepth'
            | 'IncorrectSlashingSpans'
            | 'BadState'
            | 'TooManyTargets'
            | 'BadTarget'
            | 'CannotChillOther'
            | 'TooManyNominators'
            | 'TooManyValidators'
            | 'CommissionTooLow'
            | 'BoundNotMet';
    }

    /** @name SpStakingOffenceOffenceDetails (213) */
    interface SpStakingOffenceOffenceDetails extends Struct {
        readonly offender: ITuple<[AccountId32, PalletStakingExposure]>;
        readonly reporters: Vec<AccountId32>;
    }

    /** @name CreditcoinNodeRuntimeOpaqueSessionKeys (218) */
    interface CreditcoinNodeRuntimeOpaqueSessionKeys extends Struct {
        readonly grandpa: SpConsensusGrandpaAppPublic;
        readonly babe: SpConsensusBabeAppPublic;
        readonly imOnline: PalletImOnlineSr25519AppSr25519Public;
    }

    /** @name SpCoreCryptoKeyTypeId (220) */
    interface SpCoreCryptoKeyTypeId extends U8aFixed {}

    /** @name PalletSessionCall (221) */
    interface PalletSessionCall extends Enum {
        readonly isSetKeys: boolean;
        readonly asSetKeys: {
            readonly keys_: CreditcoinNodeRuntimeOpaqueSessionKeys;
            readonly proof: Bytes;
        } & Struct;
        readonly isPurgeKeys: boolean;
        readonly type: 'SetKeys' | 'PurgeKeys';
    }

    /** @name PalletSessionError (222) */
    interface PalletSessionError extends Enum {
        readonly isInvalidProof: boolean;
        readonly isNoAssociatedValidatorId: boolean;
        readonly isDuplicatedKey: boolean;
        readonly isNoKeys: boolean;
        readonly isNoAccount: boolean;
        readonly type: 'InvalidProof' | 'NoAssociatedValidatorId' | 'DuplicatedKey' | 'NoKeys' | 'NoAccount';
    }

    /** @name PalletGrandpaStoredState (223) */
    interface PalletGrandpaStoredState extends Enum {
        readonly isLive: boolean;
        readonly isPendingPause: boolean;
        readonly asPendingPause: {
            readonly scheduledAt: u32;
            readonly delay: u32;
        } & Struct;
        readonly isPaused: boolean;
        readonly isPendingResume: boolean;
        readonly asPendingResume: {
            readonly scheduledAt: u32;
            readonly delay: u32;
        } & Struct;
        readonly type: 'Live' | 'PendingPause' | 'Paused' | 'PendingResume';
    }

    /** @name PalletGrandpaStoredPendingChange (224) */
    interface PalletGrandpaStoredPendingChange extends Struct {
        readonly scheduledAt: u32;
        readonly delay: u32;
        readonly nextAuthorities: Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>>;
        readonly forced: Option<u32>;
    }

    /** @name PalletGrandpaCall (226) */
    interface PalletGrandpaCall extends Enum {
        readonly isReportEquivocation: boolean;
        readonly asReportEquivocation: {
            readonly equivocationProof: SpConsensusGrandpaEquivocationProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isReportEquivocationUnsigned: boolean;
        readonly asReportEquivocationUnsigned: {
            readonly equivocationProof: SpConsensusGrandpaEquivocationProof;
            readonly keyOwnerProof: SpSessionMembershipProof;
        } & Struct;
        readonly isNoteStalled: boolean;
        readonly asNoteStalled: {
            readonly delay: u32;
            readonly bestFinalizedBlockNumber: u32;
        } & Struct;
        readonly type: 'ReportEquivocation' | 'ReportEquivocationUnsigned' | 'NoteStalled';
    }

    /** @name SpConsensusGrandpaEquivocationProof (227) */
    interface SpConsensusGrandpaEquivocationProof extends Struct {
        readonly setId: u64;
        readonly equivocation: SpConsensusGrandpaEquivocation;
    }

    /** @name SpConsensusGrandpaEquivocation (228) */
    interface SpConsensusGrandpaEquivocation extends Enum {
        readonly isPrevote: boolean;
        readonly asPrevote: FinalityGrandpaEquivocationPrevote;
        readonly isPrecommit: boolean;
        readonly asPrecommit: FinalityGrandpaEquivocationPrecommit;
        readonly type: 'Prevote' | 'Precommit';
    }

    /** @name FinalityGrandpaEquivocationPrevote (229) */
    interface FinalityGrandpaEquivocationPrevote extends Struct {
        readonly roundNumber: u64;
        readonly identity: SpConsensusGrandpaAppPublic;
        readonly first: ITuple<[FinalityGrandpaPrevote, SpConsensusGrandpaAppSignature]>;
        readonly second: ITuple<[FinalityGrandpaPrevote, SpConsensusGrandpaAppSignature]>;
    }

    /** @name FinalityGrandpaPrevote (230) */
    interface FinalityGrandpaPrevote extends Struct {
        readonly targetHash: H256;
        readonly targetNumber: u32;
    }

    /** @name SpConsensusGrandpaAppSignature (231) */
    interface SpConsensusGrandpaAppSignature extends SpCoreEd25519Signature {}

    /** @name SpCoreEd25519Signature (232) */
    interface SpCoreEd25519Signature extends U8aFixed {}

    /** @name FinalityGrandpaEquivocationPrecommit (234) */
    interface FinalityGrandpaEquivocationPrecommit extends Struct {
        readonly roundNumber: u64;
        readonly identity: SpConsensusGrandpaAppPublic;
        readonly first: ITuple<[FinalityGrandpaPrecommit, SpConsensusGrandpaAppSignature]>;
        readonly second: ITuple<[FinalityGrandpaPrecommit, SpConsensusGrandpaAppSignature]>;
    }

    /** @name FinalityGrandpaPrecommit (235) */
    interface FinalityGrandpaPrecommit extends Struct {
        readonly targetHash: H256;
        readonly targetNumber: u32;
    }

    /** @name PalletGrandpaError (237) */
    interface PalletGrandpaError extends Enum {
        readonly isPauseFailed: boolean;
        readonly isResumeFailed: boolean;
        readonly isChangePending: boolean;
        readonly isTooSoon: boolean;
        readonly isInvalidKeyOwnershipProof: boolean;
        readonly isInvalidEquivocationProof: boolean;
        readonly isDuplicateOffenceReport: boolean;
        readonly type:
            | 'PauseFailed'
            | 'ResumeFailed'
            | 'ChangePending'
            | 'TooSoon'
            | 'InvalidKeyOwnershipProof'
            | 'InvalidEquivocationProof'
            | 'DuplicateOffenceReport';
    }

    /** @name PalletImOnlineBoundedOpaqueNetworkState (241) */
    interface PalletImOnlineBoundedOpaqueNetworkState extends Struct {
        readonly peerId: Bytes;
        readonly externalAddresses: Vec<Bytes>;
    }

    /** @name PalletImOnlineCall (245) */
    interface PalletImOnlineCall extends Enum {
        readonly isHeartbeat: boolean;
        readonly asHeartbeat: {
            readonly heartbeat: PalletImOnlineHeartbeat;
            readonly signature: PalletImOnlineSr25519AppSr25519Signature;
        } & Struct;
        readonly type: 'Heartbeat';
    }

    /** @name PalletImOnlineHeartbeat (246) */
    interface PalletImOnlineHeartbeat extends Struct {
        readonly blockNumber: u32;
        readonly networkState: SpCoreOffchainOpaqueNetworkState;
        readonly sessionIndex: u32;
        readonly authorityIndex: u32;
        readonly validatorsLen: u32;
    }

    /** @name SpCoreOffchainOpaqueNetworkState (247) */
    interface SpCoreOffchainOpaqueNetworkState extends Struct {
        readonly peerId: OpaquePeerId;
        readonly externalAddresses: Vec<OpaqueMultiaddr>;
    }

    /** @name PalletImOnlineSr25519AppSr25519Signature (251) */
    interface PalletImOnlineSr25519AppSr25519Signature extends SpCoreSr25519Signature {}

    /** @name SpCoreSr25519Signature (252) */
    interface SpCoreSr25519Signature extends U8aFixed {}

    /** @name PalletImOnlineError (253) */
    interface PalletImOnlineError extends Enum {
        readonly isInvalidKey: boolean;
        readonly isDuplicatedHeartbeat: boolean;
        readonly type: 'InvalidKey' | 'DuplicatedHeartbeat';
    }

    /** @name PalletBagsListListNode (254) */
    interface PalletBagsListListNode extends Struct {
        readonly id: AccountId32;
        readonly prev: Option<AccountId32>;
        readonly next: Option<AccountId32>;
        readonly bagUpper: u64;
        readonly score: u64;
    }

    /** @name PalletBagsListListBag (255) */
    interface PalletBagsListListBag extends Struct {
        readonly head: Option<AccountId32>;
        readonly tail: Option<AccountId32>;
    }

    /** @name PalletBagsListCall (256) */
    interface PalletBagsListCall extends Enum {
        readonly isRebag: boolean;
        readonly asRebag: {
            readonly dislocated: MultiAddress;
        } & Struct;
        readonly isPutInFrontOf: boolean;
        readonly asPutInFrontOf: {
            readonly lighter: MultiAddress;
        } & Struct;
        readonly type: 'Rebag' | 'PutInFrontOf';
    }

    /** @name PalletBagsListError (258) */
    interface PalletBagsListError extends Enum {
        readonly isList: boolean;
        readonly asList: PalletBagsListListListError;
        readonly type: 'List';
    }

    /** @name PalletBagsListListListError (259) */
    interface PalletBagsListListListError extends Enum {
        readonly isDuplicate: boolean;
        readonly isNotHeavier: boolean;
        readonly isNotInSameBag: boolean;
        readonly isNodeNotFound: boolean;
        readonly type: 'Duplicate' | 'NotHeavier' | 'NotInSameBag' | 'NodeNotFound';
    }

    /** @name PalletTransactionPaymentReleases (261) */
    interface PalletTransactionPaymentReleases extends Enum {
        readonly isV1Ancient: boolean;
        readonly isV2: boolean;
        readonly type: 'V1Ancient' | 'V2';
    }

    /** @name PalletSudoCall (262) */
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

    /** @name PalletCreditcoinCall (264) */
    interface PalletCreditcoinCall extends Enum {
        readonly isClaimLegacyWallet: boolean;
        readonly asClaimLegacyWallet: {
            readonly publicKey: SpCoreEcdsaPublic;
        } & Struct;
        readonly isRegisterAddress: boolean;
        readonly asRegisterAddress: {
            readonly blockchain: PalletCreditcoinBlockchain;
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
        readonly isRegisterFundingTransfer: boolean;
        readonly asRegisterFundingTransfer: {
            readonly transferKind: PalletCreditcoinTransferKind;
            readonly dealOrderId: PalletCreditcoinDealOrderId;
            readonly blockchainTxId: Bytes;
        } & Struct;
        readonly isRegisterRepaymentTransfer: boolean;
        readonly asRegisterRepaymentTransfer: {
            readonly transferKind: PalletCreditcoinTransferKind;
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
        readonly isSetCollectCoinsContract: boolean;
        readonly asSetCollectCoinsContract: {
            readonly contract: PalletCreditcoinOcwTasksCollectCoinsDeployedContract;
        } & Struct;
        readonly isRemoveAuthority: boolean;
        readonly asRemoveAuthority: {
            readonly who: AccountId32;
        } & Struct;
        readonly isRegisterAddressV2: boolean;
        readonly asRegisterAddressV2: {
            readonly blockchain: PalletCreditcoinBlockchain;
            readonly address: Bytes;
            readonly ownershipProof: PalletCreditcoinOwnershipProof;
        } & Struct;
        readonly isSetBurnGateContract: boolean;
        readonly asSetBurnGateContract: {
            readonly contract: PalletCreditcoinOcwTasksCollectCoinsDeployedContract;
        } & Struct;
        readonly isSetBurnGateFaucetAddress: boolean;
        readonly asSetBurnGateFaucetAddress: {
            readonly address: AccountId32;
        } & Struct;
        readonly isRequestCollectCoinsV2: boolean;
        readonly asRequestCollectCoinsV2: {
            readonly contract: PalletCreditcoinCollectCoinsTokenContract;
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
            | 'RegisterFundingTransfer'
            | 'RegisterRepaymentTransfer'
            | 'Exempt'
            | 'PersistTaskOutput'
            | 'FailTask'
            | 'AddAuthority'
            | 'SetCollectCoinsContract'
            | 'RemoveAuthority'
            | 'RegisterAddressV2'
            | 'SetBurnGateContract'
            | 'SetBurnGateFaucetAddress'
            | 'RequestCollectCoinsV2';
    }

    /** @name SpCoreEcdsaPublic (265) */
    interface SpCoreEcdsaPublic extends U8aFixed {}

    /** @name SpCoreEcdsaSignature (267) */
    interface SpCoreEcdsaSignature extends U8aFixed {}

    /** @name SpRuntimeMultiSigner (269) */
    interface SpRuntimeMultiSigner extends Enum {
        readonly isEd25519: boolean;
        readonly asEd25519: SpCoreEd25519Public;
        readonly isSr25519: boolean;
        readonly asSr25519: SpCoreSr25519Public;
        readonly isEcdsa: boolean;
        readonly asEcdsa: SpCoreEcdsaPublic;
        readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
    }

    /** @name SpRuntimeMultiSignature (270) */
    interface SpRuntimeMultiSignature extends Enum {
        readonly isEd25519: boolean;
        readonly asEd25519: SpCoreEd25519Signature;
        readonly isSr25519: boolean;
        readonly asSr25519: SpCoreSr25519Signature;
        readonly isEcdsa: boolean;
        readonly asEcdsa: SpCoreEcdsaSignature;
        readonly type: 'Ed25519' | 'Sr25519' | 'Ecdsa';
    }

    /** @name PalletCreditcoinTaskOutput (271) */
    interface PalletCreditcoinTaskOutput extends Enum {
        readonly isVerifyTransfer: boolean;
        readonly asVerifyTransfer: ITuple<[H256, PalletCreditcoinTransfer]>;
        readonly isCollectCoins: boolean;
        readonly asCollectCoins: ITuple<[H256, PalletCreditcoinCollectCoinsCollectedCoins]>;
        readonly type: 'VerifyTransfer' | 'CollectCoins';
    }

    /** @name PalletCreditcoinTaskId (272) */
    interface PalletCreditcoinTaskId extends Enum {
        readonly isVerifyTransfer: boolean;
        readonly asVerifyTransfer: H256;
        readonly isCollectCoins: boolean;
        readonly asCollectCoins: H256;
        readonly type: 'VerifyTransfer' | 'CollectCoins';
    }

    /** @name PalletCreditcoinOwnershipProof (273) */
    interface PalletCreditcoinOwnershipProof extends Enum {
        readonly isPersonalSign: boolean;
        readonly asPersonalSign: SpCoreEcdsaSignature;
        readonly isEthSign: boolean;
        readonly asEthSign: SpCoreEcdsaSignature;
        readonly type: 'PersonalSign' | 'EthSign';
    }

    /** @name PalletCreditcoinCollectCoinsTokenContract (274) */
    interface PalletCreditcoinCollectCoinsTokenContract extends Enum {
        readonly isGcre: boolean;
        readonly asGcre: ITuple<[Bytes, Bytes]>;
        readonly isGate: boolean;
        readonly asGate: ITuple<[Bytes, Bytes]>;
        readonly type: 'Gcre' | 'Gate';
    }

    /** @name PalletSchedulerCall (275) */
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

    /** @name PalletIdentityCall (277) */
    interface PalletIdentityCall extends Enum {
        readonly isAddRegistrar: boolean;
        readonly asAddRegistrar: {
            readonly account: MultiAddress;
        } & Struct;
        readonly isSetIdentity: boolean;
        readonly asSetIdentity: {
            readonly info: PalletIdentityIdentityInfo;
        } & Struct;
        readonly isSetSubs: boolean;
        readonly asSetSubs: {
            readonly subs: Vec<ITuple<[AccountId32, Data]>>;
        } & Struct;
        readonly isClearIdentity: boolean;
        readonly isRequestJudgement: boolean;
        readonly asRequestJudgement: {
            readonly regIndex: Compact<u32>;
            readonly maxFee: Compact<u128>;
        } & Struct;
        readonly isCancelRequest: boolean;
        readonly asCancelRequest: {
            readonly regIndex: u32;
        } & Struct;
        readonly isSetFee: boolean;
        readonly asSetFee: {
            readonly index: Compact<u32>;
            readonly fee: Compact<u128>;
        } & Struct;
        readonly isSetAccountId: boolean;
        readonly asSetAccountId: {
            readonly index: Compact<u32>;
            readonly new_: MultiAddress;
        } & Struct;
        readonly isSetFields: boolean;
        readonly asSetFields: {
            readonly index: Compact<u32>;
            readonly fields: PalletIdentityBitFlags;
        } & Struct;
        readonly isProvideJudgement: boolean;
        readonly asProvideJudgement: {
            readonly regIndex: Compact<u32>;
            readonly target: MultiAddress;
            readonly judgement: PalletIdentityJudgement;
            readonly identity: H256;
        } & Struct;
        readonly isKillIdentity: boolean;
        readonly asKillIdentity: {
            readonly target: MultiAddress;
        } & Struct;
        readonly isAddSub: boolean;
        readonly asAddSub: {
            readonly sub: MultiAddress;
            readonly data: Data;
        } & Struct;
        readonly isRenameSub: boolean;
        readonly asRenameSub: {
            readonly sub: MultiAddress;
            readonly data: Data;
        } & Struct;
        readonly isRemoveSub: boolean;
        readonly asRemoveSub: {
            readonly sub: MultiAddress;
        } & Struct;
        readonly isQuitSub: boolean;
        readonly type:
            | 'AddRegistrar'
            | 'SetIdentity'
            | 'SetSubs'
            | 'ClearIdentity'
            | 'RequestJudgement'
            | 'CancelRequest'
            | 'SetFee'
            | 'SetAccountId'
            | 'SetFields'
            | 'ProvideJudgement'
            | 'KillIdentity'
            | 'AddSub'
            | 'RenameSub'
            | 'RemoveSub'
            | 'QuitSub';
    }

    /** @name PalletIdentityIdentityInfo (278) */
    interface PalletIdentityIdentityInfo extends Struct {
        readonly additional: Vec<ITuple<[Data, Data]>>;
        readonly display: Data;
        readonly legal: Data;
        readonly web: Data;
        readonly riot: Data;
        readonly email: Data;
        readonly pgpFingerprint: Option<U8aFixed>;
        readonly image: Data;
        readonly twitter: Data;
    }

    /** @name PalletIdentityBitFlags (314) */
    interface PalletIdentityBitFlags extends Set {
        readonly isDisplay: boolean;
        readonly isLegal: boolean;
        readonly isWeb: boolean;
        readonly isRiot: boolean;
        readonly isEmail: boolean;
        readonly isPgpFingerprint: boolean;
        readonly isImage: boolean;
        readonly isTwitter: boolean;
    }

    /** @name PalletIdentityIdentityField (315) */
    interface PalletIdentityIdentityField extends Enum {
        readonly isDisplay: boolean;
        readonly isLegal: boolean;
        readonly isWeb: boolean;
        readonly isRiot: boolean;
        readonly isEmail: boolean;
        readonly isPgpFingerprint: boolean;
        readonly isImage: boolean;
        readonly isTwitter: boolean;
        readonly type: 'Display' | 'Legal' | 'Web' | 'Riot' | 'Email' | 'PgpFingerprint' | 'Image' | 'Twitter';
    }

    /** @name PalletIdentityJudgement (316) */
    interface PalletIdentityJudgement extends Enum {
        readonly isUnknown: boolean;
        readonly isFeePaid: boolean;
        readonly asFeePaid: u128;
        readonly isReasonable: boolean;
        readonly isKnownGood: boolean;
        readonly isOutOfDate: boolean;
        readonly isLowQuality: boolean;
        readonly isErroneous: boolean;
        readonly type: 'Unknown' | 'FeePaid' | 'Reasonable' | 'KnownGood' | 'OutOfDate' | 'LowQuality' | 'Erroneous';
    }

    /** @name PalletUtilityCall (317) */
    interface PalletUtilityCall extends Enum {
        readonly isBatch: boolean;
        readonly asBatch: {
            readonly calls: Vec<Call>;
        } & Struct;
        readonly isAsDerivative: boolean;
        readonly asAsDerivative: {
            readonly index: u16;
            readonly call: Call;
        } & Struct;
        readonly isBatchAll: boolean;
        readonly asBatchAll: {
            readonly calls: Vec<Call>;
        } & Struct;
        readonly isDispatchAs: boolean;
        readonly asDispatchAs: {
            readonly asOrigin: CreditcoinNodeRuntimeOriginCaller;
            readonly call: Call;
        } & Struct;
        readonly isForceBatch: boolean;
        readonly asForceBatch: {
            readonly calls: Vec<Call>;
        } & Struct;
        readonly isWithWeight: boolean;
        readonly asWithWeight: {
            readonly call: Call;
            readonly weight: SpWeightsWeightV2Weight;
        } & Struct;
        readonly type: 'Batch' | 'AsDerivative' | 'BatchAll' | 'DispatchAs' | 'ForceBatch' | 'WithWeight';
    }

    /** @name CreditcoinNodeRuntimeOriginCaller (319) */
    interface CreditcoinNodeRuntimeOriginCaller extends Enum {
        readonly isSystem: boolean;
        readonly asSystem: FrameSupportDispatchRawOrigin;
        readonly isVoid: boolean;
        readonly type: 'System' | 'Void';
    }

    /** @name FrameSupportDispatchRawOrigin (320) */
    interface FrameSupportDispatchRawOrigin extends Enum {
        readonly isRoot: boolean;
        readonly isSigned: boolean;
        readonly asSigned: AccountId32;
        readonly isNone: boolean;
        readonly type: 'Root' | 'Signed' | 'None';
    }

    /** @name SpCoreVoid (321) */
    type SpCoreVoid = Null;

    /** @name PalletProxyCall (322) */
    interface PalletProxyCall extends Enum {
        readonly isProxy: boolean;
        readonly asProxy: {
            readonly real: MultiAddress;
            readonly forceProxyType: Option<Null>;
            readonly call: Call;
        } & Struct;
        readonly isAddProxy: boolean;
        readonly asAddProxy: {
            readonly delegate: MultiAddress;
            readonly proxyType: Null;
            readonly delay: u32;
        } & Struct;
        readonly isRemoveProxy: boolean;
        readonly asRemoveProxy: {
            readonly delegate: MultiAddress;
            readonly proxyType: Null;
            readonly delay: u32;
        } & Struct;
        readonly isRemoveProxies: boolean;
        readonly isCreatePure: boolean;
        readonly asCreatePure: {
            readonly proxyType: Null;
            readonly delay: u32;
            readonly index: u16;
        } & Struct;
        readonly isKillPure: boolean;
        readonly asKillPure: {
            readonly spawner: MultiAddress;
            readonly proxyType: Null;
            readonly index: u16;
            readonly height: Compact<u32>;
            readonly extIndex: Compact<u32>;
        } & Struct;
        readonly isAnnounce: boolean;
        readonly asAnnounce: {
            readonly real: MultiAddress;
            readonly callHash: H256;
        } & Struct;
        readonly isRemoveAnnouncement: boolean;
        readonly asRemoveAnnouncement: {
            readonly real: MultiAddress;
            readonly callHash: H256;
        } & Struct;
        readonly isRejectAnnouncement: boolean;
        readonly asRejectAnnouncement: {
            readonly delegate: MultiAddress;
            readonly callHash: H256;
        } & Struct;
        readonly isProxyAnnounced: boolean;
        readonly asProxyAnnounced: {
            readonly delegate: MultiAddress;
            readonly real: MultiAddress;
            readonly forceProxyType: Option<Null>;
            readonly call: Call;
        } & Struct;
        readonly type:
            | 'Proxy'
            | 'AddProxy'
            | 'RemoveProxy'
            | 'RemoveProxies'
            | 'CreatePure'
            | 'KillPure'
            | 'Announce'
            | 'RemoveAnnouncement'
            | 'RejectAnnouncement'
            | 'ProxyAnnounced';
    }

    /** @name PalletFastUnstakeCall (324) */
    interface PalletFastUnstakeCall extends Enum {
        readonly isRegisterFastUnstake: boolean;
        readonly isDeregister: boolean;
        readonly isControl: boolean;
        readonly asControl: {
            readonly erasToCheck: u32;
        } & Struct;
        readonly type: 'RegisterFastUnstake' | 'Deregister' | 'Control';
    }

    /** @name PalletNominationPoolsCall (325) */
    interface PalletNominationPoolsCall extends Enum {
        readonly isJoin: boolean;
        readonly asJoin: {
            readonly amount: Compact<u128>;
            readonly poolId: u32;
        } & Struct;
        readonly isBondExtra: boolean;
        readonly asBondExtra: {
            readonly extra: PalletNominationPoolsBondExtra;
        } & Struct;
        readonly isClaimPayout: boolean;
        readonly isUnbond: boolean;
        readonly asUnbond: {
            readonly memberAccount: MultiAddress;
            readonly unbondingPoints: Compact<u128>;
        } & Struct;
        readonly isPoolWithdrawUnbonded: boolean;
        readonly asPoolWithdrawUnbonded: {
            readonly poolId: u32;
            readonly numSlashingSpans: u32;
        } & Struct;
        readonly isWithdrawUnbonded: boolean;
        readonly asWithdrawUnbonded: {
            readonly memberAccount: MultiAddress;
            readonly numSlashingSpans: u32;
        } & Struct;
        readonly isCreate: boolean;
        readonly asCreate: {
            readonly amount: Compact<u128>;
            readonly root: MultiAddress;
            readonly nominator: MultiAddress;
            readonly bouncer: MultiAddress;
        } & Struct;
        readonly isCreateWithPoolId: boolean;
        readonly asCreateWithPoolId: {
            readonly amount: Compact<u128>;
            readonly root: MultiAddress;
            readonly nominator: MultiAddress;
            readonly bouncer: MultiAddress;
            readonly poolId: u32;
        } & Struct;
        readonly isNominate: boolean;
        readonly asNominate: {
            readonly poolId: u32;
            readonly validators: Vec<AccountId32>;
        } & Struct;
        readonly isSetState: boolean;
        readonly asSetState: {
            readonly poolId: u32;
            readonly state: PalletNominationPoolsPoolState;
        } & Struct;
        readonly isSetMetadata: boolean;
        readonly asSetMetadata: {
            readonly poolId: u32;
            readonly metadata: Bytes;
        } & Struct;
        readonly isSetConfigs: boolean;
        readonly asSetConfigs: {
            readonly minJoinBond: PalletNominationPoolsConfigOpU128;
            readonly minCreateBond: PalletNominationPoolsConfigOpU128;
            readonly maxPools: PalletNominationPoolsConfigOpU32;
            readonly maxMembers: PalletNominationPoolsConfigOpU32;
            readonly maxMembersPerPool: PalletNominationPoolsConfigOpU32;
            readonly globalMaxCommission: PalletNominationPoolsConfigOpPerbill;
        } & Struct;
        readonly isUpdateRoles: boolean;
        readonly asUpdateRoles: {
            readonly poolId: u32;
            readonly newRoot: PalletNominationPoolsConfigOpAccountId32;
            readonly newNominator: PalletNominationPoolsConfigOpAccountId32;
            readonly newBouncer: PalletNominationPoolsConfigOpAccountId32;
        } & Struct;
        readonly isChill: boolean;
        readonly asChill: {
            readonly poolId: u32;
        } & Struct;
        readonly isBondExtraOther: boolean;
        readonly asBondExtraOther: {
            readonly member: MultiAddress;
            readonly extra: PalletNominationPoolsBondExtra;
        } & Struct;
        readonly isSetClaimPermission: boolean;
        readonly asSetClaimPermission: {
            readonly permission: PalletNominationPoolsClaimPermission;
        } & Struct;
        readonly isClaimPayoutOther: boolean;
        readonly asClaimPayoutOther: {
            readonly other: AccountId32;
        } & Struct;
        readonly isSetCommission: boolean;
        readonly asSetCommission: {
            readonly poolId: u32;
            readonly newCommission: Option<ITuple<[Perbill, AccountId32]>>;
        } & Struct;
        readonly isSetCommissionMax: boolean;
        readonly asSetCommissionMax: {
            readonly poolId: u32;
            readonly maxCommission: Perbill;
        } & Struct;
        readonly isSetCommissionChangeRate: boolean;
        readonly asSetCommissionChangeRate: {
            readonly poolId: u32;
            readonly changeRate: PalletNominationPoolsCommissionChangeRate;
        } & Struct;
        readonly isClaimCommission: boolean;
        readonly asClaimCommission: {
            readonly poolId: u32;
        } & Struct;
        readonly type:
            | 'Join'
            | 'BondExtra'
            | 'ClaimPayout'
            | 'Unbond'
            | 'PoolWithdrawUnbonded'
            | 'WithdrawUnbonded'
            | 'Create'
            | 'CreateWithPoolId'
            | 'Nominate'
            | 'SetState'
            | 'SetMetadata'
            | 'SetConfigs'
            | 'UpdateRoles'
            | 'Chill'
            | 'BondExtraOther'
            | 'SetClaimPermission'
            | 'ClaimPayoutOther'
            | 'SetCommission'
            | 'SetCommissionMax'
            | 'SetCommissionChangeRate'
            | 'ClaimCommission';
    }

    /** @name PalletNominationPoolsBondExtra (326) */
    interface PalletNominationPoolsBondExtra extends Enum {
        readonly isFreeBalance: boolean;
        readonly asFreeBalance: u128;
        readonly isRewards: boolean;
        readonly type: 'FreeBalance' | 'Rewards';
    }

    /** @name PalletNominationPoolsConfigOpU128 (327) */
    interface PalletNominationPoolsConfigOpU128 extends Enum {
        readonly isNoop: boolean;
        readonly isSet: boolean;
        readonly asSet: u128;
        readonly isRemove: boolean;
        readonly type: 'Noop' | 'Set' | 'Remove';
    }

    /** @name PalletNominationPoolsConfigOpU32 (328) */
    interface PalletNominationPoolsConfigOpU32 extends Enum {
        readonly isNoop: boolean;
        readonly isSet: boolean;
        readonly asSet: u32;
        readonly isRemove: boolean;
        readonly type: 'Noop' | 'Set' | 'Remove';
    }

    /** @name PalletNominationPoolsConfigOpPerbill (329) */
    interface PalletNominationPoolsConfigOpPerbill extends Enum {
        readonly isNoop: boolean;
        readonly isSet: boolean;
        readonly asSet: Perbill;
        readonly isRemove: boolean;
        readonly type: 'Noop' | 'Set' | 'Remove';
    }

    /** @name PalletNominationPoolsConfigOpAccountId32 (330) */
    interface PalletNominationPoolsConfigOpAccountId32 extends Enum {
        readonly isNoop: boolean;
        readonly isSet: boolean;
        readonly asSet: AccountId32;
        readonly isRemove: boolean;
        readonly type: 'Noop' | 'Set' | 'Remove';
    }

    /** @name PalletNominationPoolsClaimPermission (331) */
    interface PalletNominationPoolsClaimPermission extends Enum {
        readonly isPermissioned: boolean;
        readonly isPermissionlessCompound: boolean;
        readonly isPermissionlessWithdraw: boolean;
        readonly isPermissionlessAll: boolean;
        readonly type: 'Permissioned' | 'PermissionlessCompound' | 'PermissionlessWithdraw' | 'PermissionlessAll';
    }

    /** @name PalletSudoError (332) */
    interface PalletSudoError extends Enum {
        readonly isRequireSudo: boolean;
        readonly type: 'RequireSudo';
    }

    /** @name PalletCreditcoinError (334) */
    interface PalletCreditcoinError extends Enum {
        readonly isAddressAlreadyRegistered: boolean;
        readonly isAddressAlreadyRegisteredByCaller: boolean;
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
        readonly isEthSignExternalAddressGenerationFailed: boolean;
        readonly isEthSignPublicKeyRecoveryFailed: boolean;
        readonly isPersonalSignExternalAddressGenerationFailed: boolean;
        readonly isPersonalSignPublicKeyRecoveryFailed: boolean;
        readonly isUnsupportedBlockchain: boolean;
        readonly isBurnGATEFaucetNotSet: boolean;
        readonly isBurnGATEInsufficientFaucetBalance: boolean;
        readonly type:
            | 'AddressAlreadyRegistered'
            | 'AddressAlreadyRegisteredByCaller'
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
            | 'EthSignExternalAddressGenerationFailed'
            | 'EthSignPublicKeyRecoveryFailed'
            | 'PersonalSignExternalAddressGenerationFailed'
            | 'PersonalSignPublicKeyRecoveryFailed'
            | 'UnsupportedBlockchain'
            | 'BurnGATEFaucetNotSet'
            | 'BurnGATEInsufficientFaucetBalance';
    }

    /** @name PalletDifficultyDifficultyAndTimestamp (336) */
    interface PalletDifficultyDifficultyAndTimestamp extends Struct {
        readonly difficulty: U256;
        readonly timestamp: u64;
    }

    /** @name PalletDifficultyError (339) */
    interface PalletDifficultyError extends Enum {
        readonly isZeroTargetTime: boolean;
        readonly isZeroAdjustmentPeriod: boolean;
        readonly isNegativeAdjustmentPeriod: boolean;
        readonly type: 'ZeroTargetTime' | 'ZeroAdjustmentPeriod' | 'NegativeAdjustmentPeriod';
    }

    /** @name PalletSchedulerScheduled (342) */
    interface PalletSchedulerScheduled extends Struct {
        readonly maybeId: Option<U8aFixed>;
        readonly priority: u8;
        readonly call: FrameSupportPreimagesBounded;
        readonly maybePeriodic: Option<ITuple<[u32, u32]>>;
        readonly origin: CreditcoinNodeRuntimeOriginCaller;
    }

    /** @name FrameSupportPreimagesBounded (343) */
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

    /** @name PalletSchedulerError (346) */
    interface PalletSchedulerError extends Enum {
        readonly isFailedToSchedule: boolean;
        readonly isNotFound: boolean;
        readonly isTargetBlockNumberInPast: boolean;
        readonly isRescheduleNoChange: boolean;
        readonly isNamed: boolean;
        readonly type: 'FailedToSchedule' | 'NotFound' | 'TargetBlockNumberInPast' | 'RescheduleNoChange' | 'Named';
    }

    /** @name PalletCreditcoinTask (347) */
    interface PalletCreditcoinTask extends Enum {
        readonly isVerifyTransfer: boolean;
        readonly asVerifyTransfer: PalletCreditcoinTransferUnverifiedTransfer;
        readonly isCollectCoins: boolean;
        readonly asCollectCoins: PalletCreditcoinCollectCoinsUnverifiedCollectedCoins;
        readonly type: 'VerifyTransfer' | 'CollectCoins';
    }

    /** @name PalletCreditcoinTransferUnverifiedTransfer (348) */
    interface PalletCreditcoinTransferUnverifiedTransfer extends Struct {
        readonly transfer: PalletCreditcoinTransfer;
        readonly fromExternal: Bytes;
        readonly toExternal: Bytes;
        readonly deadline: u32;
    }

    /** @name PalletOffchainTaskSchedulerError (349) */
    interface PalletOffchainTaskSchedulerError extends Enum {
        readonly isOffchainSignedTxFailed: boolean;
        readonly isNoLocalAcctForSignedTx: boolean;
        readonly type: 'OffchainSignedTxFailed' | 'NoLocalAcctForSignedTx';
    }

    /** @name PalletIdentityRegistration (350) */
    interface PalletIdentityRegistration extends Struct {
        readonly judgements: Vec<ITuple<[u32, PalletIdentityJudgement]>>;
        readonly deposit: u128;
        readonly info: PalletIdentityIdentityInfo;
    }

    /** @name PalletIdentityRegistrarInfo (358) */
    interface PalletIdentityRegistrarInfo extends Struct {
        readonly account: AccountId32;
        readonly fee: u128;
        readonly fields: PalletIdentityBitFlags;
    }

    /** @name PalletIdentityError (360) */
    interface PalletIdentityError extends Enum {
        readonly isTooManySubAccounts: boolean;
        readonly isNotFound: boolean;
        readonly isNotNamed: boolean;
        readonly isEmptyIndex: boolean;
        readonly isFeeChanged: boolean;
        readonly isNoIdentity: boolean;
        readonly isStickyJudgement: boolean;
        readonly isJudgementGiven: boolean;
        readonly isInvalidJudgement: boolean;
        readonly isInvalidIndex: boolean;
        readonly isInvalidTarget: boolean;
        readonly isTooManyFields: boolean;
        readonly isTooManyRegistrars: boolean;
        readonly isAlreadyClaimed: boolean;
        readonly isNotSub: boolean;
        readonly isNotOwned: boolean;
        readonly isJudgementForDifferentIdentity: boolean;
        readonly isJudgementPaymentFailed: boolean;
        readonly type:
            | 'TooManySubAccounts'
            | 'NotFound'
            | 'NotNamed'
            | 'EmptyIndex'
            | 'FeeChanged'
            | 'NoIdentity'
            | 'StickyJudgement'
            | 'JudgementGiven'
            | 'InvalidJudgement'
            | 'InvalidIndex'
            | 'InvalidTarget'
            | 'TooManyFields'
            | 'TooManyRegistrars'
            | 'AlreadyClaimed'
            | 'NotSub'
            | 'NotOwned'
            | 'JudgementForDifferentIdentity'
            | 'JudgementPaymentFailed';
    }

    /** @name PalletUtilityError (361) */
    interface PalletUtilityError extends Enum {
        readonly isTooManyCalls: boolean;
        readonly type: 'TooManyCalls';
    }

    /** @name PalletProxyProxyDefinition (364) */
    interface PalletProxyProxyDefinition extends Struct {
        readonly delegate: AccountId32;
        readonly proxyType: Null;
        readonly delay: u32;
    }

    /** @name PalletProxyAnnouncement (368) */
    interface PalletProxyAnnouncement extends Struct {
        readonly real: AccountId32;
        readonly callHash: H256;
        readonly height: u32;
    }

    /** @name PalletProxyError (370) */
    interface PalletProxyError extends Enum {
        readonly isTooMany: boolean;
        readonly isNotFound: boolean;
        readonly isNotProxy: boolean;
        readonly isUnproxyable: boolean;
        readonly isDuplicate: boolean;
        readonly isNoPermission: boolean;
        readonly isUnannounced: boolean;
        readonly isNoSelfProxy: boolean;
        readonly type:
            | 'TooMany'
            | 'NotFound'
            | 'NotProxy'
            | 'Unproxyable'
            | 'Duplicate'
            | 'NoPermission'
            | 'Unannounced'
            | 'NoSelfProxy';
    }

    /** @name PalletFastUnstakeUnstakeRequest (371) */
    interface PalletFastUnstakeUnstakeRequest extends Struct {
        readonly stashes: Vec<ITuple<[AccountId32, u128]>>;
        readonly checked: Vec<u32>;
    }

    /** @name PalletFastUnstakeError (374) */
    interface PalletFastUnstakeError extends Enum {
        readonly isNotController: boolean;
        readonly isAlreadyQueued: boolean;
        readonly isNotFullyBonded: boolean;
        readonly isNotQueued: boolean;
        readonly isAlreadyHead: boolean;
        readonly isCallNotAllowed: boolean;
        readonly type:
            | 'NotController'
            | 'AlreadyQueued'
            | 'NotFullyBonded'
            | 'NotQueued'
            | 'AlreadyHead'
            | 'CallNotAllowed';
    }

    /** @name PalletNominationPoolsPoolMember (375) */
    interface PalletNominationPoolsPoolMember extends Struct {
        readonly poolId: u32;
        readonly points: u128;
        readonly lastRecordedRewardCounter: u128;
        readonly unbondingEras: BTreeMap<u32, u128>;
    }

    /** @name PalletNominationPoolsBondedPoolInner (380) */
    interface PalletNominationPoolsBondedPoolInner extends Struct {
        readonly commission: PalletNominationPoolsCommission;
        readonly memberCounter: u32;
        readonly points: u128;
        readonly roles: PalletNominationPoolsPoolRoles;
        readonly state: PalletNominationPoolsPoolState;
    }

    /** @name PalletNominationPoolsCommission (381) */
    interface PalletNominationPoolsCommission extends Struct {
        readonly current: Option<ITuple<[Perbill, AccountId32]>>;
        readonly max: Option<Perbill>;
        readonly changeRate: Option<PalletNominationPoolsCommissionChangeRate>;
        readonly throttleFrom: Option<u32>;
    }

    /** @name PalletNominationPoolsPoolRoles (384) */
    interface PalletNominationPoolsPoolRoles extends Struct {
        readonly depositor: AccountId32;
        readonly root: Option<AccountId32>;
        readonly nominator: Option<AccountId32>;
        readonly bouncer: Option<AccountId32>;
    }

    /** @name PalletNominationPoolsRewardPool (385) */
    interface PalletNominationPoolsRewardPool extends Struct {
        readonly lastRecordedRewardCounter: u128;
        readonly lastRecordedTotalPayouts: u128;
        readonly totalRewardsClaimed: u128;
        readonly totalCommissionPending: u128;
        readonly totalCommissionClaimed: u128;
    }

    /** @name PalletNominationPoolsSubPools (386) */
    interface PalletNominationPoolsSubPools extends Struct {
        readonly noEra: PalletNominationPoolsUnbondPool;
        readonly withEra: BTreeMap<u32, PalletNominationPoolsUnbondPool>;
    }

    /** @name PalletNominationPoolsUnbondPool (387) */
    interface PalletNominationPoolsUnbondPool extends Struct {
        readonly points: u128;
        readonly balance: u128;
    }

    /** @name FrameSupportPalletId (393) */
    interface FrameSupportPalletId extends U8aFixed {}

    /** @name PalletNominationPoolsError (394) */
    interface PalletNominationPoolsError extends Enum {
        readonly isPoolNotFound: boolean;
        readonly isPoolMemberNotFound: boolean;
        readonly isRewardPoolNotFound: boolean;
        readonly isSubPoolsNotFound: boolean;
        readonly isAccountBelongsToOtherPool: boolean;
        readonly isFullyUnbonding: boolean;
        readonly isMaxUnbondingLimit: boolean;
        readonly isCannotWithdrawAny: boolean;
        readonly isMinimumBondNotMet: boolean;
        readonly isOverflowRisk: boolean;
        readonly isNotDestroying: boolean;
        readonly isNotNominator: boolean;
        readonly isNotKickerOrDestroying: boolean;
        readonly isNotOpen: boolean;
        readonly isMaxPools: boolean;
        readonly isMaxPoolMembers: boolean;
        readonly isCanNotChangeState: boolean;
        readonly isDoesNotHavePermission: boolean;
        readonly isMetadataExceedsMaxLen: boolean;
        readonly isDefensive: boolean;
        readonly asDefensive: PalletNominationPoolsDefensiveError;
        readonly isPartialUnbondNotAllowedPermissionlessly: boolean;
        readonly isMaxCommissionRestricted: boolean;
        readonly isCommissionExceedsMaximum: boolean;
        readonly isCommissionChangeThrottled: boolean;
        readonly isCommissionChangeRateNotAllowed: boolean;
        readonly isNoPendingCommission: boolean;
        readonly isNoCommissionCurrentSet: boolean;
        readonly isPoolIdInUse: boolean;
        readonly isInvalidPoolId: boolean;
        readonly isBondExtraRestricted: boolean;
        readonly type:
            | 'PoolNotFound'
            | 'PoolMemberNotFound'
            | 'RewardPoolNotFound'
            | 'SubPoolsNotFound'
            | 'AccountBelongsToOtherPool'
            | 'FullyUnbonding'
            | 'MaxUnbondingLimit'
            | 'CannotWithdrawAny'
            | 'MinimumBondNotMet'
            | 'OverflowRisk'
            | 'NotDestroying'
            | 'NotNominator'
            | 'NotKickerOrDestroying'
            | 'NotOpen'
            | 'MaxPools'
            | 'MaxPoolMembers'
            | 'CanNotChangeState'
            | 'DoesNotHavePermission'
            | 'MetadataExceedsMaxLen'
            | 'Defensive'
            | 'PartialUnbondNotAllowedPermissionlessly'
            | 'MaxCommissionRestricted'
            | 'CommissionExceedsMaximum'
            | 'CommissionChangeThrottled'
            | 'CommissionChangeRateNotAllowed'
            | 'NoPendingCommission'
            | 'NoCommissionCurrentSet'
            | 'PoolIdInUse'
            | 'InvalidPoolId'
            | 'BondExtraRestricted';
    }

    /** @name PalletNominationPoolsDefensiveError (395) */
    interface PalletNominationPoolsDefensiveError extends Enum {
        readonly isNotEnoughSpaceInUnbondPool: boolean;
        readonly isPoolNotFound: boolean;
        readonly isRewardPoolNotFound: boolean;
        readonly isSubPoolsNotFound: boolean;
        readonly isBondedStashKilledPrematurely: boolean;
        readonly type:
            | 'NotEnoughSpaceInUnbondPool'
            | 'PoolNotFound'
            | 'RewardPoolNotFound'
            | 'SubPoolsNotFound'
            | 'BondedStashKilledPrematurely';
    }

    /** @name FrameSystemExtensionsCheckNonZeroSender (398) */
    type FrameSystemExtensionsCheckNonZeroSender = Null;

    /** @name FrameSystemExtensionsCheckSpecVersion (399) */
    type FrameSystemExtensionsCheckSpecVersion = Null;

    /** @name FrameSystemExtensionsCheckTxVersion (400) */
    type FrameSystemExtensionsCheckTxVersion = Null;

    /** @name FrameSystemExtensionsCheckGenesis (401) */
    type FrameSystemExtensionsCheckGenesis = Null;

    /** @name FrameSystemExtensionsCheckNonce (404) */
    interface FrameSystemExtensionsCheckNonce extends Compact<u32> {}

    /** @name FrameSystemExtensionsCheckWeight (405) */
    type FrameSystemExtensionsCheckWeight = Null;

    /** @name PalletTransactionPaymentChargeTransactionPayment (406) */
    interface PalletTransactionPaymentChargeTransactionPayment extends Compact<u128> {}

    /** @name CreditcoinNodeRuntimeRuntime (407) */
    type CreditcoinNodeRuntimeRuntime = Null;
} // declare module
