// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/types/registry';

import type {
    CreditcoinNodeRuntimeOriginCaller,
    CreditcoinNodeRuntimeRuntime,
    FrameSupportScheduleLookupError,
    FrameSupportScheduleMaybeHashed,
    FrameSupportTokensMiscBalanceStatus,
    FrameSupportWeightsDispatchClass,
    FrameSupportWeightsDispatchInfo,
    FrameSupportWeightsPays,
    FrameSupportWeightsPerDispatchClassU32,
    FrameSupportWeightsPerDispatchClassU64,
    FrameSupportWeightsPerDispatchClassWeightsPerClass,
    FrameSupportWeightsRuntimeDbWeight,
    FrameSupportWeightsWeightToFeeCoefficient,
    FrameSystemAccountInfo,
    FrameSystemCall,
    FrameSystemError,
    FrameSystemEvent,
    FrameSystemEventRecord,
    FrameSystemExtensionsCheckGenesis,
    FrameSystemExtensionsCheckNonce,
    FrameSystemExtensionsCheckSpecVersion,
    FrameSystemExtensionsCheckTxVersion,
    FrameSystemExtensionsCheckWeight,
    FrameSystemLastRuntimeUpgradeInfo,
    FrameSystemLimitsBlockLength,
    FrameSystemLimitsBlockWeights,
    FrameSystemLimitsWeightsPerClass,
    FrameSystemPhase,
    FrameSystemRawOrigin,
    PalletBalancesAccountData,
    PalletBalancesBalanceLock,
    PalletBalancesCall,
    PalletBalancesError,
    PalletBalancesEvent,
    PalletBalancesReasons,
    PalletBalancesReleases,
    PalletBalancesReserveData,
    PalletChaosCall,
    PalletChaosError,
    PalletChaosEvent,
    PalletCreditcoinAddress,
    PalletCreditcoinAskOrder,
    PalletCreditcoinAskOrderId,
    PalletCreditcoinBidOrder,
    PalletCreditcoinBidOrderId,
    PalletCreditcoinCall,
    PalletCreditcoinCollectedCoins,
    PalletCreditcoinCurrencyOrLegacyTransferKind,
    PalletCreditcoinDealOrder,
    PalletCreditcoinDealOrderId,
    PalletCreditcoinError,
    PalletCreditcoinEvent,
    PalletCreditcoinLegacySighash,
    PalletCreditcoinLegacyTransferKind,
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
    PalletCreditcoinPlatformBlockchain,
    PalletCreditcoinPlatformCurrency,
    PalletCreditcoinPlatformEvmChainId,
    PalletCreditcoinPlatformEvmCurrencyType,
    PalletCreditcoinPlatformEvmInfo,
    PalletCreditcoinPlatformEvmTransferKind,
    PalletCreditcoinPlatformTransferKind,
    PalletCreditcoinTask,
    PalletCreditcoinTaskId,
    PalletCreditcoinTaskOutput,
    PalletCreditcoinTransfer,
    PalletCreditcoinUnverifiedCollectedCoins,
    PalletCreditcoinUnverifiedTransfer,
    PalletDifficultyCall,
    PalletDifficultyDifficultyAndTimestamp,
    PalletDifficultyError,
    PalletRewardsEvent,
    PalletSchedulerCall,
    PalletSchedulerError,
    PalletSchedulerEvent,
    PalletSchedulerReleases,
    PalletSchedulerScheduledV3,
    PalletSudoCall,
    PalletSudoError,
    PalletSudoEvent,
    PalletTimestampCall,
    PalletTransactionPaymentChargeTransactionPayment,
    PalletTransactionPaymentReleases,
    SpCoreEcdsaPublic,
    SpCoreEcdsaSignature,
    SpCoreEd25519Public,
    SpCoreEd25519Signature,
    SpCoreSr25519Public,
    SpCoreSr25519Signature,
    SpCoreVoid,
    SpRuntimeArithmeticError,
    SpRuntimeDigest,
    SpRuntimeDigestDigestItem,
    SpRuntimeDispatchError,
    SpRuntimeMultiSignature,
    SpRuntimeMultiSigner,
    SpRuntimeTokenError,
    SpVersionRuntimeVersion,
} from '@polkadot/types/lookup';

declare module '@polkadot/types/types/registry' {
    interface InterfaceTypes {
        CreditcoinNodeRuntimeOriginCaller: CreditcoinNodeRuntimeOriginCaller;
        CreditcoinNodeRuntimeRuntime: CreditcoinNodeRuntimeRuntime;
        FrameSupportScheduleLookupError: FrameSupportScheduleLookupError;
        FrameSupportScheduleMaybeHashed: FrameSupportScheduleMaybeHashed;
        FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
        FrameSupportWeightsDispatchClass: FrameSupportWeightsDispatchClass;
        FrameSupportWeightsDispatchInfo: FrameSupportWeightsDispatchInfo;
        FrameSupportWeightsPays: FrameSupportWeightsPays;
        FrameSupportWeightsPerDispatchClassU32: FrameSupportWeightsPerDispatchClassU32;
        FrameSupportWeightsPerDispatchClassU64: FrameSupportWeightsPerDispatchClassU64;
        FrameSupportWeightsPerDispatchClassWeightsPerClass: FrameSupportWeightsPerDispatchClassWeightsPerClass;
        FrameSupportWeightsRuntimeDbWeight: FrameSupportWeightsRuntimeDbWeight;
        FrameSupportWeightsWeightToFeeCoefficient: FrameSupportWeightsWeightToFeeCoefficient;
        FrameSystemAccountInfo: FrameSystemAccountInfo;
        FrameSystemCall: FrameSystemCall;
        FrameSystemError: FrameSystemError;
        FrameSystemEvent: FrameSystemEvent;
        FrameSystemEventRecord: FrameSystemEventRecord;
        FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
        FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
        FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
        FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
        FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
        FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
        FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
        FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
        FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
        FrameSystemPhase: FrameSystemPhase;
        FrameSystemRawOrigin: FrameSystemRawOrigin;
        PalletBalancesAccountData: PalletBalancesAccountData;
        PalletBalancesBalanceLock: PalletBalancesBalanceLock;
        PalletBalancesCall: PalletBalancesCall;
        PalletBalancesError: PalletBalancesError;
        PalletBalancesEvent: PalletBalancesEvent;
        PalletBalancesReasons: PalletBalancesReasons;
        PalletBalancesReleases: PalletBalancesReleases;
        PalletBalancesReserveData: PalletBalancesReserveData;
        PalletChaosCall: PalletChaosCall;
        PalletChaosError: PalletChaosError;
        PalletChaosEvent: PalletChaosEvent;
        PalletCreditcoinAddress: PalletCreditcoinAddress;
        PalletCreditcoinAskOrder: PalletCreditcoinAskOrder;
        PalletCreditcoinAskOrderId: PalletCreditcoinAskOrderId;
        PalletCreditcoinBidOrder: PalletCreditcoinBidOrder;
        PalletCreditcoinBidOrderId: PalletCreditcoinBidOrderId;
        PalletCreditcoinCall: PalletCreditcoinCall;
        PalletCreditcoinCollectedCoins: PalletCreditcoinCollectedCoins;
        PalletCreditcoinCurrencyOrLegacyTransferKind: PalletCreditcoinCurrencyOrLegacyTransferKind;
        PalletCreditcoinDealOrder: PalletCreditcoinDealOrder;
        PalletCreditcoinDealOrderId: PalletCreditcoinDealOrderId;
        PalletCreditcoinError: PalletCreditcoinError;
        PalletCreditcoinEvent: PalletCreditcoinEvent;
        PalletCreditcoinLegacySighash: PalletCreditcoinLegacySighash;
        PalletCreditcoinLegacyTransferKind: PalletCreditcoinLegacyTransferKind;
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
        PalletCreditcoinPlatformBlockchain: PalletCreditcoinPlatformBlockchain;
        PalletCreditcoinPlatformCurrency: PalletCreditcoinPlatformCurrency;
        PalletCreditcoinPlatformEvmChainId: PalletCreditcoinPlatformEvmChainId;
        PalletCreditcoinPlatformEvmCurrencyType: PalletCreditcoinPlatformEvmCurrencyType;
        PalletCreditcoinPlatformEvmInfo: PalletCreditcoinPlatformEvmInfo;
        PalletCreditcoinPlatformEvmTransferKind: PalletCreditcoinPlatformEvmTransferKind;
        PalletCreditcoinPlatformTransferKind: PalletCreditcoinPlatformTransferKind;
        PalletCreditcoinTask: PalletCreditcoinTask;
        PalletCreditcoinTaskId: PalletCreditcoinTaskId;
        PalletCreditcoinTaskOutput: PalletCreditcoinTaskOutput;
        PalletCreditcoinTransfer: PalletCreditcoinTransfer;
        PalletCreditcoinUnverifiedCollectedCoins: PalletCreditcoinUnverifiedCollectedCoins;
        PalletCreditcoinUnverifiedTransfer: PalletCreditcoinUnverifiedTransfer;
        PalletDifficultyCall: PalletDifficultyCall;
        PalletDifficultyDifficultyAndTimestamp: PalletDifficultyDifficultyAndTimestamp;
        PalletDifficultyError: PalletDifficultyError;
        PalletRewardsEvent: PalletRewardsEvent;
        PalletSchedulerCall: PalletSchedulerCall;
        PalletSchedulerError: PalletSchedulerError;
        PalletSchedulerEvent: PalletSchedulerEvent;
        PalletSchedulerReleases: PalletSchedulerReleases;
        PalletSchedulerScheduledV3: PalletSchedulerScheduledV3;
        PalletSudoCall: PalletSudoCall;
        PalletSudoError: PalletSudoError;
        PalletSudoEvent: PalletSudoEvent;
        PalletTimestampCall: PalletTimestampCall;
        PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
        PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
        SpCoreEcdsaPublic: SpCoreEcdsaPublic;
        SpCoreEcdsaSignature: SpCoreEcdsaSignature;
        SpCoreEd25519Public: SpCoreEd25519Public;
        SpCoreEd25519Signature: SpCoreEd25519Signature;
        SpCoreSr25519Public: SpCoreSr25519Public;
        SpCoreSr25519Signature: SpCoreSr25519Signature;
        SpCoreVoid: SpCoreVoid;
        SpRuntimeArithmeticError: SpRuntimeArithmeticError;
        SpRuntimeDigest: SpRuntimeDigest;
        SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
        SpRuntimeDispatchError: SpRuntimeDispatchError;
        SpRuntimeMultiSignature: SpRuntimeMultiSignature;
        SpRuntimeMultiSigner: SpRuntimeMultiSigner;
        SpRuntimeTokenError: SpRuntimeTokenError;
        SpVersionRuntimeVersion: SpVersionRuntimeVersion;
    } // InterfaceTypes
} // declare module
