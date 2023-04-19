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
import type { AccountId32, Call, H256, MultiAddress, Perbill, Percent } from '@polkadot/types/interfaces/runtime';
import type {
    CreditcoinNodeRuntimeOpaqueSessionKeys,
    PalletCreditcoinAskOrderId,
    PalletCreditcoinBidOrderId,
    PalletCreditcoinBlockchain,
    PalletCreditcoinDealOrderId,
    PalletCreditcoinLoanTerms,
    PalletCreditcoinOcwErrorsVerificationFailureCause,
    PalletCreditcoinOcwTasksCollectCoinsGCreContract,
    PalletCreditcoinOfferId,
    PalletCreditcoinTaskId,
    PalletCreditcoinTaskOutput,
    PalletCreditcoinTransferKind,
    PalletImOnlineHeartbeat,
    PalletImOnlineSr25519AppSr25519Signature,
    PalletStakingPalletConfigOpPerbill,
    PalletStakingPalletConfigOpPercent,
    PalletStakingPalletConfigOpU128,
    PalletStakingPalletConfigOpU32,
    PalletStakingRewardDestination,
    PalletStakingValidatorPrefs,
    SpConsensusBabeDigestsNextConfigDescriptor,
    SpConsensusGrandpaEquivocationProof,
    SpConsensusSlotsEquivocationProof,
    SpCoreEcdsaPublic,
    SpCoreEcdsaSignature,
    SpRuntimeMultiSignature,
    SpRuntimeMultiSigner,
    SpSessionMembershipProof,
    SpWeightsWeightV2Weight,
} from '@polkadot/types/lookup';

export type __AugmentedSubmittable = AugmentedSubmittable<() => unknown>;
export type __SubmittableExtrinsic<ApiType extends ApiTypes> = SubmittableExtrinsic<ApiType>;
export type __SubmittableExtrinsicFunction<ApiType extends ApiTypes> = SubmittableExtrinsicFunction<ApiType>;

declare module '@polkadot/api-base/types/submittable' {
    interface AugmentedSubmittables<ApiType extends ApiTypes> {
        babe: {
            /**
             * Plan an epoch config change. The epoch config change is recorded and will be enacted on
             * the next call to `enact_epoch_change`. The config will be activated one epoch after.
             * Multiple calls to this method will replace any existing planned config change that had
             * not been enacted yet.
             **/
            planConfigChange: AugmentedSubmittable<
                (
                    config: SpConsensusBabeDigestsNextConfigDescriptor | { V1: any } | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusBabeDigestsNextConfigDescriptor]
            >;
            /**
             * Report authority equivocation/misbehavior. This method will verify
             * the equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence will
             * be reported.
             **/
            reportEquivocation: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusSlotsEquivocationProof
                        | { offender?: any; slot?: any; firstHeader?: any; secondHeader?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusSlotsEquivocationProof, SpSessionMembershipProof]
            >;
            /**
             * Report authority equivocation/misbehavior. This method will verify
             * the equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence will
             * be reported.
             * This extrinsic must be called unsigned and it is expected that only
             * block authors will call it (validated in `ValidateUnsigned`), as such
             * if the block author is defined it will be defined as the equivocation
             * reporter.
             **/
            reportEquivocationUnsigned: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusSlotsEquivocationProof
                        | { offender?: any; slot?: any; firstHeader?: any; secondHeader?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusSlotsEquivocationProof, SpSessionMembershipProof]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        balances: {
            /**
             * Exactly as `transfer`, except the origin must be root and the source account may be
             * specified.
             * ## Complexity
             * - Same as transfer, but additional read and write because the source account is not
             * assumed to be in the overlay.
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
             * ## Complexity
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
             * keep the sender account alive (true). ## Complexity
             * - O(1). Just like transfer, but reading the user's transferable balance first.
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
                        | { amount?: any; interestRate?: any; termLength?: any }
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
                        | { amount?: any; interestRate?: any; termLength?: any }
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
                    blockchain:
                        | PalletCreditcoinBlockchain
                        | { Ethereum: any }
                        | { Rinkeby: any }
                        | { Luniverse: any }
                        | { Bitcoin: any }
                        | { Other: any }
                        | string
                        | Uint8Array,
                    address: Bytes | string | Uint8Array,
                    ownershipProof: SpCoreEcdsaSignature | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinBlockchain, Bytes, SpCoreEcdsaSignature]
            >;
            registerDealOrder: AugmentedSubmittable<
                (
                    lenderAddressId: H256 | string | Uint8Array,
                    borrowerAddressId: H256 | string | Uint8Array,
                    terms:
                        | PalletCreditcoinLoanTerms
                        | { amount?: any; interestRate?: any; termLength?: any }
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
                    transferKind:
                        | PalletCreditcoinTransferKind
                        | { Erc20: any }
                        | { Ethless: any }
                        | { Native: any }
                        | { Other: any }
                        | string
                        | Uint8Array,
                    dealOrderId: PalletCreditcoinDealOrderId,
                    blockchainTxId: Bytes | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinTransferKind, PalletCreditcoinDealOrderId, Bytes]
            >;
            registerRepaymentTransfer: AugmentedSubmittable<
                (
                    transferKind:
                        | PalletCreditcoinTransferKind
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
                [PalletCreditcoinTransferKind, U256, PalletCreditcoinDealOrderId, Bytes]
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
        grandpa: {
            /**
             * Note that the current authority set of the GRANDPA finality gadget has stalled.
             *
             * This will trigger a forced authority set change at the beginning of the next session, to
             * be enacted `delay` blocks after that. The `delay` should be high enough to safely assume
             * that the block signalling the forced change will not be re-orged e.g. 1000 blocks.
             * The block production rate (which may be slowed down because of finality lagging) should
             * be taken into account when choosing the `delay`. The GRANDPA voters based on the new
             * authority will start voting on top of `best_finalized_block_number` for new finalized
             * blocks. `best_finalized_block_number` should be the highest of the latest finalized
             * block of all validators of the new authority set.
             *
             * Only callable by root.
             **/
            noteStalled: AugmentedSubmittable<
                (
                    delay: u32 | AnyNumber | Uint8Array,
                    bestFinalizedBlockNumber: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Report voter equivocation/misbehavior. This method will verify the
             * equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence
             * will be reported.
             **/
            reportEquivocation: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusGrandpaEquivocationProof
                        | { setId?: any; equivocation?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusGrandpaEquivocationProof, SpSessionMembershipProof]
            >;
            /**
             * Report voter equivocation/misbehavior. This method will verify the
             * equivocation proof and validate the given key ownership proof
             * against the extracted offender. If both are valid, the offence
             * will be reported.
             *
             * This extrinsic must be called unsigned and it is expected that only
             * block authors will call it (validated in `ValidateUnsigned`), as such
             * if the block author is defined it will be defined as the equivocation
             * reporter.
             **/
            reportEquivocationUnsigned: AugmentedSubmittable<
                (
                    equivocationProof:
                        | SpConsensusGrandpaEquivocationProof
                        | { setId?: any; equivocation?: any }
                        | string
                        | Uint8Array,
                    keyOwnerProof:
                        | SpSessionMembershipProof
                        | { session?: any; trieNodes?: any; validatorCount?: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [SpConsensusGrandpaEquivocationProof, SpSessionMembershipProof]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        imOnline: {
            /**
             * ## Complexity:
             * - `O(K + E)` where K is length of `Keys` (heartbeat.validators_len) and E is length of
             * `heartbeat.network_state.external_address`
             * - `O(K)`: decoding of length `K`
             * - `O(E)`: decoding/encoding of length `E`
             **/
            heartbeat: AugmentedSubmittable<
                (
                    heartbeat:
                        | PalletImOnlineHeartbeat
                        | {
                              blockNumber?: any;
                              networkState?: any;
                              sessionIndex?: any;
                              authorityIndex?: any;
                              validatorsLen?: any;
                          }
                        | string
                        | Uint8Array,
                    signature: PalletImOnlineSr25519AppSr25519Signature | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletImOnlineHeartbeat, PalletImOnlineSr25519AppSr25519Signature]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        posSwitch: {
            /**
             * Switch to PoS
             **/
            switchToPos: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
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
        session: {
            /**
             * Removes any session key(s) of the function caller.
             *
             * This doesn't take effect until the next session.
             *
             * The dispatch origin of this function must be Signed and the account must be either be
             * convertible to a validator ID using the chain's typical addressing system (this usually
             * means being a controller account) or directly convertible into a validator ID (which
             * usually means being a stash account).
             *
             * ## Complexity
             * - `O(1)` in number of key types. Actual cost depends on the number of length of
             * `T::Keys::key_ids()` which is fixed.
             **/
            purgeKeys: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Sets the session key(s) of the function caller to `keys`.
             * Allows an account to set its session key prior to becoming a validator.
             * This doesn't take effect until the next session.
             *
             * The dispatch origin of this function must be signed.
             *
             * ## Complexity
             * - `O(1)`. Actual cost depends on the number of length of `T::Keys::key_ids()` which is
             * fixed.
             **/
            setKeys: AugmentedSubmittable<
                (
                    keys:
                        | CreditcoinNodeRuntimeOpaqueSessionKeys
                        | { grandpa?: any; babe?: any; imOnline?: any }
                        | string
                        | Uint8Array,
                    proof: Bytes | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [CreditcoinNodeRuntimeOpaqueSessionKeys, Bytes]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        staking: {
            /**
             * Take the origin account as a stash and lock up `value` of its balance. `controller` will
             * be the account that controls it.
             *
             * `value` must be more than the `minimum_balance` specified by `T::Currency`.
             *
             * The dispatch origin for this call must be _Signed_ by the stash account.
             *
             * Emits `Bonded`.
             * ## Complexity
             * - Independent of the arguments. Moderate complexity.
             * - O(1).
             * - Three extra DB entries.
             *
             * NOTE: Two of the storage writes (`Self::bonded`, `Self::payee`) are _never_ cleaned
             * unless the `origin` falls below _existential deposit_ and gets removed as dust.
             **/
            bond: AugmentedSubmittable<
                (
                    controller:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    value: Compact<u128> | AnyNumber | Uint8Array,
                    payee:
                        | PalletStakingRewardDestination
                        | { Staked: any }
                        | { Stash: any }
                        | { Controller: any }
                        | { Account: any }
                        | { None: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Compact<u128>, PalletStakingRewardDestination]
            >;
            /**
             * Add some extra amount that have appeared in the stash `free_balance` into the balance up
             * for staking.
             *
             * The dispatch origin for this call must be _Signed_ by the stash, not the controller.
             *
             * Use this if there are additional funds in your stash account that you wish to bond.
             * Unlike [`bond`](Self::bond) or [`unbond`](Self::unbond) this function does not impose
             * any limitation on the amount that can be added.
             *
             * Emits `Bonded`.
             *
             * ## Complexity
             * - Independent of the arguments. Insignificant complexity.
             * - O(1).
             **/
            bondExtra: AugmentedSubmittable<
                (maxAdditional: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u128>]
            >;
            /**
             * Cancel enactment of a deferred slash.
             *
             * Can be called by the `T::AdminOrigin`.
             *
             * Parameters: era and indices of the slashes for that era to kill.
             **/
            cancelDeferredSlash: AugmentedSubmittable<
                (
                    era: u32 | AnyNumber | Uint8Array,
                    slashIndices: Vec<u32> | (u32 | AnyNumber | Uint8Array)[],
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Vec<u32>]
            >;
            /**
             * Declare no desire to either validate or nominate.
             *
             * Effects will be felt at the beginning of the next era.
             *
             * The dispatch origin for this call must be _Signed_ by the controller, not the stash.
             *
             * ## Complexity
             * - Independent of the arguments. Insignificant complexity.
             * - Contains one read.
             * - Writes are limited to the `origin` account key.
             **/
            chill: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Declare a `controller` to stop participating as either a validator or nominator.
             *
             * Effects will be felt at the beginning of the next era.
             *
             * The dispatch origin for this call must be _Signed_, but can be called by anyone.
             *
             * If the caller is the same as the controller being targeted, then no further checks are
             * enforced, and this function behaves just like `chill`.
             *
             * If the caller is different than the controller being targeted, the following conditions
             * must be met:
             *
             * * `controller` must belong to a nominator who has become non-decodable,
             *
             * Or:
             *
             * * A `ChillThreshold` must be set and checked which defines how close to the max
             * nominators or validators we must reach before users can start chilling one-another.
             * * A `MaxNominatorCount` and `MaxValidatorCount` must be set which is used to determine
             * how close we are to the threshold.
             * * A `MinNominatorBond` and `MinValidatorBond` must be set and checked, which determines
             * if this is a person that should be chilled because they have not met the threshold
             * bond required.
             *
             * This can be helpful if bond requirements are updated, and we need to remove old users
             * who do not satisfy these requirements.
             **/
            chillOther: AugmentedSubmittable<
                (controller: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Force a validator to have at least the minimum commission. This will not affect a
             * validator who already has a commission greater than or equal to the minimum. Any account
             * can call this.
             **/
            forceApplyMinCommission: AugmentedSubmittable<
                (validatorStash: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Force there to be a new era at the end of the next session. After this, it will be
             * reset to normal (non-forced) behaviour.
             *
             * The dispatch origin must be Root.
             *
             * # Warning
             *
             * The election process starts multiple blocks before the end of the era.
             * If this is called just before a new era is triggered, the election process may not
             * have enough blocks to get a result.
             *
             * ## Complexity
             * - No arguments.
             * - Weight: O(1)
             **/
            forceNewEra: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Force there to be a new era at the end of sessions indefinitely.
             *
             * The dispatch origin must be Root.
             *
             * # Warning
             *
             * The election process starts multiple blocks before the end of the era.
             * If this is called just before a new era is triggered, the election process may not
             * have enough blocks to get a result.
             **/
            forceNewEraAlways: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Force there to be no new eras indefinitely.
             *
             * The dispatch origin must be Root.
             *
             * # Warning
             *
             * The election process starts multiple blocks before the end of the era.
             * Thus the election process may be ongoing when this is called. In this case the
             * election will continue until the next era is triggered.
             *
             * ## Complexity
             * - No arguments.
             * - Weight: O(1)
             **/
            forceNoEras: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Force a current staker to become completely unstaked, immediately.
             *
             * The dispatch origin must be Root.
             **/
            forceUnstake: AugmentedSubmittable<
                (
                    stash: AccountId32 | string | Uint8Array,
                    numSlashingSpans: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, u32]
            >;
            /**
             * Increments the ideal number of validators upto maximum of
             * `ElectionProviderBase::MaxWinners`.
             *
             * The dispatch origin must be Root.
             *
             * ## Complexity
             * Same as [`Self::set_validator_count`].
             **/
            increaseValidatorCount: AugmentedSubmittable<
                (additional: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Remove the given nominations from the calling validator.
             *
             * Effects will be felt at the beginning of the next era.
             *
             * The dispatch origin for this call must be _Signed_ by the controller, not the stash.
             *
             * - `who`: A list of nominator stash accounts who are nominating this validator which
             * should no longer be nominating this validator.
             *
             * Note: Making this call only makes sense if you first set the validator preferences to
             * block any further nominations.
             **/
            kick: AugmentedSubmittable<
                (
                    who:
                        | Vec<MultiAddress>
                        | (
                              | MultiAddress
                              | { Id: any }
                              | { Index: any }
                              | { Raw: any }
                              | { Address32: any }
                              | { Address20: any }
                              | string
                              | Uint8Array
                          )[],
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<MultiAddress>]
            >;
            /**
             * Declare the desire to nominate `targets` for the origin controller.
             *
             * Effects will be felt at the beginning of the next era.
             *
             * The dispatch origin for this call must be _Signed_ by the controller, not the stash.
             *
             * ## Complexity
             * - The transaction's complexity is proportional to the size of `targets` (N)
             * which is capped at CompactAssignments::LIMIT (T::MaxNominations).
             * - Both the reads and writes follow a similar pattern.
             **/
            nominate: AugmentedSubmittable<
                (
                    targets:
                        | Vec<MultiAddress>
                        | (
                              | MultiAddress
                              | { Id: any }
                              | { Index: any }
                              | { Raw: any }
                              | { Address32: any }
                              | { Address20: any }
                              | string
                              | Uint8Array
                          )[],
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<MultiAddress>]
            >;
            /**
             * Pay out all the stakers behind a single validator for a single era.
             *
             * - `validator_stash` is the stash account of the validator. Their nominators, up to
             * `T::MaxNominatorRewardedPerValidator`, will also receive their rewards.
             * - `era` may be any era between `[current_era - history_depth; current_era]`.
             *
             * The origin of this call must be _Signed_. Any account can call this function, even if
             * it is not one of the stakers.
             *
             * ## Complexity
             * - At most O(MaxNominatorRewardedPerValidator).
             **/
            payoutStakers: AugmentedSubmittable<
                (
                    validatorStash: AccountId32 | string | Uint8Array,
                    era: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, u32]
            >;
            /**
             * Remove all data structures concerning a staker/stash once it is at a state where it can
             * be considered `dust` in the staking system. The requirements are:
             *
             * 1. the `total_balance` of the stash is below existential deposit.
             * 2. or, the `ledger.total` of the stash is below existential deposit.
             *
             * The former can happen in cases like a slash; the latter when a fully unbonded account
             * is still receiving staking rewards in `RewardDestination::Staked`.
             *
             * It can be called by anyone, as long as `stash` meets the above requirements.
             *
             * Refunds the transaction fees upon successful execution.
             **/
            reapStash: AugmentedSubmittable<
                (
                    stash: AccountId32 | string | Uint8Array,
                    numSlashingSpans: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [AccountId32, u32]
            >;
            /**
             * Rebond a portion of the stash scheduled to be unlocked.
             *
             * The dispatch origin must be signed by the controller.
             *
             * ## Complexity
             * - Time complexity: O(L), where L is unlocking chunks
             * - Bounded by `MaxUnlockingChunks`.
             **/
            rebond: AugmentedSubmittable<
                (value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u128>]
            >;
            /**
             * Scale up the ideal number of validators by a factor upto maximum of
             * `ElectionProviderBase::MaxWinners`.
             *
             * The dispatch origin must be Root.
             *
             * ## Complexity
             * Same as [`Self::set_validator_count`].
             **/
            scaleValidatorCount: AugmentedSubmittable<
                (factor: Percent | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Percent]
            >;
            /**
             * (Re-)set the controller of a stash.
             *
             * Effects will be felt instantly (as soon as this function is completed successfully).
             *
             * The dispatch origin for this call must be _Signed_ by the stash, not the controller.
             *
             * ## Complexity
             * O(1)
             * - Independent of the arguments. Insignificant complexity.
             * - Contains a limited number of reads.
             * - Writes are limited to the `origin` account key.
             **/
            setController: AugmentedSubmittable<
                (
                    controller:
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
             * Set the validators who cannot be slashed (if any).
             *
             * The dispatch origin must be Root.
             **/
            setInvulnerables: AugmentedSubmittable<
                (
                    invulnerables: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<AccountId32>]
            >;
            /**
             * Sets the minimum amount of commission that each validators must maintain.
             *
             * This call has lower privilege requirements than `set_staking_config` and can be called
             * by the `T::AdminOrigin`. Root can always call this.
             **/
            setMinCommission: AugmentedSubmittable<
                (updated: Perbill | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Perbill]
            >;
            /**
             * (Re-)set the payment target for a controller.
             *
             * Effects will be felt instantly (as soon as this function is completed successfully).
             *
             * The dispatch origin for this call must be _Signed_ by the controller, not the stash.
             *
             * ## Complexity
             * - O(1)
             * - Independent of the arguments. Insignificant complexity.
             * - Contains a limited number of reads.
             * - Writes are limited to the `origin` account key.
             * ---------
             **/
            setPayee: AugmentedSubmittable<
                (
                    payee:
                        | PalletStakingRewardDestination
                        | { Staked: any }
                        | { Stash: any }
                        | { Controller: any }
                        | { Account: any }
                        | { None: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletStakingRewardDestination]
            >;
            /**
             * Update the various staking configurations .
             *
             * * `min_nominator_bond`: The minimum active bond needed to be a nominator.
             * * `min_validator_bond`: The minimum active bond needed to be a validator.
             * * `max_nominator_count`: The max number of users who can be a nominator at once. When
             * set to `None`, no limit is enforced.
             * * `max_validator_count`: The max number of users who can be a validator at once. When
             * set to `None`, no limit is enforced.
             * * `chill_threshold`: The ratio of `max_nominator_count` or `max_validator_count` which
             * should be filled in order for the `chill_other` transaction to work.
             * * `min_commission`: The minimum amount of commission that each validators must maintain.
             * This is checked only upon calling `validate`. Existing validators are not affected.
             *
             * RuntimeOrigin must be Root to call this function.
             *
             * NOTE: Existing nominators and validators will not be affected by this update.
             * to kick people under the new limits, `chill_other` should be called.
             **/
            setStakingConfigs: AugmentedSubmittable<
                (
                    minNominatorBond:
                        | PalletStakingPalletConfigOpU128
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    minValidatorBond:
                        | PalletStakingPalletConfigOpU128
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    maxNominatorCount:
                        | PalletStakingPalletConfigOpU32
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    maxValidatorCount:
                        | PalletStakingPalletConfigOpU32
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    chillThreshold:
                        | PalletStakingPalletConfigOpPercent
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    minCommission:
                        | PalletStakingPalletConfigOpPerbill
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [
                    PalletStakingPalletConfigOpU128,
                    PalletStakingPalletConfigOpU128,
                    PalletStakingPalletConfigOpU32,
                    PalletStakingPalletConfigOpU32,
                    PalletStakingPalletConfigOpPercent,
                    PalletStakingPalletConfigOpPerbill,
                ]
            >;
            /**
             * Sets the ideal number of validators.
             *
             * The dispatch origin must be Root.
             *
             * ## Complexity
             * O(1)
             **/
            setValidatorCount: AugmentedSubmittable<
                (updated: Compact<u32> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>]
            >;
            /**
             * Schedule a portion of the stash to be unlocked ready for transfer out after the bond
             * period ends. If this leaves an amount actively bonded less than
             * T::Currency::minimum_balance(), then it is increased to the full amount.
             *
             * The dispatch origin for this call must be _Signed_ by the controller, not the stash.
             *
             * Once the unlock period is done, you can call `withdraw_unbonded` to actually move
             * the funds out of management ready for transfer.
             *
             * No more than a limited number of unlocking chunks (see `MaxUnlockingChunks`)
             * can co-exists at the same time. If there are no unlocking chunks slots available
             * [`Call::withdraw_unbonded`] is called to remove some of the chunks (if possible).
             *
             * If a user encounters the `InsufficientBond` error when calling this extrinsic,
             * they should call `chill` first in order to free up their bonded funds.
             *
             * Emits `Unbonded`.
             *
             * See also [`Call::withdraw_unbonded`].
             **/
            unbond: AugmentedSubmittable<
                (value: Compact<u128> | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Compact<u128>]
            >;
            /**
             * Declare the desire to validate for the origin controller.
             *
             * Effects will be felt at the beginning of the next era.
             *
             * The dispatch origin for this call must be _Signed_ by the controller, not the stash.
             **/
            validate: AugmentedSubmittable<
                (
                    prefs: PalletStakingValidatorPrefs | { commission?: any; blocked?: any } | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletStakingValidatorPrefs]
            >;
            /**
             * Remove any unlocked chunks from the `unlocking` queue from our management.
             *
             * This essentially frees up that balance to be used by the stash account to do
             * whatever it wants.
             *
             * The dispatch origin for this call must be _Signed_ by the controller.
             *
             * Emits `Withdrawn`.
             *
             * See also [`Call::unbond`].
             *
             * ## Complexity
             * O(S) where S is the number of slashing spans to remove
             * NOTE: Weight annotation is the kill scenario, we refund otherwise.
             **/
            withdrawUnbonded: AugmentedSubmittable<
                (numSlashingSpans: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
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
             * ## Complexity
             * - O(1).
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
             * ## Complexity
             * - O(1).
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
             * ## Complexity
             * - O(1).
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
             * ## Complexity
             * - O(1).
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
             * ## Complexity
             * - `O(1)`
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
             * ## Complexity
             * - `O(C + S)` where `C` length of `code` and `S` complexity of `can_set_code`
             **/
            setCode: AugmentedSubmittable<
                (code: Bytes | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [Bytes]
            >;
            /**
             * Set the new runtime code without doing any checks of the given `code`.
             *
             * ## Complexity
             * - `O(C)` where `C` length of `code`
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
             * ## Complexity
             * - `O(1)` (Note that implementations of `OnTimestampSet` must also be `O(1)`)
             * - 1 storage read and 1 storage mutation (codec `O(1)`). (because of `DidUpdate::take` in
             * `on_finalize`)
             * - 1 event handler `on_timestamp_set`. Must be `O(1)`.
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
        voterList: {
            /**
             * Move the caller's Id directly in front of `lighter`.
             *
             * The dispatch origin for this call must be _Signed_ and can only be called by the Id of
             * the account going in front of `lighter`.
             *
             * Only works if
             * - both nodes are within the same bag,
             * - and `origin` has a greater `Score` than `lighter`.
             **/
            putInFrontOf: AugmentedSubmittable<
                (
                    lighter:
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
             * Declare that some `dislocated` account has, through rewards or penalties, sufficiently
             * changed its score that it should properly fall into a different bag than its current
             * one.
             *
             * Anyone can call this function about any potentially dislocated account.
             *
             * Will always update the stored score of `dislocated` to the correct score, based on
             * `ScoreProvider`.
             *
             * If `dislocated` does not exists, it returns an error.
             **/
            rebag: AugmentedSubmittable<
                (
                    dislocated:
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
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
    } // AugmentedSubmittables
} // declare module
