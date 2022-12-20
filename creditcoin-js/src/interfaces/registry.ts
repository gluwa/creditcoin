// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/types/types/registry';

import type {
    CreditcoinNodeRuntimeOriginCaller,
    CreditcoinNodeRuntimeRuntime,
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
    PalletBalancesAccountData,
    PalletBalancesBalanceLock,
    PalletBalancesCall,
    PalletBalancesError,
    PalletBalancesEvent,
    PalletBalancesReasons,
    PalletBalancesReleases,
    PalletBalancesReserveData,
    PalletCreditcoinAddress,
    PalletCreditcoinAskOrder,
    PalletCreditcoinAskOrderId,
    PalletCreditcoinBidOrder,
    PalletCreditcoinBidOrderId,
    PalletCreditcoinCall,
    PalletCreditcoinCollectCoinsCollectedCoins,
    PalletCreditcoinCollectCoinsUnverifiedCollectedCoins,
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
    PalletCreditcoinTransferUnverifiedTransfer,
    PalletDifficultyCall,
    PalletDifficultyDifficultyAndTimestamp,
    PalletDifficultyError,
    PalletOffchainTaskSchedulerError,
    PalletOffchainTaskSchedulerEvent,
    PalletRewardsEvent,
    PalletSchedulerCall,
    PalletSchedulerError,
    PalletSchedulerEvent,
    PalletSchedulerScheduled,
    PalletSudoCall,
    PalletSudoError,
    PalletSudoEvent,
    PalletTimestampCall,
    PalletTransactionPaymentChargeTransactionPayment,
    PalletTransactionPaymentEvent,
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
    SpRuntimeModuleError,
    SpRuntimeMultiSignature,
    SpRuntimeMultiSigner,
    SpRuntimeTokenError,
    SpRuntimeTransactionalError,
    SpVersionRuntimeVersion,
    SpWeightsRuntimeDbWeight,
    SpWeightsWeightV2Weight,
} from '@polkadot/types/lookup';

declare module '@polkadot/types/types/registry' {
    interface InterfaceTypes {
        CreditcoinNodeRuntimeOriginCaller: CreditcoinNodeRuntimeOriginCaller;
        CreditcoinNodeRuntimeRuntime: CreditcoinNodeRuntimeRuntime;
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
        PalletBalancesAccountData: PalletBalancesAccountData;
        PalletBalancesBalanceLock: PalletBalancesBalanceLock;
        PalletBalancesCall: PalletBalancesCall;
        PalletBalancesError: PalletBalancesError;
        PalletBalancesEvent: PalletBalancesEvent;
        PalletBalancesReasons: PalletBalancesReasons;
        PalletBalancesReleases: PalletBalancesReleases;
        PalletBalancesReserveData: PalletBalancesReserveData;
        PalletCreditcoinAddress: PalletCreditcoinAddress;
        PalletCreditcoinAskOrder: PalletCreditcoinAskOrder;
        PalletCreditcoinAskOrderId: PalletCreditcoinAskOrderId;
        PalletCreditcoinBidOrder: PalletCreditcoinBidOrder;
        PalletCreditcoinBidOrderId: PalletCreditcoinBidOrderId;
        PalletCreditcoinCall: PalletCreditcoinCall;
        PalletCreditcoinCollectCoinsCollectedCoins: PalletCreditcoinCollectCoinsCollectedCoins;
        PalletCreditcoinCollectCoinsUnverifiedCollectedCoins: PalletCreditcoinCollectCoinsUnverifiedCollectedCoins;
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
        PalletCreditcoinTransferUnverifiedTransfer: PalletCreditcoinTransferUnverifiedTransfer;
        PalletDifficultyCall: PalletDifficultyCall;
        PalletDifficultyDifficultyAndTimestamp: PalletDifficultyDifficultyAndTimestamp;
        PalletDifficultyError: PalletDifficultyError;
        PalletOffchainTaskSchedulerError: PalletOffchainTaskSchedulerError;
        PalletOffchainTaskSchedulerEvent: PalletOffchainTaskSchedulerEvent;
        PalletRewardsEvent: PalletRewardsEvent;
        PalletSchedulerCall: PalletSchedulerCall;
        PalletSchedulerError: PalletSchedulerError;
        PalletSchedulerEvent: PalletSchedulerEvent;
        PalletSchedulerScheduled: PalletSchedulerScheduled;
        PalletSudoCall: PalletSudoCall;
        PalletSudoError: PalletSudoError;
        PalletSudoEvent: PalletSudoEvent;
        PalletTimestampCall: PalletTimestampCall;
        PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
        PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
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
        SpRuntimeModuleError: SpRuntimeModuleError;
        SpRuntimeMultiSignature: SpRuntimeMultiSignature;
        SpRuntimeMultiSigner: SpRuntimeMultiSigner;
        SpRuntimeTokenError: SpRuntimeTokenError;
        SpRuntimeTransactionalError: SpRuntimeTransactionalError;
        SpVersionRuntimeVersion: SpVersionRuntimeVersion;
        SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
        SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
    } // InterfaceTypes
} // declare module
