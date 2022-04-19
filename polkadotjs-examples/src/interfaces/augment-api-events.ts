// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from '@polkadot/api-base/types';
import type { Null, Option, Result, u128 } from '@polkadot/types-codec';
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';
import type { FrameSupportTokensMiscBalanceStatus, FrameSupportWeightsDispatchInfo, PalletCreditcoinAddress, PalletCreditcoinAskOrder, PalletCreditcoinAskOrderId, PalletCreditcoinBidOrder, PalletCreditcoinBidOrderId, PalletCreditcoinDealOrder, PalletCreditcoinDealOrderId, PalletCreditcoinLegacySighash, PalletCreditcoinOffer, PalletCreditcoinOfferId, PalletCreditcoinTransfer, SpRuntimeDispatchError } from '@polkadot/types/lookup';

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
      ReserveRepatriated: AugmentedEvent<ApiType, [AccountId32, AccountId32, u128, FrameSupportTokensMiscBalanceStatus]>;
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
       * Event documentation should end with an array that provides descriptive names for event
       * parameters. [something, who]
       **/
      AddressRegistered: AugmentedEvent<ApiType, [H256, PalletCreditcoinAddress]>;
      AskOrderAdded: AugmentedEvent<ApiType, [PalletCreditcoinAskOrderId, PalletCreditcoinAskOrder]>;
      BidOrderAdded: AugmentedEvent<ApiType, [PalletCreditcoinBidOrderId, PalletCreditcoinBidOrder]>;
      DealOrderAdded: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
      DealOrderClosed: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
      DealOrderFunded: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId, PalletCreditcoinDealOrder]>;
      LegacyWalletClaimed: AugmentedEvent<ApiType, [AccountId32, PalletCreditcoinLegacySighash, u128]>;
      LoanExempted: AugmentedEvent<ApiType, [PalletCreditcoinDealOrderId, H256]>;
      OfferAdded: AugmentedEvent<ApiType, [PalletCreditcoinOfferId, PalletCreditcoinOffer]>;
      TransferProcessed: AugmentedEvent<ApiType, [H256, PalletCreditcoinTransfer]>;
      TransferRegistered: AugmentedEvent<ApiType, [H256, PalletCreditcoinTransfer]>;
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
