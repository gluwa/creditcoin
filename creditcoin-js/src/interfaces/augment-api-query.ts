// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/storage';

import type { ApiTypes, AugmentedQuery, QueryableStorageEntry } from '@polkadot/api-base/types';
import type { Bytes, Null, Option, U256, Vec, bool, i64, u128, u32, u64 } from '@polkadot/types-codec';
import type { AnyNumber, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';
import type {
    FrameSupportWeightsPerDispatchClassU64,
    FrameSystemAccountInfo,
    FrameSystemEventRecord,
    FrameSystemLastRuntimeUpgradeInfo,
    FrameSystemPhase,
    PalletBalancesAccountData,
    PalletBalancesBalanceLock,
    PalletBalancesReleases,
    PalletBalancesReserveData,
    PalletCreditcoinAddress,
    PalletCreditcoinAskOrder,
    PalletCreditcoinBidOrder,
    PalletCreditcoinCollectCoinsCollectedCoins,
    PalletCreditcoinDealOrder,
    PalletCreditcoinLegacySighash,
    PalletCreditcoinOcwTasksCollectCoinsGCreContract,
    PalletCreditcoinOffer,
    PalletCreditcoinPlatformCurrency,
    PalletCreditcoinTask,
    PalletCreditcoinTaskId,
    PalletCreditcoinTransfer,
    PalletDifficultyDifficultyAndTimestamp,
    PalletSchedulerReleases,
    PalletSchedulerScheduledV3,
    PalletTransactionPaymentReleases,
    SpRuntimeDigest,
} from '@polkadot/types/lookup';
import type { Observable } from '@polkadot/types/types';

export type __AugmentedQuery<ApiType extends ApiTypes> = AugmentedQuery<ApiType, () => unknown>;
export type __QueryableStorageEntry<ApiType extends ApiTypes> = QueryableStorageEntry<ApiType>;

declare module '@polkadot/api-base/types/storage' {
    interface AugmentedQueries<ApiType extends ApiTypes> {
        balances: {
            /**
             * The balance of an account.
             *
             * NOTE: This is only used in the case that this pallet is used to store balances.
             **/
            account: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<PalletBalancesAccountData>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Any liquidity locks on some account balances.
             * NOTE: Should only be accessed when setting, changing and freeing a lock.
             **/
            locks: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesBalanceLock>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Named reserves on some account balances.
             **/
            reserves: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Vec<PalletBalancesReserveData>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Storage version of the pallet.
             *
             * This is set to v2.0.0 for new networks.
             **/
            storageVersion: AugmentedQuery<ApiType, () => Observable<PalletBalancesReleases>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The total units issued in the system.
             **/
            totalIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        creditcoin: {
            addresses: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<PalletCreditcoinAddress>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            askOrders: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: H256 | string | Uint8Array,
                ) => Observable<Option<PalletCreditcoinAskOrder>>,
                [u32, H256]
            > &
                QueryableStorageEntry<ApiType, [u32, H256]>;
            authorities: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<Null>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            bidOrders: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: H256 | string | Uint8Array,
                ) => Observable<Option<PalletCreditcoinBidOrder>>,
                [u32, H256]
            > &
                QueryableStorageEntry<ApiType, [u32, H256]>;
            collectCoinsContract: AugmentedQuery<
                ApiType,
                () => Observable<PalletCreditcoinOcwTasksCollectCoinsGCreContract>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            collectedCoins: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<PalletCreditcoinCollectCoinsCollectedCoins>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            currencies: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<PalletCreditcoinPlatformCurrency>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            dealOrders: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: H256 | string | Uint8Array,
                ) => Observable<Option<PalletCreditcoinDealOrder>>,
                [u32, H256]
            > &
                QueryableStorageEntry<ApiType, [u32, H256]>;
            legacyBalanceKeeper: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            legacyWallets: AugmentedQuery<
                ApiType,
                (arg: PalletCreditcoinLegacySighash | string | Uint8Array) => Observable<Option<u128>>,
                [PalletCreditcoinLegacySighash]
            > &
                QueryableStorageEntry<ApiType, [PalletCreditcoinLegacySighash]>;
            offers: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: H256 | string | Uint8Array,
                ) => Observable<Option<PalletCreditcoinOffer>>,
                [u32, H256]
            > &
                QueryableStorageEntry<ApiType, [u32, H256]>;
            pendingTasks: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2:
                        | PalletCreditcoinTaskId
                        | { VerifyTransfer: any }
                        | { CollectCoins: any }
                        | string
                        | Uint8Array,
                ) => Observable<Option<PalletCreditcoinTask>>,
                [u32, PalletCreditcoinTaskId]
            > &
                QueryableStorageEntry<ApiType, [u32, PalletCreditcoinTaskId]>;
            transfers: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Option<PalletCreditcoinTransfer>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            usedGuids: AugmentedQuery<
                ApiType,
                (arg: Bytes | string | Uint8Array) => Observable<Option<Null>>,
                [Bytes]
            > &
                QueryableStorageEntry<ApiType, [Bytes]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        difficulty: {
            currentDifficulty: AugmentedQuery<ApiType, () => Observable<U256>, []> & QueryableStorageEntry<ApiType, []>;
            difficultyAdjustmentPeriod: AugmentedQuery<ApiType, () => Observable<i64>, []> &
                QueryableStorageEntry<ApiType, []>;
            previousDifficultiesAndTimestamps: AugmentedQuery<
                ApiType,
                () => Observable<Vec<PalletDifficultyDifficultyAndTimestamp>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            targetBlockTime: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        rewards: {
            blockAuthor: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        scheduler: {
            /**
             * Items to be executed, indexed by the block number that they should be executed on.
             **/
            agenda: AugmentedQuery<
                ApiType,
                (arg: u32 | AnyNumber | Uint8Array) => Observable<Vec<Option<PalletSchedulerScheduledV3>>>,
                [u32]
            > &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Lookup from identity to the block number and index of the task.
             **/
            lookup: AugmentedQuery<
                ApiType,
                (arg: Bytes | string | Uint8Array) => Observable<Option<ITuple<[u32, u32]>>>,
                [Bytes]
            > &
                QueryableStorageEntry<ApiType, [Bytes]>;
            /**
             * Storage version of the pallet.
             *
             * New networks start with last version.
             **/
            storageVersion: AugmentedQuery<ApiType, () => Observable<PalletSchedulerReleases>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        sudo: {
            /**
             * The `AccountId` of the sudo key.
             **/
            key: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        system: {
            /**
             * The full account information for a particular account ID.
             **/
            account: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<FrameSystemAccountInfo>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            /**
             * Total length (in bytes) for all extrinsics put together, for the current block.
             **/
            allExtrinsicsLen: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Map of block numbers to block hashes.
             **/
            blockHash: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<H256>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * The current weight for the block.
             **/
            blockWeight: AugmentedQuery<ApiType, () => Observable<FrameSupportWeightsPerDispatchClassU64>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Digest of the current block, also part of the block header.
             **/
            digest: AugmentedQuery<ApiType, () => Observable<SpRuntimeDigest>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * The number of events in the `Events<T>` list.
             **/
            eventCount: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Events deposited for the current block.
             *
             * NOTE: This storage item is explicitly unbounded since it is never intended to be read
             * from within the runtime.
             **/
            events: AugmentedQuery<ApiType, () => Observable<Vec<FrameSystemEventRecord>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Mapping between a topic (represented by T::Hash) and a vector of indexes
             * of events in the `<Events<T>>` list.
             *
             * All topic vectors have deterministic storage locations depending on the topic. This
             * allows light-clients to leverage the changes trie storage tracking mechanism and
             * in case of changes fetch the list of events of interest.
             *
             * The value has the type `(T::BlockNumber, EventIndex)` because if we used only just
             * the `EventIndex` then in case if the topic has the same contents on the next block
             * no notification will be triggered thus the event might be lost.
             **/
            eventTopics: AugmentedQuery<
                ApiType,
                (arg: H256 | string | Uint8Array) => Observable<Vec<ITuple<[u32, u32]>>>,
                [H256]
            > &
                QueryableStorageEntry<ApiType, [H256]>;
            /**
             * The execution phase of the block.
             **/
            executionPhase: AugmentedQuery<ApiType, () => Observable<Option<FrameSystemPhase>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Total extrinsics count for the current block.
             **/
            extrinsicCount: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Extrinsics data for the current block (maps an extrinsic's index to its data).
             **/
            extrinsicData: AugmentedQuery<ApiType, (arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>, [u32]> &
                QueryableStorageEntry<ApiType, [u32]>;
            /**
             * Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened.
             **/
            lastRuntimeUpgrade: AugmentedQuery<
                ApiType,
                () => Observable<Option<FrameSystemLastRuntimeUpgradeInfo>>,
                []
            > &
                QueryableStorageEntry<ApiType, []>;
            /**
             * The current block number being processed. Set by `execute_block`.
             **/
            number: AugmentedQuery<ApiType, () => Observable<u32>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Hash of the previous block.
             **/
            parentHash: AugmentedQuery<ApiType, () => Observable<H256>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * True if we have upgraded so that AccountInfo contains three types of `RefCount`. False
             * (default) if not.
             **/
            upgradedToTripleRefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * True if we have upgraded so that `type RefCount` is `u32`. False (default) if not.
             **/
            upgradedToU32RefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        taskScheduler: {
            authorities: AugmentedQuery<
                ApiType,
                (arg: AccountId32 | string | Uint8Array) => Observable<Option<Null>>,
                [AccountId32]
            > &
                QueryableStorageEntry<ApiType, [AccountId32]>;
            pendingTasks: AugmentedQuery<
                ApiType,
                (
                    arg1: u32 | AnyNumber | Uint8Array,
                    arg2: H256 | string | Uint8Array,
                ) => Observable<Option<PalletCreditcoinTask>>,
                [u32, H256]
            > &
                QueryableStorageEntry<ApiType, [u32, H256]>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        timestamp: {
            /**
             * Did the timestamp get updated in this block?
             **/
            didUpdate: AugmentedQuery<ApiType, () => Observable<bool>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Current time for the current block.
             **/
            now: AugmentedQuery<ApiType, () => Observable<u64>, []> & QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
        transactionPayment: {
            nextFeeMultiplier: AugmentedQuery<ApiType, () => Observable<u128>, []> & QueryableStorageEntry<ApiType, []>;
            storageVersion: AugmentedQuery<ApiType, () => Observable<PalletTransactionPaymentReleases>, []> &
                QueryableStorageEntry<ApiType, []>;
            /**
             * Generic query
             **/
            [key: string]: QueryableStorageEntry<ApiType>;
        };
    } // AugmentedQueries
} // declare module
