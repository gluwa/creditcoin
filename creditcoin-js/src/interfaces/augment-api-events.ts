// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/events';

import type { ApiTypes, AugmentedEvent } from '@polkadot/api-base/types';
import type { Bytes, Null, Option, Result, U8aFixed, Vec, bool, u128, u16, u32, u64 } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H256, Perbill } from '@polkadot/types/interfaces/runtime';
import type {
    FrameSupportDispatchDispatchInfo,
    FrameSupportTokensMiscBalanceStatus,
    PalletCreditcoinAddress,
    PalletCreditcoinAskOrder,
    PalletCreditcoinAskOrderId,
    PalletCreditcoinBidOrder,
    PalletCreditcoinBidOrderId,
    PalletCreditcoinCollectCoinsBurnGATE,
    PalletCreditcoinCollectCoinsCollectedCoins,
    PalletCreditcoinCollectCoinsUnverifiedBurnGATE,
    PalletCreditcoinCollectCoinsUnverifiedCollectedCoins,
    PalletCreditcoinDealOrder,
    PalletCreditcoinDealOrderId,
    PalletCreditcoinLegacySighash,
    PalletCreditcoinOcwErrorsVerificationFailureCause,
    PalletCreditcoinOffer,
    PalletCreditcoinOfferId,
    PalletCreditcoinTransfer,
    PalletImOnlineSr25519AppSr25519Public,
    PalletNominationPoolsCommissionChangeRate,
    PalletNominationPoolsPoolState,
    PalletStakingExposure,
    PalletStakingForcing,
    PalletStakingValidatorPrefs,
    SpConsensusGrandpaAppPublic,
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
            BurnedGATEMinted: AugmentedEvent<ApiType, [H256, PalletCreditcoinCollectCoinsBurnGATE]>;
            BurnGATEFailedVerification: AugmentedEvent<
                ApiType,
                [H256, PalletCreditcoinOcwErrorsVerificationFailureCause]
            >;
            BurnGATERegistered: AugmentedEvent<ApiType, [H256, PalletCreditcoinCollectCoinsUnverifiedBurnGATE]>;
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
        fastUnstake: {
            /**
             * A batch was partially checked for the given eras, but the process did not finish.
             **/
            BatchChecked: AugmentedEvent<ApiType, [eras: Vec<u32>], { eras: Vec<u32> }>;
            /**
             * A batch of a given size was terminated.
             *
             * This is always follows by a number of `Unstaked` or `Slashed` events, marking the end
             * of the batch. A new batch will be created upon next block.
             **/
            BatchFinished: AugmentedEvent<ApiType, [size_: u32], { size_: u32 }>;
            /**
             * An internal error happened. Operations will be paused now.
             **/
            InternalError: AugmentedEvent<ApiType, []>;
            /**
             * A staker was slashed for requesting fast-unstake whilst being exposed.
             **/
            Slashed: AugmentedEvent<ApiType, [stash: AccountId32, amount: u128], { stash: AccountId32; amount: u128 }>;
            /**
             * A staker was unstaked.
             **/
            Unstaked: AugmentedEvent<
                ApiType,
                [stash: AccountId32, result: Result<Null, SpRuntimeDispatchError>],
                { stash: AccountId32; result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        grandpa: {
            /**
             * New authority set has been applied.
             **/
            NewAuthorities: AugmentedEvent<
                ApiType,
                [authoritySet: Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>>],
                { authoritySet: Vec<ITuple<[SpConsensusGrandpaAppPublic, u64]>> }
            >;
            /**
             * Current authority set has been paused.
             **/
            Paused: AugmentedEvent<ApiType, []>;
            /**
             * Current authority set has been resumed.
             **/
            Resumed: AugmentedEvent<ApiType, []>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        identity: {
            /**
             * A name was cleared, and the given balance returned.
             **/
            IdentityCleared: AugmentedEvent<
                ApiType,
                [who: AccountId32, deposit: u128],
                { who: AccountId32; deposit: u128 }
            >;
            /**
             * A name was removed and the given balance slashed.
             **/
            IdentityKilled: AugmentedEvent<
                ApiType,
                [who: AccountId32, deposit: u128],
                { who: AccountId32; deposit: u128 }
            >;
            /**
             * A name was set or reset (which will remove all judgements).
             **/
            IdentitySet: AugmentedEvent<ApiType, [who: AccountId32], { who: AccountId32 }>;
            /**
             * A judgement was given by a registrar.
             **/
            JudgementGiven: AugmentedEvent<
                ApiType,
                [target: AccountId32, registrarIndex: u32],
                { target: AccountId32; registrarIndex: u32 }
            >;
            /**
             * A judgement was asked from a registrar.
             **/
            JudgementRequested: AugmentedEvent<
                ApiType,
                [who: AccountId32, registrarIndex: u32],
                { who: AccountId32; registrarIndex: u32 }
            >;
            /**
             * A judgement request was retracted.
             **/
            JudgementUnrequested: AugmentedEvent<
                ApiType,
                [who: AccountId32, registrarIndex: u32],
                { who: AccountId32; registrarIndex: u32 }
            >;
            /**
             * A registrar was added.
             **/
            RegistrarAdded: AugmentedEvent<ApiType, [registrarIndex: u32], { registrarIndex: u32 }>;
            /**
             * A sub-identity was added to an identity and the deposit paid.
             **/
            SubIdentityAdded: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /**
             * A sub-identity was removed from an identity and the deposit freed.
             **/
            SubIdentityRemoved: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /**
             * A sub-identity was cleared, and the given deposit repatriated from the
             * main identity account to the sub-identity account.
             **/
            SubIdentityRevoked: AugmentedEvent<
                ApiType,
                [sub: AccountId32, main: AccountId32, deposit: u128],
                { sub: AccountId32; main: AccountId32; deposit: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        imOnline: {
            /**
             * At the end of the session, no offence was committed.
             **/
            AllGood: AugmentedEvent<ApiType, []>;
            /**
             * A new heartbeat was received from `AuthorityId`.
             **/
            HeartbeatReceived: AugmentedEvent<
                ApiType,
                [authorityId: PalletImOnlineSr25519AppSr25519Public],
                { authorityId: PalletImOnlineSr25519AppSr25519Public }
            >;
            /**
             * At the end of the session, at least one validator was found to be offline.
             **/
            SomeOffline: AugmentedEvent<
                ApiType,
                [offline: Vec<ITuple<[AccountId32, PalletStakingExposure]>>],
                { offline: Vec<ITuple<[AccountId32, PalletStakingExposure]>> }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        nominationPools: {
            /**
             * A member has became bonded in a pool.
             **/
            Bonded: AugmentedEvent<
                ApiType,
                [member: AccountId32, poolId: u32, bonded: u128, joined: bool],
                { member: AccountId32; poolId: u32; bonded: u128; joined: bool }
            >;
            /**
             * A pool has been created.
             **/
            Created: AugmentedEvent<
                ApiType,
                [depositor: AccountId32, poolId: u32],
                { depositor: AccountId32; poolId: u32 }
            >;
            /**
             * A pool has been destroyed.
             **/
            Destroyed: AugmentedEvent<ApiType, [poolId: u32], { poolId: u32 }>;
            /**
             * A member has been removed from a pool.
             *
             * The removal can be voluntary (withdrawn all unbonded funds) or involuntary (kicked).
             **/
            MemberRemoved: AugmentedEvent<
                ApiType,
                [poolId: u32, member: AccountId32],
                { poolId: u32; member: AccountId32 }
            >;
            /**
             * A payout has been made to a member.
             **/
            PaidOut: AugmentedEvent<
                ApiType,
                [member: AccountId32, poolId: u32, payout: u128],
                { member: AccountId32; poolId: u32; payout: u128 }
            >;
            /**
             * A pool's commission `change_rate` has been changed.
             **/
            PoolCommissionChangeRateUpdated: AugmentedEvent<
                ApiType,
                [poolId: u32, changeRate: PalletNominationPoolsCommissionChangeRate],
                { poolId: u32; changeRate: PalletNominationPoolsCommissionChangeRate }
            >;
            /**
             * Pool commission has been claimed.
             **/
            PoolCommissionClaimed: AugmentedEvent<
                ApiType,
                [poolId: u32, commission: u128],
                { poolId: u32; commission: u128 }
            >;
            /**
             * A pool's commission setting has been changed.
             **/
            PoolCommissionUpdated: AugmentedEvent<
                ApiType,
                [poolId: u32, current: Option<ITuple<[Perbill, AccountId32]>>],
                { poolId: u32; current: Option<ITuple<[Perbill, AccountId32]>> }
            >;
            /**
             * A pool's maximum commission setting has been changed.
             **/
            PoolMaxCommissionUpdated: AugmentedEvent<
                ApiType,
                [poolId: u32, maxCommission: Perbill],
                { poolId: u32; maxCommission: Perbill }
            >;
            /**
             * The active balance of pool `pool_id` has been slashed to `balance`.
             **/
            PoolSlashed: AugmentedEvent<ApiType, [poolId: u32, balance: u128], { poolId: u32; balance: u128 }>;
            /**
             * The roles of a pool have been updated to the given new roles. Note that the depositor
             * can never change.
             **/
            RolesUpdated: AugmentedEvent<
                ApiType,
                [root: Option<AccountId32>, bouncer: Option<AccountId32>, nominator: Option<AccountId32>],
                { root: Option<AccountId32>; bouncer: Option<AccountId32>; nominator: Option<AccountId32> }
            >;
            /**
             * The state of a pool has changed
             **/
            StateChanged: AugmentedEvent<
                ApiType,
                [poolId: u32, newState: PalletNominationPoolsPoolState],
                { poolId: u32; newState: PalletNominationPoolsPoolState }
            >;
            /**
             * A member has unbonded from their pool.
             *
             * - `balance` is the corresponding balance of the number of points that has been
             * requested to be unbonded (the argument of the `unbond` transaction) from the bonded
             * pool.
             * - `points` is the number of points that are issued as a result of `balance` being
             * dissolved into the corresponding unbonding pool.
             * - `era` is the era in which the balance will be unbonded.
             * In the absence of slashing, these values will match. In the presence of slashing, the
             * number of points that are issued in the unbonding pool will be less than the amount
             * requested to be unbonded.
             **/
            Unbonded: AugmentedEvent<
                ApiType,
                [member: AccountId32, poolId: u32, balance: u128, points: u128, era: u32],
                { member: AccountId32; poolId: u32; balance: u128; points: u128; era: u32 }
            >;
            /**
             * The unbond pool at `era` of pool `pool_id` has been slashed to `balance`.
             **/
            UnbondingPoolSlashed: AugmentedEvent<
                ApiType,
                [poolId: u32, era: u32, balance: u128],
                { poolId: u32; era: u32; balance: u128 }
            >;
            /**
             * A member has withdrawn from their pool.
             *
             * The given number of `points` have been dissolved in return of `balance`.
             *
             * Similar to `Unbonded` event, in the absence of slashing, the ratio of point to balance
             * will be 1.
             **/
            Withdrawn: AugmentedEvent<
                ApiType,
                [member: AccountId32, poolId: u32, balance: u128, points: u128],
                { member: AccountId32; poolId: u32; balance: u128; points: u128 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        offences: {
            /**
             * There is an offence reported of the given `kind` happened at the `session_index` and
             * (kind-specific) time slot. This event is not deposited for duplicate slashes.
             * \[kind, timeslot\].
             **/
            Offence: AugmentedEvent<ApiType, [kind: U8aFixed, timeslot: Bytes], { kind: U8aFixed; timeslot: Bytes }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        posSwitch: {
            /**
             * Switched to PoS. []
             **/
            Switched: AugmentedEvent<ApiType, []>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        proxy: {
            /**
             * An announcement was placed to make a call in the future.
             **/
            Announced: AugmentedEvent<
                ApiType,
                [real: AccountId32, proxy: AccountId32, callHash: H256],
                { real: AccountId32; proxy: AccountId32; callHash: H256 }
            >;
            /**
             * A proxy was added.
             **/
            ProxyAdded: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, delegatee: AccountId32, proxyType: Null, delay: u32],
                { delegator: AccountId32; delegatee: AccountId32; proxyType: Null; delay: u32 }
            >;
            /**
             * A proxy was executed correctly, with the given.
             **/
            ProxyExecuted: AugmentedEvent<
                ApiType,
                [result: Result<Null, SpRuntimeDispatchError>],
                { result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A proxy was removed.
             **/
            ProxyRemoved: AugmentedEvent<
                ApiType,
                [delegator: AccountId32, delegatee: AccountId32, proxyType: Null, delay: u32],
                { delegator: AccountId32; delegatee: AccountId32; proxyType: Null; delay: u32 }
            >;
            /**
             * A pure account has been created by new proxy with given
             * disambiguation index and proxy type.
             **/
            PureCreated: AugmentedEvent<
                ApiType,
                [pure: AccountId32, who: AccountId32, proxyType: Null, disambiguationIndex: u16],
                { pure: AccountId32; who: AccountId32; proxyType: Null; disambiguationIndex: u16 }
            >;
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
        session: {
            /**
             * New session has happened. Note that the argument is the session index, not the
             * block number as the type might suggest.
             **/
            NewSession: AugmentedEvent<ApiType, [sessionIndex: u32], { sessionIndex: u32 }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        staking: {
            /**
             * An account has bonded this amount. \[stash, amount\]
             *
             * NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,
             * it will not be emitted for staking rewards when they are added to stake.
             **/
            Bonded: AugmentedEvent<ApiType, [stash: AccountId32, amount: u128], { stash: AccountId32; amount: u128 }>;
            /**
             * An account has stopped participating as either a validator or nominator.
             **/
            Chilled: AugmentedEvent<ApiType, [stash: AccountId32], { stash: AccountId32 }>;
            /**
             * The era payout has been set; the first balance is the validator-payout; the second is
             * the remainder from the maximum amount of reward.
             **/
            EraPaid: AugmentedEvent<
                ApiType,
                [eraIndex: u32, validatorPayout: u128, remainder: u128],
                { eraIndex: u32; validatorPayout: u128; remainder: u128 }
            >;
            /**
             * A new force era mode was set.
             **/
            ForceEra: AugmentedEvent<ApiType, [mode: PalletStakingForcing], { mode: PalletStakingForcing }>;
            /**
             * A nominator has been kicked from a validator.
             **/
            Kicked: AugmentedEvent<
                ApiType,
                [nominator: AccountId32, stash: AccountId32],
                { nominator: AccountId32; stash: AccountId32 }
            >;
            /**
             * An old slashing report from a prior era was discarded because it could
             * not be processed.
             **/
            OldSlashingReportDiscarded: AugmentedEvent<ApiType, [sessionIndex: u32], { sessionIndex: u32 }>;
            /**
             * The stakers' rewards are getting paid.
             **/
            PayoutStarted: AugmentedEvent<
                ApiType,
                [eraIndex: u32, validatorStash: AccountId32],
                { eraIndex: u32; validatorStash: AccountId32 }
            >;
            /**
             * The nominator has been rewarded by this amount.
             **/
            Rewarded: AugmentedEvent<ApiType, [stash: AccountId32, amount: u128], { stash: AccountId32; amount: u128 }>;
            /**
             * A staker (validator or nominator) has been slashed by the given amount.
             **/
            Slashed: AugmentedEvent<
                ApiType,
                [staker: AccountId32, amount: u128],
                { staker: AccountId32; amount: u128 }
            >;
            /**
             * A slash for the given validator, for the given percentage of their stake, at the given
             * era as been reported.
             **/
            SlashReported: AugmentedEvent<
                ApiType,
                [validator: AccountId32, fraction: Perbill, slashEra: u32],
                { validator: AccountId32; fraction: Perbill; slashEra: u32 }
            >;
            /**
             * A new set of stakers was elected.
             **/
            StakersElected: AugmentedEvent<ApiType, []>;
            /**
             * The election failed. No new era is planned.
             **/
            StakingElectionFailed: AugmentedEvent<ApiType, []>;
            /**
             * An account has unbonded this amount.
             **/
            Unbonded: AugmentedEvent<ApiType, [stash: AccountId32, amount: u128], { stash: AccountId32; amount: u128 }>;
            /**
             * A validator has set their preferences.
             **/
            ValidatorPrefsSet: AugmentedEvent<
                ApiType,
                [stash: AccountId32, prefs: PalletStakingValidatorPrefs],
                { stash: AccountId32; prefs: PalletStakingValidatorPrefs }
            >;
            /**
             * An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`
             * from the unlocking queue.
             **/
            Withdrawn: AugmentedEvent<
                ApiType,
                [stash: AccountId32, amount: u128],
                { stash: AccountId32; amount: u128 }
            >;
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
        utility: {
            /**
             * Batch of dispatches completed fully with no error.
             **/
            BatchCompleted: AugmentedEvent<ApiType, []>;
            /**
             * Batch of dispatches completed but has errors.
             **/
            BatchCompletedWithErrors: AugmentedEvent<ApiType, []>;
            /**
             * Batch of dispatches did not complete fully. Index of first failing dispatch given, as
             * well as the error.
             **/
            BatchInterrupted: AugmentedEvent<
                ApiType,
                [index: u32, error: SpRuntimeDispatchError],
                { index: u32; error: SpRuntimeDispatchError }
            >;
            /**
             * A call was dispatched.
             **/
            DispatchedAs: AugmentedEvent<
                ApiType,
                [result: Result<Null, SpRuntimeDispatchError>],
                { result: Result<Null, SpRuntimeDispatchError> }
            >;
            /**
             * A single item within a Batch of dispatches has completed with no error.
             **/
            ItemCompleted: AugmentedEvent<ApiType, []>;
            /**
             * A single item within a Batch of dispatches has completed with error.
             **/
            ItemFailed: AugmentedEvent<ApiType, [error: SpRuntimeDispatchError], { error: SpRuntimeDispatchError }>;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
        voterList: {
            /**
             * Moved an account from one bag to another.
             **/
            Rebagged: AugmentedEvent<
                ApiType,
                [who: AccountId32, from: u64, to: u64],
                { who: AccountId32; from: u64; to: u64 }
            >;
            /**
             * Updated the score of some account to the given amount.
             **/
            ScoreUpdated: AugmentedEvent<
                ApiType,
                [who: AccountId32, newScore: u64],
                { who: AccountId32; newScore: u64 }
            >;
            /**
             * Generic event
             **/
            [key: string]: AugmentedEvent<ApiType>;
        };
    } // AugmentedEvents
} // declare module
