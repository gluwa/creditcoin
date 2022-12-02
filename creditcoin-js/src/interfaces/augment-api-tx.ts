// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/submittable';

import type {
    ApiTypes,
    AugmentedSubmittable,
    SubmittableExtrinsic,
    SubmittableExtrinsicFunction,
} from '@polkadot/api-base/types';
import type { Bytes, Compact, Option, U256, U8aFixed, Vec, bool, i64, u128, u32, u64, u8 } from '@polkadot/types-codec';
import type { AnyNumber, IMethod, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, H256, MultiAddress, Perbill } from '@polkadot/types/interfaces/runtime';
import type {
    PalletCreditcoinAskOrderId,
    PalletCreditcoinBidOrderId,
    PalletCreditcoinDealOrderId,
    PalletCreditcoinLegacyTransferKind,
    PalletCreditcoinLoanTerms,
    PalletCreditcoinOcwErrorsVerificationFailureCause,
    PalletCreditcoinOcwTasksCollectCoinsGCreContract,
    PalletCreditcoinOfferId,
    PalletCreditcoinPlatformBlockchain,
    PalletCreditcoinPlatformCurrency,
    PalletCreditcoinPlatformTransferKind,
    PalletCreditcoinTaskId,
    PalletCreditcoinTaskOutput,
    SpCoreEcdsaPublic,
    SpCoreEcdsaSignature,
    SpRuntimeMultiSignature,
    SpRuntimeMultiSigner,
    SpWeightsWeightV2Weight,
} from '@polkadot/types/lookup';

export type __AugmentedSubmittable = AugmentedSubmittable<() => unknown>;
export type __SubmittableExtrinsic<ApiType extends ApiTypes> = SubmittableExtrinsic<ApiType>;
export type __SubmittableExtrinsicFunction<ApiType extends ApiTypes> = SubmittableExtrinsicFunction<ApiType>;

declare module '@polkadot/api-base/types/submittable' {
    interface AugmentedSubmittables<ApiType extends ApiTypes> {
        balances: {
            /**
             * Exactly as `transfer`, except the origin must be root and the source account may be
             * specified.
             * # <weight>
             * - Same as transfer, but additional read and write because the source account is not
             * assumed to be in the overlay.
             * # </weight>
             **/
            forceTransfer: AugmentedSubmittable<
                (
                    source:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    value: Compact<u128> | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress, Compact<u128>]
            >;
            /**
             * Unreserve some balance from a user by force.
             *
             * Can only be called by ROOT.
             **/
            forceUnreserve: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    amount: u128 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, u128]
            >;
            /**
             * Set the balances of a given account.
             *
             * This will alter `FreeBalance` and `ReservedBalance` in storage. it will
             * also alter the total issuance of the system (`TotalIssuance`) appropriately.
             * If the new free or reserved balance is below the existential deposit,
             * it will reset the account nonce (`frame_system::AccountNonce`).
             *
             * The dispatch origin for this call is `root`.
             **/
            setBalance: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    newFree: Compact<u128> | AnyNumber | Uint8Array,
                    newReserved: Compact<u128> | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Compact<u128>, Compact<u128>]
            >;
            /**
             * Transfer some liquid free balance to another account.
             *
             * `transfer` will set the `FreeBalance` of the sender and receiver.
             * If the sender's account is below the existential deposit as a result
             * of the transfer, the account will be reaped.
             *
             * The dispatch origin for this call must be `Signed` by the transactor.
             *
             * # <weight>
             * - Dependent on arguments but not critical, given proper implementations for input config
             * types. See related functions below.
             * - It contains a limited number of reads and writes internally and no complex
             * computation.
             *
             * Related functions:
             *
             * - `ensure_can_withdraw` is always called internally but has a bounded complexity.
             * - Transferring balances to accounts that did not exist before will cause
             * `T::OnNewAccount::on_new_account` to be called.
             * - Removing enough funds from an account will trigger `T::DustRemoval::on_unbalanced`.
             * - `transfer_keep_alive` works the same way as `transfer`, but has an additional check
             * that the transfer will not kill the origin account.
             * ---------------------------------
             * - Origin account is already in memory, so no DB operations for them.
             * # </weight>
             **/
            transfer: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    value: Compact<u128> | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Compact<u128>]
            >;
            /**
             * Transfer the entire transferable balance from the caller account.
             *
             * NOTE: This function only attempts to transfer _transferable_ balances. This means that
             * any locked, reserved, or existential deposits (when `keep_alive` is `true`), will not be
             * transferred by this function. To ensure that this function results in a killed account,
             * you might need to prepare the account by removing any reference counters, storage
             * deposits, etc...
             *
             * The dispatch origin of this call must be Signed.
             *
             * - `dest`: The recipient of the transfer.
             * - `keep_alive`: A boolean to determine if the `transfer_all` operation should send all
             * of the funds the account has, causing the sender account to be killed (false), or
             * transfer everything except at least the existential deposit, which will guarantee to
             * keep the sender account alive (true). # <weight>
             * - O(1). Just like transfer, but reading the user's transferable balance first.
             * #</weight>
             **/
            transferAll: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    keepAlive: bool | boolean | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, bool]
            >;
            /**
             * Same as the [`transfer`] call, but with a check that the transfer will not kill the
             * origin account.
             *
             * 99% of the time you want [`transfer`] instead.
             *
             * [`transfer`]: struct.Pallet.html#method.transfer
             **/
            transferKeepAlive: AugmentedSubmittable<
                (
                    dest:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    value: Compact<u128> | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Compact<u128>]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        creditcoin: {
            addAskOrder: AugmentedSubmittable<
                (
                    addressId: H256 | string | Uint8Array,
                    terms:
                        | PalletCreditcoinLoanTerms
                        | { amount?: any; interestRate?: any; termLength?: any; currency?: any }
                        | string
                        | Uint8Array,
                    expirationBlock: u32 | AnyNumber | Uint8Array,
                    guid: Bytes | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [H256, PalletCreditcoinLoanTerms, u32, Bytes]
            >;
            addAuthority: AugmentedSubmittable<
                (who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            addBidOrder: AugmentedSubmittable<
                (
                    addressId: H256 | string | Uint8Array,
                    terms:
                        | PalletCreditcoinLoanTerms
                        | { amount?: any; interestRate?: any; termLength?: any; currency?: any }
                        | string
                        | Uint8Array,
                    expirationBlock: u32 | AnyNumber | Uint8Array,
                    guid: Bytes | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [H256, PalletCreditcoinLoanTerms, u32, Bytes]
            >;
            addDealOrder: AugmentedSubmittable<
                (
                    offerId: PalletCreditcoinOfferId,
                    expirationBlock: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinOfferId, u32]
            >;
            addOffer: AugmentedSubmittable<
                (
                    askOrderId: PalletCreditcoinAskOrderId,
                    bidOrderId: PalletCreditcoinBidOrderId,
                    expirationBlock: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinAskOrderId, PalletCreditcoinBidOrderId, u32]
            >;
            /**
             * Claims legacy wallet and transfers the balance to the sender's account.
             **/
            claimLegacyWallet: AugmentedSubmittable<
                (publicKey: SpCoreEcdsaPublic | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [SpCoreEcdsaPublic]
            >;
            closeDealOrder: AugmentedSubmittable<
                (
                    dealOrderId: PalletCreditcoinDealOrderId,
                    transferId: H256 | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinDealOrderId, H256]
            >;
            exempt: AugmentedSubmittable<
                (dealOrderId: PalletCreditcoinDealOrderId) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinDealOrderId]
            >;
            failTask: AugmentedSubmittable<
                (
                    deadline: u32 | AnyNumber | Uint8Array,
                    taskId:
                        | PalletCreditcoinTaskId
                        | { VerifyTransfer: any }
                        | { CollectCoins: any }
                        | string
                        | Uint8Array,
                    cause:
                        | PalletCreditcoinOcwErrorsVerificationFailureCause
                        | 'TaskNonexistent'
                        | 'TaskFailed'
                        | 'TaskPending'
                        | 'TaskUnconfirmed'
                        | 'TaskInFuture'
                        | 'IncorrectContract'
                        | 'MissingReceiver'
                        | 'MissingSender'
                        | 'AbiMismatch'
                        | 'IncorrectInputLength'
                        | 'EmptyInput'
                        | 'IncorrectInputType'
                        | 'IncorrectAmount'
                        | 'IncorrectNonce'
                        | 'IncorrectReceiver'
                        | 'IncorrectSender'
                        | 'InvalidAddress'
                        | 'UnsupportedMethod'
                        | 'TransactionNotFound'
                        | number
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u32, PalletCreditcoinTaskId, PalletCreditcoinOcwErrorsVerificationFailureCause]
            >;
            fundDealOrder: AugmentedSubmittable<
                (
                    dealOrderId: PalletCreditcoinDealOrderId,
                    transferId: H256 | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinDealOrderId, H256]
            >;
            lockDealOrder: AugmentedSubmittable<
                (dealOrderId: PalletCreditcoinDealOrderId) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinDealOrderId]
            >;
            persistTaskOutput: AugmentedSubmittable<
                (
                    deadline: u32 | AnyNumber | Uint8Array,
                    taskOutput:
                        | PalletCreditcoinTaskOutput
                        | { VerifyTransfer: any }
                        | { CollectCoins: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u32, PalletCreditcoinTaskOutput]
            >;
            /**
             * Registers an external address on `blockchain` and `network` with value `address`
             **/
            registerAddress: AugmentedSubmittable<
                (
                    blockchain: PalletCreditcoinPlatformBlockchain | { Evm: any } | string | Uint8Array,
                    address: Bytes | string | Uint8Array,
                    ownershipProof: SpCoreEcdsaSignature | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinPlatformBlockchain, Bytes, SpCoreEcdsaSignature]
            >;
            registerCurrency: AugmentedSubmittable<
                (
                    currency: PalletCreditcoinPlatformCurrency | { Evm: any } | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinPlatformCurrency]
            >;
            registerDealOrder: AugmentedSubmittable<
                (
                    lenderAddressId: H256 | string | Uint8Array,
                    borrowerAddressId: H256 | string | Uint8Array,
                    terms:
                        | PalletCreditcoinLoanTerms
                        | { amount?: any; interestRate?: any; termLength?: any; currency?: any }
                        | string
                        | Uint8Array,
                    expirationBlock: u32 | AnyNumber | Uint8Array,
                    askGuid: Bytes | string | Uint8Array,
                    bidGuid: Bytes | string | Uint8Array,
                    borrowerKey:
                        | SpRuntimeMultiSigner
                        | { Ed25519: any }
                        | { Sr25519: any }
                        | { Ecdsa: any }
                        | string
                        | Uint8Array,
                    borrowerSignature:
                        | SpRuntimeMultiSignature
                        | { Ed25519: any }
                        | { Sr25519: any }
                        | { Ecdsa: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [
                    H256,
                    H256,
                    PalletCreditcoinLoanTerms,
                    u32,
                    Bytes,
                    Bytes,
                    SpRuntimeMultiSigner,
                    SpRuntimeMultiSignature,
                ]
            >;
            registerFundingTransfer: AugmentedSubmittable<
                (
                    transferKind: PalletCreditcoinPlatformTransferKind | { Evm: any } | string | Uint8Array,
                    dealOrderId: PalletCreditcoinDealOrderId,
                    blockchainTxId: Bytes | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinPlatformTransferKind, PalletCreditcoinDealOrderId, Bytes]
            >;
            registerFundingTransferLegacy: AugmentedSubmittable<
                (
                    transferKind:
                        | PalletCreditcoinLegacyTransferKind
                        | { Erc20: any }
                        | { Ethless: any }
                        | { Native: any }
                        | { Other: any }
                        | string
                        | Uint8Array,
                    dealOrderId: PalletCreditcoinDealOrderId,
                    blockchainTxId: Bytes | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinLegacyTransferKind, PalletCreditcoinDealOrderId, Bytes]
            >;
            registerRepaymentTransfer: AugmentedSubmittable<
                (
                    transferKind: PalletCreditcoinPlatformTransferKind | { Evm: any } | string | Uint8Array,
                    repaymentAmount: U256 | AnyNumber | Uint8Array,
                    dealOrderId: PalletCreditcoinDealOrderId,
                    blockchainTxId: Bytes | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinPlatformTransferKind, U256, PalletCreditcoinDealOrderId, Bytes]
            >;
            registerRepaymentTransferLegacy: AugmentedSubmittable<
                (
                    transferKind:
                        | PalletCreditcoinLegacyTransferKind
                        | { Erc20: any }
                        | { Ethless: any }
                        | { Native: any }
                        | { Other: any }
                        | string
                        | Uint8Array,
                    repaymentAmount: U256 | AnyNumber | Uint8Array,
                    dealOrderId: PalletCreditcoinDealOrderId,
                    blockchainTxId: Bytes | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinLegacyTransferKind, U256, PalletCreditcoinDealOrderId, Bytes]
            >;
            removeAuthority: AugmentedSubmittable<
                (who: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            requestCollectCoins: AugmentedSubmittable<
                (
                    evmAddress: Bytes | string | Uint8Array,
                    txId: Bytes | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, Bytes]
            >;
            setCollectCoinsContract: AugmentedSubmittable<
                (
                    contract:
                        | PalletCreditcoinOcwTasksCollectCoinsGCreContract
                        | { address?: any; chain?: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinOcwTasksCollectCoinsGCreContract]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        difficulty: {
            setAdjustmentPeriod: AugmentedSubmittable<
                (period: i64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [i64]
            >;
            setTargetBlockTime: AugmentedSubmittable<
                (targetTime: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        scheduler: {
            /**
             * Cancel an anonymously scheduled task.
             **/
            cancel: AugmentedSubmittable<
                (
                    when: u32 | AnyNumber | Uint8Array,
                    index: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Cancel a named scheduled task.
             **/
            cancelNamed: AugmentedSubmittable<
                (id: U8aFixed | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [U8aFixed]
            >;
            /**
             * Anonymously schedule a task.
             **/
            schedule: AugmentedSubmittable<
                (
                    when: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
            /**
             * Anonymously schedule a task after a delay.
             *
             * # <weight>
             * Same as [`schedule`].
             * # </weight>
             **/
            scheduleAfter: AugmentedSubmittable<
                (
                    after: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
            /**
             * Schedule a named task.
             **/
            scheduleNamed: AugmentedSubmittable<
                (
                    id: U8aFixed | string | Uint8Array,
                    when: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [U8aFixed, u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
            /**
             * Schedule a named task after a delay.
             *
             * # <weight>
             * Same as [`schedule_named`](Self::schedule_named).
             * # </weight>
             **/
            scheduleNamedAfter: AugmentedSubmittable<
                (
                    id: U8aFixed | string | Uint8Array,
                    after: u32 | AnyNumber | Uint8Array,
                    maybePeriodic:
                        | Option<ITuple<[u32, u32]>>
                        | null
                        | Uint8Array
                        | ITuple<[u32, u32]>
                        | [u32 | AnyNumber | Uint8Array, u32 | AnyNumber | Uint8Array],
                    priority: u8 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [U8aFixed, u32, Option<ITuple<[u32, u32]>>, u8, Call]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        sudo: {
            /**
             * Authenticates the current sudo key and sets the given AccountId (`new`) as the new sudo
             * key.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * # <weight>
             * - O(1).
             * - Limited storage reads.
             * - One DB change.
             * # </weight>
             **/
            setKey: AugmentedSubmittable<
                (
                    updated:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress]
            >;
            /**
             * Authenticates the sudo key and dispatches a function call with `Root` origin.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * # <weight>
             * - O(1).
             * - Limited storage reads.
             * - One DB write (event).
             * - Weight of derivative `call` execution + 10,000.
             * # </weight>
             **/
            sudo: AugmentedSubmittable<
                (call: Call | IMethod | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Call]
            >;
            /**
             * Authenticates the sudo key and dispatches a function call with `Signed` origin from
             * a given account.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * # <weight>
             * - O(1).
             * - Limited storage reads.
             * - One DB write (event).
             * - Weight of derivative `call` execution + 10,000.
             * # </weight>
             **/
            sudoAs: AugmentedSubmittable<
                (
                    who:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    call: Call | IMethod | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Call]
            >;
            /**
             * Authenticates the sudo key and dispatches a function call with `Root` origin.
             * This function does not check the weight of the call, and instead allows the
             * Sudo user to specify the weight of the call.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * # <weight>
             * - O(1).
             * - The weight of this call is defined by the caller.
             * # </weight>
             **/
            sudoUncheckedWeight: AugmentedSubmittable<
                (
                    call: Call | IMethod | string | Uint8Array,
                    weight: SpWeightsWeightV2Weight | { refTime?: any; proofSize?: any } | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [Call, SpWeightsWeightV2Weight]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        system: {
            /**
             * A dispatch that will fill the block weight up to the given ratio.
             **/
            fillBlock: AugmentedSubmittable<
                (ratio: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Perbill]
            >;
            /**
             * Kill all storage items with a key that starts with the given prefix.
             *
             * **NOTE:** We rely on the Root origin to provide us the number of subkeys under
             * the prefix we are removing to accurately calculate the weight of this function.
             **/
            killPrefix: AugmentedSubmittable<
                (
                    prefix: Bytes | string | Uint8Array,
                    subkeys: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [Bytes, u32]
            >;
            /**
             * Kill some items from storage.
             **/
            killStorage: AugmentedSubmittable<
                (keys: Vec<Bytes> | (Bytes | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Bytes>]
            >;
            /**
             * Make some on-chain remark.
             *
             * # <weight>
             * - `O(1)`
             * # </weight>
             **/
            remark: AugmentedSubmittable<
                (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Make some on-chain remark and emit event.
             **/
            remarkWithEvent: AugmentedSubmittable<
                (remark: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the new runtime code.
             *
             * # <weight>
             * - `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`
             * - 1 call to `can_set_code`: `O(S)` (calls `sp_io::misc::runtime_version` which is
             * expensive).
             * - 1 storage write (codec `O(C)`).
             * - 1 digest item.
             * - 1 event.
             * The weight of this function is dependent on the runtime, but generally this is very
             * expensive. We will treat this as a full block.
             * # </weight>
             **/
            setCode: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the new runtime code without doing any checks of the given `code`.
             *
             * # <weight>
             * - `O(C)` where `C` length of `code`
             * - 1 storage write (codec `O(C)`).
             * - 1 digest item.
             * - 1 event.
             * The weight of this function is dependent on the runtime. We will treat this as a full
             * block. # </weight>
             **/
            setCodeWithoutChecks: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the number of pages in the WebAssembly environment's heap.
             **/
            setHeapPages: AugmentedSubmittable<
                (pages: u64 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u64]
            >;
            /**
             * Set some items of storage.
             **/
            setStorage: AugmentedSubmittable<
                (
                    items: Vec<ITuple<[Bytes, Bytes]>> | [Bytes | string | Uint8Array, Bytes | string | Uint8Array][],
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<ITuple<[Bytes, Bytes]>>]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        timestamp: {
            /**
             * Set the current time.
             *
             * This call should be invoked exactly once per block. It will panic at the finalization
             * phase, if this call hasn't been invoked by that time.
             *
             * The timestamp should be greater than the previous one by the amount specified by
             * `MinimumPeriod`.
             *
             * The dispatch origin for this call must be `Inherent`.
             *
             * # <weight>
             * - `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)
             * - 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in
             * `on_finalize`)
             * - 1 event handler `on_timestamp_set`. Must be `O(1)`.
             * # </weight>
             **/
            set: AugmentedSubmittable<
                (now: Compact<u64> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u64>]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
    } // AugmentedSubmittables
} // declare module
