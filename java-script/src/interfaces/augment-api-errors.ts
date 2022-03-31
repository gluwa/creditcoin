// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api-base/types';

declare module '@polkadot/api-base/types/errors' {
  export interface AugmentedErrors<ApiType extends ApiTypes> {
    balances: {
      /**
       * Beneficiary account must pre-exist
       **/
      DeadAccount: AugmentedError<ApiType>;
      /**
       * Value too low to create account due to existential deposit
       **/
      ExistentialDeposit: AugmentedError<ApiType>;
      /**
       * A vesting schedule already exists for this account
       **/
      ExistingVestingSchedule: AugmentedError<ApiType>;
      /**
       * Balance too low to send value
       **/
      InsufficientBalance: AugmentedError<ApiType>;
      /**
       * Transfer/payment would kill account
       **/
      KeepAlive: AugmentedError<ApiType>;
      /**
       * Account liquidity restrictions prevent withdrawal
       **/
      LiquidityRestrictions: AugmentedError<ApiType>;
      /**
       * Number of named reserves exceed MaxReserves
       **/
      TooManyReserves: AugmentedError<ApiType>;
      /**
       * Vesting balance too high to send value
       **/
      VestingBalance: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    creditcoin: {
      /**
       * The specified address has already been registered to another account
       **/
      AddressAlreadyRegistered: AugmentedError<ApiType>;
      AddressPlatformMismatch: AugmentedError<ApiType>;
      AlreadyAuthority: AugmentedError<ApiType>;
      AskBidMismatch: AugmentedError<ApiType>;
      AskOrderExpired: AugmentedError<ApiType>;
      BidOrderExpired: AugmentedError<ApiType>;
      DealNotFunded: AugmentedError<ApiType>;
      DealOrderAlreadyClosed: AugmentedError<ApiType>;
      DealOrderAlreadyFunded: AugmentedError<ApiType>;
      DealOrderAlreadyLocked: AugmentedError<ApiType>;
      DealOrderExpired: AugmentedError<ApiType>;
      DealOrderMustBeLocked: AugmentedError<ApiType>;
      DuplicateDealOrder: AugmentedError<ApiType>;
      DuplicateId: AugmentedError<ApiType>;
      DuplicateOffer: AugmentedError<ApiType>;
      GuidAlreadyUsed: AugmentedError<ApiType>;
      InsufficientAuthority: AugmentedError<ApiType>;
      InvalidMaturity: AugmentedError<ApiType>;
      InvalidSignature: AugmentedError<ApiType>;
      LegacyBalanceKeeperMissing: AugmentedError<ApiType>;
      LegacySighashMalformed: AugmentedError<ApiType>;
      LegacyWalletNotFound: AugmentedError<ApiType>;
      MalformedDealOrder: AugmentedError<ApiType>;
      MalformedTransfer: AugmentedError<ApiType>;
      NoLocalAcctForSignedTx: AugmentedError<ApiType>;
      NonExistentAddress: AugmentedError<ApiType>;
      NonExistentAskOrder: AugmentedError<ApiType>;
      NonExistentBidOrder: AugmentedError<ApiType>;
      NonExistentDealOrder: AugmentedError<ApiType>;
      NonExistentOffer: AugmentedError<ApiType>;
      NonExistentRepaymentOrder: AugmentedError<ApiType>;
      NonExistentTransfer: AugmentedError<ApiType>;
      NotAddressOwner: AugmentedError<ApiType>;
      NotBorrower: AugmentedError<ApiType>;
      NotLegacyWalletOwner: AugmentedError<ApiType>;
      NotLender: AugmentedError<ApiType>;
      OffchainSignedTxFailed: AugmentedError<ApiType>;
      OfferExpired: AugmentedError<ApiType>;
      RepaymentOrderNonZeroGain: AugmentedError<ApiType>;
      RepaymentOrderUnsupported: AugmentedError<ApiType>;
      SameOwner: AugmentedError<ApiType>;
      ScaleDecodeError: AugmentedError<ApiType>;
      TransferAlreadyProcessed: AugmentedError<ApiType>;
      TransferAlreadyRegistered: AugmentedError<ApiType>;
      TransferAmountInsufficient: AugmentedError<ApiType>;
      TransferMismatch: AugmentedError<ApiType>;
      UnsupportedTransferKind: AugmentedError<ApiType>;
      UnverifiedTransferPoolFull: AugmentedError<ApiType>;
      VerifyStringTooLong: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    difficulty: {
      NegativeAdjustmentPeriod: AugmentedError<ApiType>;
      ZeroAdjustmentPeriod: AugmentedError<ApiType>;
      ZeroTargetTime: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    sudo: {
      /**
       * Sender must be the Sudo account
       **/
      RequireSudo: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
    system: {
      /**
       * The origin filter prevent the call to be dispatched.
       **/
      CallFiltered: AugmentedError<ApiType>;
      /**
       * Failed to extract the runtime version from the new runtime.
       * 
       * Either calling `Core_version` or decoding `RuntimeVersion` failed.
       **/
      FailedToExtractRuntimeVersion: AugmentedError<ApiType>;
      /**
       * The name of specification does not match between the current runtime
       * and the new runtime.
       **/
      InvalidSpecName: AugmentedError<ApiType>;
      /**
       * Suicide called when the account has non-default composite data.
       **/
      NonDefaultComposite: AugmentedError<ApiType>;
      /**
       * There is a non-zero reference count preventing the account from being purged.
       **/
      NonZeroRefCount: AugmentedError<ApiType>;
      /**
       * The specification version is not allowed to decrease between the current runtime
       * and the new runtime.
       **/
      SpecVersionNeedsToIncrease: AugmentedError<ApiType>;
      /**
       * Generic error
       **/
      [key: string]: AugmentedError<ApiType>;
    };
  } // AugmentedErrors
} // declare module
