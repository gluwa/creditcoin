// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type {
    CreditcoinNodeRuntimeRuntime,
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
    PalletCreditcoinBlockchain,
    PalletCreditcoinCall,
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
    PalletCreditcoinOffer,
    PalletCreditcoinOfferId,
    PalletCreditcoinOrderId,
    PalletCreditcoinRepaymentOrderId,
    PalletCreditcoinTransfer,
    PalletCreditcoinTransferKind,
    PalletCreditcoinUnverifiedTransfer,
    PalletDifficultyCall,
    PalletDifficultyDifficultyAndTimestamp,
    PalletDifficultyError,
    PalletRewardsEvent,
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
    export interface InterfaceTypes {
        CreditcoinNodeRuntimeRuntime: CreditcoinNodeRuntimeRuntime;
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
        PalletCreditcoinBlockchain: PalletCreditcoinBlockchain;
        PalletCreditcoinCall: PalletCreditcoinCall;
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
        PalletCreditcoinOffer: PalletCreditcoinOffer;
        PalletCreditcoinOfferId: PalletCreditcoinOfferId;
        PalletCreditcoinOrderId: PalletCreditcoinOrderId;
        PalletCreditcoinRepaymentOrderId: PalletCreditcoinRepaymentOrderId;
        PalletCreditcoinTransfer: PalletCreditcoinTransfer;
        PalletCreditcoinTransferKind: PalletCreditcoinTransferKind;
        PalletCreditcoinUnverifiedTransfer: PalletCreditcoinUnverifiedTransfer;
        PalletDifficultyCall: PalletDifficultyCall;
        PalletDifficultyDifficultyAndTimestamp: PalletDifficultyDifficultyAndTimestamp;
        PalletDifficultyError: PalletDifficultyError;
        PalletRewardsEvent: PalletRewardsEvent;
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
