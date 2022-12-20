// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/events';

import type { ApiTypes, AugmentedEvent } from '@polkadot/api-base/types';
import type { Null, Option, Result, U8aFixed, u128, u32 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';
import type {
    FrameSupportDispatchDispatchInfo,
    FrameSupportTokensMiscBalanceStatus,
    PalletCreditcoinAddress,
    PalletCreditcoinAskOrder,
    PalletCreditcoinAskOrderId,
    PalletCreditcoinBidOrder,
    PalletCreditcoinBidOrderId,
    PalletCreditcoinCollectCoinsCollectedCoins,
    PalletCreditcoinCollectCoinsUnverifiedCollectedCoins,
    PalletCreditcoinDealOrder,
    PalletCreditcoinDealOrderId,
    PalletCreditcoinLegacySighash,
    PalletCreditcoinOcwErrorsVerificationFailureCause,
    PalletCreditcoinOffer,
    PalletCreditcoinOfferId,
    PalletCreditcoinPlatformCurrency,
    PalletCreditcoinTransfer,
    SpRuntimeDispatchError,
} from '@polkadot/types/lookup';

export type __AugmentedEvent<ApiType extends ApiTypes> = AugmentedEvent<ApiType>;

declare module '@polkadot/api-base/types/events' {
    interface AugmentedEvents<ApiType extends ApiTypes> {
        balances: {
            /**
             * A balance was set by root.
             **/
            BalanceSet: AugmentedEvent<
                ApiType,
                [who: AccountId32, free: u128, reserved: u128],
                { who: AccountId32; free: u128; reserved: u128 }
            >;
            /**
             * Some amount was deposited (e.g. for transaction fees).
             **/
            Deposit: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * An account was removed whose balance was non-zero but below ExistentialDeposit,
             * resulting in an outright loss.
             **/
            DustLost: AugmentedEvent<
                ApiType,
                [account: AccountId32, amount: u128],
                { account: AccountId32; amount: u128 }
            >;
            /**
             * An account was created with some free balance.
             **/
            Endowed: AugmentedEvent<
                ApiType,
                [account: AccountId32, freeBalance: u128],
                { account: AccountId32; freeBalance: u128 }
            >;
            /**
             * Some balance was reserved (moved from free to reserved).
             **/
            Reserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some balance was moved from the reserve of the first account to the second account.
             * Final argument indicates the destination balance type.
             **/
            ReserveRepatriated: AugmentedEvent<
                ApiType,
                [
                    from: AccountId32,
                    to: AccountId32,
                    amount: u128,
                    destinationStatus: FrameSupportTokensMiscBalanceStatus,
                ],
                {
                    from: AccountId32;
                    to: AccountId32;
                    amount: u128;
                    destinationStatus: FrameSupportTokensMiscBalanceStatus;
                }
            >;
            /**
             * Some amount was removed from the account (e.g. for misbehavior).
             **/
            Slashed: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Transfer succeeded.
             **/
            Transfer: AugmentedEvent<
                ApiType,
                [from: AccountId32, to: AccountId32, amount: u128],
                { from: AccountId32; to: AccountId32; amount: u128 }
            >;
            /**
             * Some balance was unreserved (moved from reserved to free).
             **/
            Unreserved: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Some amount was withdrawn from the account (e.g. for transaction fees).
             **/
            Withdraw: AugmentedEvent<ApiType, [who: AccountId32, amount: u128], { who: AccountId32; amount: u128 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        creditcoin: {
            /**
             * An address on an external chain has been registered.
             * [registered_address_id, registered_address]
             **/
            AddressRegistered: AugmentedEvent<ApiType, [H256, PalletCreditcoinAddress]>;
            /**
             * An ask order has been added by a prospective lender. This indicates that the lender
             * is looking to issue a loan with certain terms.
             * [ask_order_id, ask_order]
             **/
            AskOrderAdded: AugmentedEvent<ApiType, [PalletCreditcoinAskOrderId, PalletCreditcoinAskOrder]>;
            /**
             * A bid order has been added by a prospective borrower. This indicates that the borrower
             * is looking for a loan with certain terms.
             * [bid_order_id, bid_order]
             **/
            BidOrderAdded: AugmentedEvent<ApiType, [PalletCreditcoinBidOrderId, PalletCreditcoinBidOrder]>;
            /**
             * exchanging vested ERC-20 CC for native CC failed.
             * [collected_coins_id, cause]
             **/
            CollectCoinsFailedVerification: AugmentedEvent<
                ApiType,
                [H256, PalletCreditcoinOcwErrorsVerificationFailureCause]
            >;
            /**
             * Collecting coins from Eth ERC-20 has been registered and will be verified.
             * [collected_coins_id, registered_collect_coins]
             **/
            CollectCoinsRegistered: AugmentedEvent<
                ApiType,
                [H256, PalletCreditcoinCollectCoinsUnverifiedCollectedCoins]
            >;
            /**
             * CollectCoins has been successfully verified and minted.
             * [collected_coins_id, collected_coins]
             **/
            CollectedCoinsMinted: AugmentedEvent<ApiType, [H256, PalletCreditcoinCollectCoinsCollectedCoins]>;
            /**
             * A currency has been registered and can now be used in loan terms.
             * [currency_id, currency]
             **/
            CurrencyRegistered: AugmentedEvent<ApiType, [H256, PalletCreditcoinPlatformCurrency]>;
            /**
             * A deal order has been added by a borrower. This indicates that the borrower
             * has accepted a lender's offer and intends to enter the loan.
             * [deal_order_id, deal_order]
             **/
            DealOrderAdded: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
            /**
             * A deal order has been closed by a borrower. This indicates that the borrower
             * has repaid the loan in full and is now closing out the loan.
             * [closed_deal_order_id]
             **/
            DealOrderClosed: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId]>;
            /**
             * A deal order has been funded by a lender. This indicates that the lender
             * has initiated the actual loan by transferring the loan amount to the borrower
             * on an external chain.
             * [funded_deal_order_id]
             **/
            DealOrderFunded: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId]>;
            /**
             * A deal order has been locked by a borrower. This indicates that the borrower
             * is preparing to make a repayment and locks the loan from being sold or transferred
             * to another party.
             * [deal_order_id]
             **/
            DealOrderLocked: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId]>;
            /**
             * A legacy wallet from Creditcoin 1.X has been claimed. The balance of the legacy wallet
             * has been transferred to the owner's Creditcoin 2.0 account.
             * [legacy_wallet_claimer, legacy_wallet_sighash, legacy_wallet_balance]
             **/
            LegacyWalletClaimed: AugmentedEvent<ApiType, [AccountId32, PalletCreditcoinLegacySighash, u128]>;
            /**
             * A loan exemption has been granted by a lender. This indicates that the lender
             * is releasing all of the outstanding debt on the loan. The borrower
             * is no longer responsible for repaying the amount.
             * [exempted_deal_order_id]
             **/
            LoanExempted: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId]>;
            /**
             * An offer has been added by a lender. This indicates that the lender
             * is interested in entering a loan with the owner of the bid order.
             * [offer_id, offer]
             **/
            OfferAdded: AugmentedEvent<ApiType, [PalletCreditcoinOfferId, PalletCreditcoinOffer]>;
            TransferFailedVerification: AugmentedEvent<
                ApiType,
                [H256, PalletCreditcoinOcwErrorsVerificationFailureCause]
            >;
            /**
             * An external transfer has been processed and marked as part of a loan.
             * [processed_transfer_id]
             **/
            TransferProcessed: AugmentedEvent<ApiType, [H256]>;
            /**
             * An external transfer has been registered and will be verified.
             * [registered_transfer_id, registered_transfer]
             **/
            TransferRegistered: AugmentedEvent<ApiType, [H256, PalletCreditcoinTransfer]>;
            /**
             * An external transfer has been successfully verified.
             * [verified_transfer_id]
             **/
            TransferVerified: AugmentedEvent<ApiType, [H256]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        rewards: {
            /**
             * Reward was issued. [block_author, amount]
             **/
            RewardIssued: AugmentedEvent<ApiType, [AccountId32, u128]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        scheduler: {
            /**
             * The call for the provided hash was not found so the task has been aborted.
             **/
            CallUnavailable: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed> }
            >;
            /**
             * Canceled some task.
             **/
            Canceled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32; index: u32 }>;
            /**
             * Dispatched some task.
             **/
            Dispatched: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>, result: Result<Null, SpRuntimeDispatchError>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed>; result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * The given task was unable to be renewed since the agenda is full at that block.
             **/
            PeriodicFailed: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed> }
            >;
            /**
             * The given task can never be executed since it is overweight.
             **/
            PermanentlyOverweight: AugmentedEvent<
                ApiType,
                [task: ITuple<[u32, u32]>, id: Option<U8aFixed>],
                { task: ITuple<[u32, u32]>; id: Option<U8aFixed> }
            >;
            /**
             * Scheduled some task.
             **/
            Scheduled: AugmentedEvent<ApiType, [when: u32, index: u32], { when: u32; index: u32 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        sudo: {
            /**
             * The \[sudoer\] just switched identity; the old key is supplied if one existed.
             **/
            KeyChanged: AugmentedEvent<ApiType, [oldSudoer: Option<AccountId32>], { oldSudoer: Option<AccountId32> }>;
            /**
             * A sudo just took place. \[result\]
             **/
            Sudid: AugmentedEvent<
                ApiType,
                [sudoResult: Result<Null, SpRuntimeDispatchError>],
                { sudoResult: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A sudo just took place. \[result\]
             **/
            SudoAsDone: AugmentedEvent<
                ApiType,
                [sudoResult: Result<Null, SpRuntimeDispatchError>],
                { sudoResult: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        system: {
            /**
             * `:code` was updated.
             **/
            CodeUpdated: AugmentedEvent<ApiType, []>;
            /**
             * An extrinsic failed.
             **/
            ExtrinsicFailed: AugmentedEvent<
                ApiType,
                [dispatchError: SpRuntimeDispatchError, dispatchInfo: FrameSupportDispatchDispatchInfo],
                { dispatchError: SpRuntimeDispatchError; dispatchInfo: FrameSupportDispatchDispatchInfo }
            >;
            /**
             * An extrinsic completed successfully.
             **/
            ExtrinsicSuccess: AugmentedEvent<
                ApiType,
                [dispatchInfo: FrameSupportDispatchDispatchInfo],
                { dispatchInfo: FrameSupportDispatchDispatchInfo }
            >;
            /**
             * An account was reaped.
             **/
            KilledAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            /**
             * A new account was created.
             **/
            NewAccount: AugmentedEvent<ApiType, [account: AccountId32], { account: AccountId32 }>;
            /**
             * On on-chain remark happened.
             **/
            Remarked: AugmentedEvent<ApiType, [sender: AccountId32, hash_: H256], { sender: AccountId32; hash_: H256 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        taskScheduler: {
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        transactionPayment: {
            /**
             * A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,
             * has been paid by `who`.
             **/
            TransactionFeePaid: AugmentedEvent<
                ApiType,
                [who: AccountId32, actualFee: u128, tip: u128],
                { who: AccountId32; actualFee: u128; tip: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
    } // AugmentedEvents
} // declare module
