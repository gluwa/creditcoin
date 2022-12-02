// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/errors';

import type { ApiTypes, AugmentedError } from '@polkadot/api-base/types';

export type __AugmentedError<ApiType extends ApiTypes> = AugmentedError<ApiType>;

declare module '@polkadot/api-base/types/errors' {
    interface AugmentedErrors<ApiType extends ApiTypes> {
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
            /**
             * The addresses specified are not on compatible external chains.
             **/
            AddressBlockchainMismatch: AugmentedError<ApiType>;
            /**
             * The address format was not recognized for the given blockchain and external address.
             **/
            AddressFormatNotSupported: AugmentedError<ApiType>;
            /**
             * The account is already an authority.
             **/
            AlreadyAuthority: AugmentedError<ApiType>;
            /**
             * The terms of the ask and bid order do not agree.
             **/
            AskBidMismatch: AugmentedError<ApiType>;
            /**
             * The ask order has expired and is no longer valid.
             **/
            AskOrderExpired: AugmentedError<ApiType>;
            /**
             * The bid order has expired and is no longer valid.
             **/
            BidOrderExpired: AugmentedError<ApiType>;
            /**
             * The coin collection has already been registered.
             **/
            CollectCoinsAlreadyRegistered: AugmentedError<ApiType>;
            /**
             * The currency has already been registered.
             **/
            CurrencyAlreadyRegistered: AugmentedError<ApiType>;
            /**
             * The currency with the given ID has not been registered.
             **/
            CurrencyNotRegistered: AugmentedError<ApiType>;
            /**
             * The deal cannot be locked because it is not funded yet.
             **/
            DealNotFunded: AugmentedError<ApiType>;
            /**
             * The deal order is already closed and cannot be closed again.
             **/
            DealOrderAlreadyClosed: AugmentedError<ApiType>;
            /**
             * The deal order is already funded and cannot be funded again.
             **/
            DealOrderAlreadyFunded: AugmentedError<ApiType>;
            /**
             * The deal order is already locked and cannot be locked again.
             **/
            DealOrderAlreadyLocked: AugmentedError<ApiType>;
            /**
             * The deal order has expired and is no longer valid.
             **/
            DealOrderExpired: AugmentedError<ApiType>;
            /**
             * The deal order must be locked before it can be closed.
             **/
            DealOrderMustBeLocked: AugmentedError<ApiType>;
            /**
             * The legacy/deprecated version of an extrinsic was called, the new version should be used instead.
             **/
            DeprecatedExtrinsic: AugmentedError<ApiType>;
            /**
             * The deal order already exists.
             **/
            DuplicateDealOrder: AugmentedError<ApiType>;
            /**
             * The specified ID has already been used.
             **/
            DuplicateId: AugmentedError<ApiType>;
            /**
             * The offer has already been made.
             **/
            DuplicateOffer: AugmentedError<ApiType>;
            /**
             * The specified guid has already been used and cannot be re-used.
             **/
            GuidAlreadyUsed: AugmentedError<ApiType>;
            /**
             * The node does not have sufficient authority to verify a transfer.
             **/
            InsufficientAuthority: AugmentedError<ApiType>;
            /**
             * The signature does not match the public key and message.
             **/
            InvalidSignature: AugmentedError<ApiType>;
            /**
             * The value of the loan term's term length is zero, which is invalid.
             **/
            InvalidTermLength: AugmentedError<ApiType>;
            /**
             * There is no legacy balance keeper, so no legacy wallets can be claimed.
             * This is a configuration error and should only occur during local development.
             **/
            LegacyBalanceKeeperMissing: AugmentedError<ApiType>;
            /**
             * There is no legacy wallet corresponding to the public key.
             **/
            LegacyWalletNotFound: AugmentedError<ApiType>;
            /**
             * The deal order is malformed and has a block number greater than the
             * tip. This is an internal error.
             **/
            MalformedDealOrder: AugmentedError<ApiType>;
            /**
             * The external address is malformed or otherwise invalid for the platform.
             **/
            MalformedExternalAddress: AugmentedError<ApiType>;
            /**
             * The transfer is malformed and has a block number greater than the
             * tip. This is an internal error.
             **/
            MalformedTransfer: AugmentedError<ApiType>;
            /**
             * The node is an authority but there is no account to create a
             * callback transaction. This is likely an internal error.
             **/
            NoLocalAcctForSignedTx: AugmentedError<ApiType>;
            /**
             * The specified address does not exist.
             **/
            NonExistentAddress: AugmentedError<ApiType>;
            /**
             * The specified ask order does not exist.
             **/
            NonExistentAskOrder: AugmentedError<ApiType>;
            /**
             * The specified bid order does not exist.
             **/
            NonExistentBidOrder: AugmentedError<ApiType>;
            /**
             * The specified deal order does not exist.
             **/
            NonExistentDealOrder: AugmentedError<ApiType>;
            /**
             * The specified offer does not exist.
             **/
            NonExistentOffer: AugmentedError<ApiType>;
            /**
             * The specified transfer does not exist.
             **/
            NonExistentTransfer: AugmentedError<ApiType>;
            /**
             * The address cannot be used because the user does not own it.
             **/
            NotAddressOwner: AugmentedError<ApiType>;
            /**
             * The account you are trying to remove is not  an authority.
             **/
            NotAnAuthority: AugmentedError<ApiType>;
            /**
             * Only the borrower can perform the action.
             **/
            NotBorrower: AugmentedError<ApiType>;
            /**
             * The legacy wallet is not owned by the user.
             **/
            NotLegacyWalletOwner: AugmentedError<ApiType>;
            /**
             * Only the lender can perform the action.
             **/
            NotLender: AugmentedError<ApiType>;
            /**
             * Failed to send an offchain callback transaction. This is likely
             * an internal error.
             **/
            OffchainSignedTxFailed: AugmentedError<ApiType>;
            /**
             * The offer order has expired and is no longer valid.
             **/
            OfferExpired: AugmentedError<ApiType>;
            /**
             * The address retrieved from the proof-of-ownership signature did not match the external address being registered.
             **/
            OwnershipNotSatisfied: AugmentedError<ApiType>;
            RepaymentOrderNonZeroGain: AugmentedError<ApiType>;
            /**
             * Repayment orders are not currently supported.
             **/
            RepaymentOrderUnsupported: AugmentedError<ApiType>;
            /**
             * The bid order is owned by the user, a user cannot lend to themself.
             **/
            SameOwner: AugmentedError<ApiType>;
            /**
             * The account that registered the transfer does
             * not match the account attempting to use the transfer.
             **/
            TransferAccountMismatch: AugmentedError<ApiType>;
            /**
             * The transfer has already been processed and cannot be used.
             **/
            TransferAlreadyProcessed: AugmentedError<ApiType>;
            /**
             * The transfer has already been registered.
             **/
            TransferAlreadyRegistered: AugmentedError<ApiType>;
            /**
             * The transfer amount is less than the amount in the loan terms.
             **/
            TransferAmountInsufficient: AugmentedError<ApiType>;
            /**
             * The amount on the deal order does not match the transfer amount.
             **/
            TransferAmountMismatch: AugmentedError<ApiType>;
            /**
             * The specified deal order ID does not match the transfer deal order ID.
             **/
            TransferDealOrderMismatch: AugmentedError<ApiType>;
            /**
             * The specified transfer type is not currently supported by
             * the blockchain the loan is executed on.
             **/
            UnsupportedTransferKind: AugmentedError<ApiType>;
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
        scheduler: {
            /**
             * Failed to schedule a call
             **/
            FailedToSchedule: AugmentedError<ApiType>;
            /**
             * Attempt to use a non-named function on a named task.
             **/
            Named: AugmentedError<ApiType>;
            /**
             * Cannot find the scheduled call.
             **/
            NotFound: AugmentedError<ApiType>;
            /**
             * Reschedule failed because it does not change scheduled time.
             **/
            RescheduleNoChange: AugmentedError<ApiType>;
            /**
             * Given target block number is in the past.
             **/
            TargetBlockNumberInPast: AugmentedError<ApiType>;
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
