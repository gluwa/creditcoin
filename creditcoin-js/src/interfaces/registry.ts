// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/types/registry';

import type {
    CreditcoinNodeRuntimeOpaqueSessionKeys,
    CreditcoinNodeRuntimeOriginCaller,
    CreditcoinNodeRuntimeRuntime,
    FinalityGrandpaEquivocationPrecommit,
    FinalityGrandpaEquivocationPrevote,
    FinalityGrandpaPrecommit,
    FinalityGrandpaPrevote,
    FrameSupportDispatchDispatchClass,
    FrameSupportDispatchDispatchInfo,
    FrameSupportDispatchPays,
    FrameSupportDispatchPerDispatchClassU32,
    FrameSupportDispatchPerDispatchClassWeight,
    FrameSupportDispatchPerDispatchClassWeightsPerClass,
    FrameSupportDispatchRawOrigin,
    FrameSupportPreimagesBounded,
    FrameSupportTokensMiscBalanceStatus,
    FrameSystemAccountInfo,
    FrameSystemCall,
    FrameSystemError,
    FrameSystemEvent,
    FrameSystemEventRecord,
    FrameSystemExtensionsCheckGenesis,
    FrameSystemExtensionsCheckNonZeroSender,
    FrameSystemExtensionsCheckNonce,
    FrameSystemExtensionsCheckSpecVersion,
    FrameSystemExtensionsCheckTxVersion,
    FrameSystemExtensionsCheckWeight,
    FrameSystemLastRuntimeUpgradeInfo,
    FrameSystemLimitsBlockLength,
    FrameSystemLimitsBlockWeights,
    FrameSystemLimitsWeightsPerClass,
    FrameSystemPhase,
    PalletBabeCall,
    PalletBabeError,
    PalletBagsListCall,
    PalletBagsListError,
    PalletBagsListEvent,
    PalletBagsListListBag,
    PalletBagsListListListError,
    PalletBagsListListNode,
    PalletBalancesAccountData,
    PalletBalancesBalanceLock,
    PalletBalancesCall,
    PalletBalancesError,
    PalletBalancesEvent,
    PalletBalancesReasons,
    PalletBalancesReserveData,
    PalletCreditcoinAddress,
    PalletCreditcoinAskOrder,
    PalletCreditcoinAskOrderId,
    PalletCreditcoinBidOrder,
    PalletCreditcoinBidOrderId,
    PalletCreditcoinBlockchain,
    PalletCreditcoinCall,
    PalletCreditcoinCollectCoinsCollectedCoins,
    PalletCreditcoinCollectCoinsUnverifiedCollectedCoins,
    PalletCreditcoinDealOrder,
    PalletCreditcoinDealOrderId,
    PalletCreditcoinError,
    PalletCreditcoinEvent,
    PalletCreditcoinLegacySighash,
    PalletCreditcoinLoanTerms,
    PalletCreditcoinLoanTermsAskTerms,
    PalletCreditcoinLoanTermsBidTerms,
    PalletCreditcoinLoanTermsDuration,
    PalletCreditcoinLoanTermsInterestRate,
    PalletCreditcoinLoanTermsInterestType,
    PalletCreditcoinOcwErrorsVerificationFailureCause,
    PalletCreditcoinOcwTasksCollectCoinsGCreContract,
    PalletCreditcoinOffer,
    PalletCreditcoinOfferId,
    PalletCreditcoinOrderId,
    PalletCreditcoinRepaymentOrderId,
    PalletCreditcoinTask,
    PalletCreditcoinTaskId,
    PalletCreditcoinTaskOutput,
    PalletCreditcoinTransfer,
    PalletCreditcoinTransferKind,
    PalletCreditcoinTransferUnverifiedTransfer,
    PalletDifficultyCall,
    PalletDifficultyDifficultyAndTimestamp,
    PalletDifficultyError,
    PalletGrandpaCall,
    PalletGrandpaError,
    PalletGrandpaEvent,
    PalletGrandpaStoredPendingChange,
    PalletGrandpaStoredState,
    PalletImOnlineBoundedOpaqueNetworkState,
    PalletImOnlineCall,
    PalletImOnlineError,
    PalletImOnlineEvent,
    PalletImOnlineHeartbeat,
    PalletImOnlineSr25519AppSr25519Public,
    PalletImOnlineSr25519AppSr25519Signature,
    PalletOffchainTaskSchedulerError,
    PalletOffchainTaskSchedulerEvent,
    PalletPosSwitchCall,
    PalletPosSwitchError,
    PalletPosSwitchEvent,
    PalletRewardsEvent,
    PalletSchedulerCall,
    PalletSchedulerError,
    PalletSchedulerEvent,
    PalletSchedulerScheduled,
    PalletSessionCall,
    PalletSessionError,
    PalletSessionEvent,
    PalletStakingActiveEraInfo,
    PalletStakingEraRewardPoints,
    PalletStakingExposure,
    PalletStakingForcing,
    PalletStakingIndividualExposure,
    PalletStakingNominations,
    PalletStakingPalletCall,
    PalletStakingPalletConfigOpPerbill,
    PalletStakingPalletConfigOpPercent,
    PalletStakingPalletConfigOpU128,
    PalletStakingPalletConfigOpU32,
    PalletStakingPalletError,
    PalletStakingPalletEvent,
    PalletStakingRewardDestination,
    PalletStakingSlashingSlashingSpans,
    PalletStakingSlashingSpanRecord,
    PalletStakingStakingLedger,
    PalletStakingUnappliedSlash,
    PalletStakingUnlockChunk,
    PalletStakingValidatorPrefs,
    PalletSudoCall,
    PalletSudoError,
    PalletSudoEvent,
    PalletTimestampCall,
    PalletTransactionPaymentChargeTransactionPayment,
    PalletTransactionPaymentEvent,
    PalletTransactionPaymentReleases,
    SpArithmeticArithmeticError,
    SpConsensusBabeAllowedSlots,
    SpConsensusBabeAppPublic,
    SpConsensusBabeBabeEpochConfiguration,
    SpConsensusBabeDigestsNextConfigDescriptor,
    SpConsensusBabeDigestsPreDigest,
    SpConsensusBabeDigestsPrimaryPreDigest,
    SpConsensusBabeDigestsSecondaryPlainPreDigest,
    SpConsensusBabeDigestsSecondaryVRFPreDigest,
    SpConsensusGrandpaAppPublic,
    SpConsensusGrandpaAppSignature,
    SpConsensusGrandpaEquivocation,
    SpConsensusGrandpaEquivocationProof,
    SpConsensusSlotsEquivocationProof,
    SpCoreCryptoKeyTypeId,
    SpCoreEcdsaPublic,
    SpCoreEcdsaSignature,
    SpCoreEd25519Public,
    SpCoreEd25519Signature,
    SpCoreOffchainOpaqueNetworkState,
    SpCoreSr25519Public,
    SpCoreSr25519Signature,
    SpCoreVoid,
    SpRuntimeBlakeTwo256,
    SpRuntimeDigest,
    SpRuntimeDigestDigestItem,
    SpRuntimeDispatchError,
    SpRuntimeHeader,
    SpRuntimeModuleError,
    SpRuntimeMultiSignature,
    SpRuntimeMultiSigner,
    SpRuntimeTokenError,
    SpRuntimeTransactionalError,
    SpSessionMembershipProof,
    SpVersionRuntimeVersion,
    SpWeightsRuntimeDbWeight,
    SpWeightsWeightV2Weight,
} from '@polkadot/types/lookup';

declare module '@polkadot/types/types/registry' {
    interface InterfaceTypes {
        CreditcoinNodeRuntimeOpaqueSessionKeys: CreditcoinNodeRuntimeOpaqueSessionKeys;
        CreditcoinNodeRuntimeOriginCaller: CreditcoinNodeRuntimeOriginCaller;
        CreditcoinNodeRuntimeRuntime: CreditcoinNodeRuntimeRuntime;
        FinalityGrandpaEquivocationPrecommit: FinalityGrandpaEquivocationPrecommit;
        FinalityGrandpaEquivocationPrevote: FinalityGrandpaEquivocationPrevote;
        FinalityGrandpaPrecommit: FinalityGrandpaPrecommit;
        FinalityGrandpaPrevote: FinalityGrandpaPrevote;
        FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
        FrameSupportDispatchDispatchInfo: FrameSupportDispatchDispatchInfo;
        FrameSupportDispatchPays: FrameSupportDispatchPays;
        FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
        FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
        FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
        FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
        FrameSupportPreimagesBounded: FrameSupportPreimagesBounded;
        FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
        FrameSystemAccountInfo: FrameSystemAccountInfo;
        FrameSystemCall: FrameSystemCall;
        FrameSystemError: FrameSystemError;
        FrameSystemEvent: FrameSystemEvent;
        FrameSystemEventRecord: FrameSystemEventRecord;
        FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
        FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
        FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
        FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
        FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
        FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
        FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
        FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
        FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
        FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
        FrameSystemPhase: FrameSystemPhase;
        PalletBabeCall: PalletBabeCall;
        PalletBabeError: PalletBabeError;
        PalletBagsListCall: PalletBagsListCall;
        PalletBagsListError: PalletBagsListError;
        PalletBagsListEvent: PalletBagsListEvent;
        PalletBagsListListBag: PalletBagsListListBag;
        PalletBagsListListListError: PalletBagsListListListError;
        PalletBagsListListNode: PalletBagsListListNode;
        PalletBalancesAccountData: PalletBalancesAccountData;
        PalletBalancesBalanceLock: PalletBalancesBalanceLock;
        PalletBalancesCall: PalletBalancesCall;
        PalletBalancesError: PalletBalancesError;
        PalletBalancesEvent: PalletBalancesEvent;
        PalletBalancesReasons: PalletBalancesReasons;
        PalletBalancesReserveData: PalletBalancesReserveData;
        PalletCreditcoinAddress: PalletCreditcoinAddress;
        PalletCreditcoinAskOrder: PalletCreditcoinAskOrder;
        PalletCreditcoinAskOrderId: PalletCreditcoinAskOrderId;
        PalletCreditcoinBidOrder: PalletCreditcoinBidOrder;
        PalletCreditcoinBidOrderId: PalletCreditcoinBidOrderId;
        PalletCreditcoinBlockchain: PalletCreditcoinBlockchain;
        PalletCreditcoinCall: PalletCreditcoinCall;
        PalletCreditcoinCollectCoinsCollectedCoins: PalletCreditcoinCollectCoinsCollectedCoins;
        PalletCreditcoinCollectCoinsUnverifiedCollectedCoins: PalletCreditcoinCollectCoinsUnverifiedCollectedCoins;
        PalletCreditcoinDealOrder: PalletCreditcoinDealOrder;
        PalletCreditcoinDealOrderId: PalletCreditcoinDealOrderId;
        PalletCreditcoinError: PalletCreditcoinError;
        PalletCreditcoinEvent: PalletCreditcoinEvent;
        PalletCreditcoinLegacySighash: PalletCreditcoinLegacySighash;
        PalletCreditcoinLoanTerms: PalletCreditcoinLoanTerms;
        PalletCreditcoinLoanTermsAskTerms: PalletCreditcoinLoanTermsAskTerms;
        PalletCreditcoinLoanTermsBidTerms: PalletCreditcoinLoanTermsBidTerms;
        PalletCreditcoinLoanTermsDuration: PalletCreditcoinLoanTermsDuration;
        PalletCreditcoinLoanTermsInterestRate: PalletCreditcoinLoanTermsInterestRate;
        PalletCreditcoinLoanTermsInterestType: PalletCreditcoinLoanTermsInterestType;
        PalletCreditcoinOcwErrorsVerificationFailureCause: PalletCreditcoinOcwErrorsVerificationFailureCause;
        PalletCreditcoinOcwTasksCollectCoinsGCreContract: PalletCreditcoinOcwTasksCollectCoinsGCreContract;
        PalletCreditcoinOffer: PalletCreditcoinOffer;
        PalletCreditcoinOfferId: PalletCreditcoinOfferId;
        PalletCreditcoinOrderId: PalletCreditcoinOrderId;
        PalletCreditcoinRepaymentOrderId: PalletCreditcoinRepaymentOrderId;
        PalletCreditcoinTask: PalletCreditcoinTask;
        PalletCreditcoinTaskId: PalletCreditcoinTaskId;
        PalletCreditcoinTaskOutput: PalletCreditcoinTaskOutput;
        PalletCreditcoinTransfer: PalletCreditcoinTransfer;
        PalletCreditcoinTransferKind: PalletCreditcoinTransferKind;
        PalletCreditcoinTransferUnverifiedTransfer: PalletCreditcoinTransferUnverifiedTransfer;
        PalletDifficultyCall: PalletDifficultyCall;
        PalletDifficultyDifficultyAndTimestamp: PalletDifficultyDifficultyAndTimestamp;
        PalletDifficultyError: PalletDifficultyError;
        PalletGrandpaCall: PalletGrandpaCall;
        PalletGrandpaError: PalletGrandpaError;
        PalletGrandpaEvent: PalletGrandpaEvent;
        PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
        PalletGrandpaStoredState: PalletGrandpaStoredState;
        PalletImOnlineBoundedOpaqueNetworkState: PalletImOnlineBoundedOpaqueNetworkState;
        PalletImOnlineCall: PalletImOnlineCall;
        PalletImOnlineError: PalletImOnlineError;
        PalletImOnlineEvent: PalletImOnlineEvent;
        PalletImOnlineHeartbeat: PalletImOnlineHeartbeat;
        PalletImOnlineSr25519AppSr25519Public: PalletImOnlineSr25519AppSr25519Public;
        PalletImOnlineSr25519AppSr25519Signature: PalletImOnlineSr25519AppSr25519Signature;
        PalletOffchainTaskSchedulerError: PalletOffchainTaskSchedulerError;
        PalletOffchainTaskSchedulerEvent: PalletOffchainTaskSchedulerEvent;
        PalletPosSwitchCall: PalletPosSwitchCall;
        PalletPosSwitchError: PalletPosSwitchError;
        PalletPosSwitchEvent: PalletPosSwitchEvent;
        PalletRewardsEvent: PalletRewardsEvent;
        PalletSchedulerCall: PalletSchedulerCall;
        PalletSchedulerError: PalletSchedulerError;
        PalletSchedulerEvent: PalletSchedulerEvent;
        PalletSchedulerScheduled: PalletSchedulerScheduled;
        PalletSessionCall: PalletSessionCall;
        PalletSessionError: PalletSessionError;
        PalletSessionEvent: PalletSessionEvent;
        PalletStakingActiveEraInfo: PalletStakingActiveEraInfo;
        PalletStakingEraRewardPoints: PalletStakingEraRewardPoints;
        PalletStakingExposure: PalletStakingExposure;
        PalletStakingForcing: PalletStakingForcing;
        PalletStakingIndividualExposure: PalletStakingIndividualExposure;
        PalletStakingNominations: PalletStakingNominations;
        PalletStakingPalletCall: PalletStakingPalletCall;
        PalletStakingPalletConfigOpPerbill: PalletStakingPalletConfigOpPerbill;
        PalletStakingPalletConfigOpPercent: PalletStakingPalletConfigOpPercent;
        PalletStakingPalletConfigOpU128: PalletStakingPalletConfigOpU128;
        PalletStakingPalletConfigOpU32: PalletStakingPalletConfigOpU32;
        PalletStakingPalletError: PalletStakingPalletError;
        PalletStakingPalletEvent: PalletStakingPalletEvent;
        PalletStakingRewardDestination: PalletStakingRewardDestination;
        PalletStakingSlashingSlashingSpans: PalletStakingSlashingSlashingSpans;
        PalletStakingSlashingSpanRecord: PalletStakingSlashingSpanRecord;
        PalletStakingStakingLedger: PalletStakingStakingLedger;
        PalletStakingUnappliedSlash: PalletStakingUnappliedSlash;
        PalletStakingUnlockChunk: PalletStakingUnlockChunk;
        PalletStakingValidatorPrefs: PalletStakingValidatorPrefs;
        PalletSudoCall: PalletSudoCall;
        PalletSudoError: PalletSudoError;
        PalletSudoEvent: PalletSudoEvent;
        PalletTimestampCall: PalletTimestampCall;
        PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
        PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
        PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
        SpArithmeticArithmeticError: SpArithmeticArithmeticError;
        SpConsensusBabeAllowedSlots: SpConsensusBabeAllowedSlots;
        SpConsensusBabeAppPublic: SpConsensusBabeAppPublic;
        SpConsensusBabeBabeEpochConfiguration: SpConsensusBabeBabeEpochConfiguration;
        SpConsensusBabeDigestsNextConfigDescriptor: SpConsensusBabeDigestsNextConfigDescriptor;
        SpConsensusBabeDigestsPreDigest: SpConsensusBabeDigestsPreDigest;
        SpConsensusBabeDigestsPrimaryPreDigest: SpConsensusBabeDigestsPrimaryPreDigest;
        SpConsensusBabeDigestsSecondaryPlainPreDigest: SpConsensusBabeDigestsSecondaryPlainPreDigest;
        SpConsensusBabeDigestsSecondaryVRFPreDigest: SpConsensusBabeDigestsSecondaryVRFPreDigest;
        SpConsensusGrandpaAppPublic: SpConsensusGrandpaAppPublic;
        SpConsensusGrandpaAppSignature: SpConsensusGrandpaAppSignature;
        SpConsensusGrandpaEquivocation: SpConsensusGrandpaEquivocation;
        SpConsensusGrandpaEquivocationProof: SpConsensusGrandpaEquivocationProof;
        SpConsensusSlotsEquivocationProof: SpConsensusSlotsEquivocationProof;
        SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
        SpCoreEcdsaPublic: SpCoreEcdsaPublic;
        SpCoreEcdsaSignature: SpCoreEcdsaSignature;
        SpCoreEd25519Public: SpCoreEd25519Public;
        SpCoreEd25519Signature: SpCoreEd25519Signature;
        SpCoreOffchainOpaqueNetworkState: SpCoreOffchainOpaqueNetworkState;
        SpCoreSr25519Public: SpCoreSr25519Public;
        SpCoreSr25519Signature: SpCoreSr25519Signature;
        SpCoreVoid: SpCoreVoid;
        SpRuntimeBlakeTwo256: SpRuntimeBlakeTwo256;
        SpRuntimeDigest: SpRuntimeDigest;
        SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
        SpRuntimeDispatchError: SpRuntimeDispatchError;
        SpRuntimeHeader: SpRuntimeHeader;
        SpRuntimeModuleError: SpRuntimeModuleError;
        SpRuntimeMultiSignature: SpRuntimeMultiSignature;
        SpRuntimeMultiSigner: SpRuntimeMultiSigner;
        SpRuntimeTokenError: SpRuntimeTokenError;
        SpRuntimeTransactionalError: SpRuntimeTransactionalError;
        SpSessionMembershipProof: SpSessionMembershipProof;
        SpVersionRuntimeVersion: SpVersionRuntimeVersion;
        SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
        SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
    } // InterfaceTypes
} // declare module
