// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api-base/types';
import type { Null, Option, Result, u128 } from '@polkadot/types-codec';
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';
import type {
    FrameSupportTokensMiscBalanceStatus,
    FrameSupportWeightsDispatchInfo,
    PalletCreditcoinAddress,
    PalletCreditcoinAskOrder,
    PalletCreditcoinAskOrderId,
    PalletCreditcoinBidOrder,
    PalletCreditcoinBidOrderId,
    PalletCreditcoinDealOrder,
    PalletCreditcoinDealOrderId,
    PalletCreditcoinLegacySighash,
    PalletCreditcoinOffer,
    PalletCreditcoinOfferId,
    PalletCreditcoinTransfer,
    SpRuntimeDispatchError,
} from '@polkadot/types/lookup';

declare module '@polkadot/api-base/types/events' {
    export interface AugmentedEvents<ApiType extends ApiTypes> {
        balances: {
            /**
             * A balance was set by root.
             **/
            BalanceSet: AugmentedEvent<ApiType, [AccountId32, u128, u128]>;
            /**
             * Some amount was deposited (e.g. for transaction fees).
             **/
            Deposit: AugmentedEvent<ApiType, [AccountId32, u128]>;
            /**
             * An account was removed whose balance was non-zero but below ExistentialDeposit,
             * resulting in an outright loss.
             **/
            DustLost: AugmentedEvent<ApiType, [AccountId32, u128]>;
            /**
             * An account was created with some free balance.
             **/
            Endowed: AugmentedEvent<ApiType, [AccountId32, u128]>;
            /**
             * Some balance was reserved (moved from free to reserved).
             **/
            Reserved: AugmentedEvent<ApiType, [AccountId32, u128]>;
            /**
             * Some balance was moved from the reserve of the first account to the second account.
             * Final argument indicates the destination balance type.
             **/
            ReserveRepatriated: AugmentedEvent<
                ApiType,
                [AccountId32, AccountId32, u128, FrameSupportTokensMiscBalanceStatus]
            >;
            /**
             * Some amount was removed from the account (e.g. for misbehavior).
             **/
            Slashed: AugmentedEvent<ApiType, [AccountId32, u128]>;
            /**
             * Transfer succeeded.
             **/
            Transfer: AugmentedEvent<ApiType, [AccountId32, AccountId32, u128]>;
            /**
             * Some balance was unreserved (moved from reserved to free).
             **/
            Unreserved: AugmentedEvent<ApiType, [AccountId32, u128]>;
            /**
             * Some amount was withdrawn from the account (e.g. for transaction fees).
             **/
            Withdraw: AugmentedEvent<ApiType, [AccountId32, u128]>;
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
             * A deal order has been added by a borrower. This indicates that the borrower
             * has accepted a lender's offer and intends to enter the loan.
             * [deal_order_id, deal_order]
             **/
            DealOrderAdded: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
            /**
             * A deal order has been closed by a borrower. This indicates that the borrower
             * has repaid the loan in full and is now closing out the loan.
             * [closed_deal_order_id, closed_deal_order]
             **/
            DealOrderClosed: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
            /**
             * A deal order has been funded by a lender. This indicates that the lender
             * has initiated the actual loan by transferring the loan amount to the borrower
             * on an external chain.
             * [funded_deal_order_id, funded_deal_order]
             **/
            DealOrderFunded: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
            /**
             * A deal order has been locked by a borrower. This indicates that the borrower
             * is preparing to make a repayment and locks the loan from being sold or transferred
             * to another party.
             **/
            DealOrderLocked: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
            /**
             * A legacy wallet from Creditcoin 1.X has been claimed. The balance of the legacy wallet
             * has been transferred to the owner's Creditcoin 2.0 account.
             * [legacy_wallet_claimer, legacy_wallet_sighash, legacy_wallet_balance]
             **/
            LegacyWalletClaimed: AugmentedEvent<ApiType, [AccountId32, PalletCreditcoinLegacySighash, u128]>;
            /**
             * A loan exemption has been granted by a lender. This indicates that the lender
             * is releasing some or all of the outstanding debt on the loan. The borrower
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
            /**
             * An external transfer has been processed and marked as part of a loan.
             * [processed_transfer_id, processed_transfer]
             **/
            TransferProcessed: AugmentedEvent<ApiType, [H256, PalletCreditcoinTransfer]>;
            /**
             * An external transfer has been registered and will be verified.
             * [registered_transfer_id, registered_transfer]
             **/
            TransferRegistered: AugmentedEvent<ApiType, [H256, PalletCreditcoinTransfer]>;
            /**
             * An external transfer has been successfully verified.
             * [verified_transfer_id, verified_transfer]
             **/
            TransferVerified: AugmentedEvent<ApiType, [H256, PalletCreditcoinTransfer]>;
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
        sudo: {
            /**
             * The \[sudoer\] just switched identity; the old key is supplied if one existed.
             **/
            KeyChanged: AugmentedEvent<ApiType, [Option<AccountId32>]>;
            /**
             * A sudo just took place. \[result\]
             **/
            Sudid: AugmentedEvent<ApiType, [Result<Null, SpRuntimeDispatchError>]>;
            /**
             * A sudo just took place. \[result\]
             **/
            SudoAsDone: AugmentedEvent<ApiType, [Result<Null, SpRuntimeDispatchError>]>;
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
            ExtrinsicFailed: AugmentedEvent<ApiType, [SpRuntimeDispatchError, FrameSupportWeightsDispatchInfo]>;
            /**
             * An extrinsic completed successfully.
             **/
            ExtrinsicSuccess: AugmentedEvent<ApiType, [FrameSupportWeightsDispatchInfo]>;
            /**
             * An account was reaped.
             **/
            KilledAccount: AugmentedEvent<ApiType, [AccountId32]>;
            /**
             * A new account was created.
             **/
            NewAccount: AugmentedEvent<ApiType, [AccountId32]>;
            /**
             * On on-chain remark happened.
             **/
            Remarked: AugmentedEvent<ApiType, [AccountId32, H256]>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
    } // AugmentedEvents
} // declare module
