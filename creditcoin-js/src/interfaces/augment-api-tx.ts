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
import type { Data } from '@polkadot/types';
import type {
    Bytes,
    Compact,
    Null,
    Option,
    U256,
    U8aFixed,
    Vec,
    bool,
    u128,
    u16,
    u32,
    u64,
    u8,
} from '@polkadot/types-codec';
import type { AnyNumber, IMethod, ITuple } from '@polkadot/types-codec/types';
import type { AccountId32, Call, H256, MultiAddress, Perbill, Percent } from '@polkadot/types/interfaces/runtime';
import type {
    CreditcoinNodeRuntimeOpaqueSessionKeys,
    CreditcoinNodeRuntimeOriginCaller,
    PalletCreditcoinAskOrderId,
    PalletCreditcoinBidOrderId,
    PalletCreditcoinBlockchain,
    PalletCreditcoinDealOrderId,
    PalletCreditcoinLoanTerms,
    PalletCreditcoinOcwErrorsVerificationFailureCause,
    PalletCreditcoinOfferId,
    PalletCreditcoinOwnershipProof,
    PalletCreditcoinTaskId,
    PalletCreditcoinTaskOutput,
    PalletCreditcoinTransferKind,
    PalletIdentityBitFlags,
    PalletIdentityIdentityInfo,
    PalletIdentityJudgement,
    PalletImOnlineHeartbeat,
    PalletImOnlineSr25519AppSr25519Signature,
    PalletNominationPoolsBondExtra,
    PalletNominationPoolsClaimPermission,
    PalletNominationPoolsCommissionChangeRate,
    PalletNominationPoolsConfigOpAccountId32,
    PalletNominationPoolsConfigOpPerbill,
    PalletNominationPoolsConfigOpU128,
    PalletNominationPoolsConfigOpU32,
    PalletNominationPoolsPoolState,
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
                    taskId: PalletCreditcoinTaskId | { VerifyTransfer: any } | string | Uint8Array,
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
                        | 'InsufficientFaucetBalance'
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
                    taskOutput: PalletCreditcoinTaskOutput | { VerifyTransfer: any } | string | Uint8Array,
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
            /**
             * Registers an address on an external blockchain as the property of an onchain address.
             * To prove ownership, a signature is provided. To create the signature, the public key of the external address is used to sign a hash of the account_id of whoever is submitting this transaction.
             * The signature type allows the caller to specify if this address was signed using the older an insecure EthSign method or the new PersonalSign method. See here for details https://docs.metamask.io/wallet/how-to/sign-data/
             **/
            registerAddressV2: AugmentedSubmittable<
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
                    ownershipProof:
                        | PalletCreditcoinOwnershipProof
                        | { PersonalSign: any }
                        | { EthSign: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletCreditcoinBlockchain, Bytes, PalletCreditcoinOwnershipProof]
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
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        fastUnstake: {
            /**
             * Control the operation of this pallet.
             *
             * Dispatch origin must be signed by the [`Config::ControlOrigin`].
             **/
            control: AugmentedSubmittable<
                (erasToCheck: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Deregister oneself from the fast-unstake.
             *
             * This is useful if one is registered, they are still waiting, and they change their mind.
             *
             * Note that the associated stash is still fully unbonded and chilled as a consequence of
             * calling `register_fast_unstake`. This should probably be followed by a call to
             * `Staking::rebond`.
             **/
            deregister: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Register oneself for fast-unstake.
             *
             * The dispatch origin of this call must be signed by the controller account, similar to
             * `staking::unbond`.
             *
             * The stash associated with the origin must have no ongoing unlocking chunks. If
             * successful, this will fully unbond and chill the stash. Then, it will enqueue the stash
             * to be checked in further blocks.
             *
             * If by the time this is called, the stash is actually eligible for fast-unstake, then
             * they are guaranteed to remain eligible, because the call will chill them as well.
             *
             * If the check works, the entire staking data is removed, i.e. the stash is fully
             * unstaked.
             *
             * If the check fails, the stash remains chilled and waiting for being unbonded as in with
             * the normal staking system, but they lose part of their unbonding chunks due to consuming
             * the chain's resources.
             **/
            registerFastUnstake: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
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
        identity: {
            /**
             * Add a registrar to the system.
             *
             * The dispatch origin for this call must be `T::RegistrarOrigin`.
             *
             * - `account`: the account of the registrar.
             *
             * Emits `RegistrarAdded` if successful.
             *
             * ## Complexity
             * - `O(R)` where `R` registrar-count (governance-bounded and code-bounded).
             **/
            addRegistrar: AugmentedSubmittable<
                (
                    account:
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
             * Add the given account to the sender's subs.
             *
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated
             * to the sender.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * sub identity of `sub`.
             **/
            addSub: AugmentedSubmittable<
                (
                    sub:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    data:
                        | Data
                        | { None: any }
                        | { Raw: any }
                        | { BlakeTwo256: any }
                        | { Sha256: any }
                        | { Keccak256: any }
                        | { ShaThree256: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Data]
            >;
            /**
             * Cancel a previous request.
             *
             * Payment: A previously reserved deposit is returned on success.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a
             * registered identity.
             *
             * - `reg_index`: The index of the registrar whose judgement is no longer requested.
             *
             * Emits `JudgementUnrequested` if successful.
             *
             * ## Complexity
             * - `O(R + X)`.
             * - where `R` registrar-count (governance-bounded).
             * - where `X` additional-field-count (deposit-bounded and code-bounded).
             **/
            cancelRequest: AugmentedSubmittable<
                (regIndex: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * Clear an account's identity info and all sub-accounts and return all deposits.
             *
             * Payment: All reserved balances on the account are returned.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * identity.
             *
             * Emits `IdentityCleared` if successful.
             *
             * ## Complexity
             * - `O(R + S + X)`
             * - where `R` registrar-count (governance-bounded).
             * - where `S` subs-count (hard- and deposit-bounded).
             * - where `X` additional-field-count (deposit-bounded and code-bounded).
             **/
            clearIdentity: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Remove an account's identity and sub-account information and slash the deposits.
             *
             * Payment: Reserved balances from `set_subs` and `set_identity` are slashed and handled by
             * `Slash`. Verification request deposits are not returned; they should be cancelled
             * manually using `cancel_request`.
             *
             * The dispatch origin for this call must match `T::ForceOrigin`.
             *
             * - `target`: the account whose identity the judgement is upon. This must be an account
             * with a registered identity.
             *
             * Emits `IdentityKilled` if successful.
             *
             * ## Complexity
             * - `O(R + S + X)`
             * - where `R` registrar-count (governance-bounded).
             * - where `S` subs-count (hard- and deposit-bounded).
             * - where `X` additional-field-count (deposit-bounded and code-bounded).
             **/
            killIdentity: AugmentedSubmittable<
                (
                    target:
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
             * Provide a judgement for an account's identity.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `reg_index`.
             *
             * - `reg_index`: the index of the registrar whose judgement is being made.
             * - `target`: the account whose identity the judgement is upon. This must be an account
             * with a registered identity.
             * - `judgement`: the judgement of the registrar of index `reg_index` about `target`.
             * - `identity`: The hash of the [`IdentityInfo`] for that the judgement is provided.
             *
             * Emits `JudgementGiven` if successful.
             *
             * ## Complexity
             * - `O(R + X)`.
             * - where `R` registrar-count (governance-bounded).
             * - where `X` additional-field-count (deposit-bounded and code-bounded).
             **/
            provideJudgement: AugmentedSubmittable<
                (
                    regIndex: Compact<u32> | AnyNumber | Uint8Array,
                    target:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    judgement:
                        | PalletIdentityJudgement
                        | { Unknown: any }
                        | { FeePaid: any }
                        | { Reasonable: any }
                        | { KnownGood: any }
                        | { OutOfDate: any }
                        | { LowQuality: any }
                        | { Erroneous: any }
                        | string
                        | Uint8Array,
                    identity: H256 | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, MultiAddress, PalletIdentityJudgement, H256]
            >;
            /**
             * Remove the sender as a sub-account.
             *
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated
             * to the sender (*not* the original depositor).
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * super-identity.
             *
             * NOTE: This should not normally be used, but is provided in the case that the non-
             * controller of an account is maliciously registered as a sub-account.
             **/
            quitSub: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Remove the given account from the sender's subs.
             *
             * Payment: Balance reserved by a previous `set_subs` call for one sub will be repatriated
             * to the sender.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * sub identity of `sub`.
             **/
            removeSub: AugmentedSubmittable<
                (
                    sub:
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
             * Alter the associated name of the given sub-account.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * sub identity of `sub`.
             **/
            renameSub: AugmentedSubmittable<
                (
                    sub:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    data:
                        | Data
                        | { None: any }
                        | { Raw: any }
                        | { BlakeTwo256: any }
                        | { Sha256: any }
                        | { Keccak256: any }
                        | { ShaThree256: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Data]
            >;
            /**
             * Request a judgement from a registrar.
             *
             * Payment: At most `max_fee` will be reserved for payment to the registrar if judgement
             * given.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a
             * registered identity.
             *
             * - `reg_index`: The index of the registrar whose judgement is requested.
             * - `max_fee`: The maximum fee that may be paid. This should just be auto-populated as:
             *
             * ```nocompile
             * Self::registrars().get(reg_index).unwrap().fee
             * ```
             *
             * Emits `JudgementRequested` if successful.
             *
             * ## Complexity
             * - `O(R + X)`.
             * - where `R` registrar-count (governance-bounded).
             * - where `X` additional-field-count (deposit-bounded and code-bounded).
             **/
            requestJudgement: AugmentedSubmittable<
                (
                    regIndex: Compact<u32> | AnyNumber | Uint8Array,
                    maxFee: Compact<u128> | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, Compact<u128>]
            >;
            /**
             * Change the account associated with a registrar.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `new`: the new account ID.
             *
             * ## Complexity
             * - `O(R)`.
             * - where `R` registrar-count (governance-bounded).
             **/
            setAccountId: AugmentedSubmittable<
                (
                    index: Compact<u32> | AnyNumber | Uint8Array,
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
                [Compact<u32>, MultiAddress]
            >;
            /**
             * Set the fee required for a judgement to be requested from a registrar.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `fee`: the new fee.
             *
             * ## Complexity
             * - `O(R)`.
             * - where `R` registrar-count (governance-bounded).
             **/
            setFee: AugmentedSubmittable<
                (
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    fee: Compact<u128> | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, Compact<u128>]
            >;
            /**
             * Set the field information for a registrar.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must be the account
             * of the registrar whose index is `index`.
             *
             * - `index`: the index of the registrar whose fee is to be set.
             * - `fields`: the fields that the registrar concerns themselves with.
             *
             * ## Complexity
             * - `O(R)`.
             * - where `R` registrar-count (governance-bounded).
             **/
            setFields: AugmentedSubmittable<
                (
                    index: Compact<u32> | AnyNumber | Uint8Array,
                    fields: PalletIdentityBitFlags,
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u32>, PalletIdentityBitFlags]
            >;
            /**
             * Set an account's identity information and reserve the appropriate deposit.
             *
             * If the account already has identity information, the deposit is taken as part payment
             * for the new deposit.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * - `info`: The identity information.
             *
             * Emits `IdentitySet` if successful.
             *
             * ## Complexity
             * - `O(X + X' + R)`
             * - where `X` additional-field-count (deposit-bounded and code-bounded)
             * - where `R` judgements-count (registrar-count-bounded)
             **/
            setIdentity: AugmentedSubmittable<
                (
                    info:
                        | PalletIdentityIdentityInfo
                        | {
                              additional?: any;
                              display?: any;
                              legal?: any;
                              web?: any;
                              riot?: any;
                              email?: any;
                              pgpFingerprint?: any;
                              image?: any;
                              twitter?: any;
                          }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletIdentityIdentityInfo]
            >;
            /**
             * Set the sub-accounts of the sender.
             *
             * Payment: Any aggregate balance reserved by previous `set_subs` calls will be returned
             * and an amount `SubAccountDeposit` will be reserved for each item in `subs`.
             *
             * The dispatch origin for this call must be _Signed_ and the sender must have a registered
             * identity.
             *
             * - `subs`: The identity's (new) sub-accounts.
             *
             * ## Complexity
             * - `O(P + S)`
             * - where `P` old-subs-count (hard- and deposit-bounded).
             * - where `S` subs-count (hard- and deposit-bounded).
             **/
            setSubs: AugmentedSubmittable<
                (
                    subs:
                        | Vec<ITuple<[AccountId32, Data]>>
                        | [
                              AccountId32 | string | Uint8Array,
                              (
                                  | Data
                                  | { None: any }
                                  | { Raw: any }
                                  | { BlakeTwo256: any }
                                  | { Sha256: any }
                                  | { Keccak256: any }
                                  | { ShaThree256: any }
                                  | string
                                  | Uint8Array
                              ),
                          ][],
                ) => SubmittableExtrinsic<ApiType>,
                [Vec<ITuple<[AccountId32, Data]>>]
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
        nominationPools: {
            /**
             * Bond `extra` more funds from `origin` into the pool to which they already belong.
             *
             * Additional funds can come from either the free balance of the account, of from the
             * accumulated rewards, see [`BondExtra`].
             *
             * Bonding extra funds implies an automatic payout of all pending rewards as well.
             * See `bond_extra_other` to bond pending rewards of `other` members.
             **/
            bondExtra: AugmentedSubmittable<
                (
                    extra:
                        | PalletNominationPoolsBondExtra
                        | { FreeBalance: any }
                        | { Rewards: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletNominationPoolsBondExtra]
            >;
            /**
             * `origin` bonds funds from `extra` for some pool member `member` into their respective
             * pools.
             *
             * `origin` can bond extra funds from free balance or pending rewards when `origin ==
             * other`.
             *
             * In the case of `origin != other`, `origin` can only bond extra pending rewards of
             * `other` members assuming set_claim_permission for the given member is
             * `PermissionlessAll` or `PermissionlessCompound`.
             **/
            bondExtraOther: AugmentedSubmittable<
                (
                    member:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    extra:
                        | PalletNominationPoolsBondExtra
                        | { FreeBalance: any }
                        | { Rewards: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, PalletNominationPoolsBondExtra]
            >;
            /**
             * Chill on behalf of the pool.
             *
             * The dispatch origin of this call must be signed by the pool nominator or the pool
             * root role, same as [`Pallet::nominate`].
             *
             * This directly forward the call to the staking pallet, on behalf of the pool bonded
             * account.
             **/
            chill: AugmentedSubmittable<(poolId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>, [u32]>;
            /**
             * Claim pending commission.
             *
             * The dispatch origin of this call must be signed by the `root` role of the pool. Pending
             * commission is paid out and added to total claimed commission`. Total pending commission
             * is reset to zero. the current.
             **/
            claimCommission: AugmentedSubmittable<
                (poolId: u32 | AnyNumber | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [u32]
            >;
            /**
             * A bonded member can use this to claim their payout based on the rewards that the pool
             * has accumulated since their last claimed payout (OR since joining if this is their first
             * time claiming rewards). The payout will be transferred to the member's account.
             *
             * The member will earn rewards pro rata based on the members stake vs the sum of the
             * members in the pools stake. Rewards do not "expire".
             *
             * See `claim_payout_other` to caim rewards on bahalf of some `other` pool member.
             **/
            claimPayout: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * `origin` can claim payouts on some pool member `other`'s behalf.
             *
             * Pool member `other` must have a `PermissionlessAll` or `PermissionlessWithdraw` in order
             * for this call to be successful.
             **/
            claimPayoutOther: AugmentedSubmittable<
                (other: AccountId32 | string | Uint8Array) => SubmittableExtrinsic<ApiType>,
                [AccountId32]
            >;
            /**
             * Create a new delegation pool.
             *
             * # Arguments
             *
             * * `amount` - The amount of funds to delegate to the pool. This also acts of a sort of
             * deposit since the pools creator cannot fully unbond funds until the pool is being
             * destroyed.
             * * `index` - A disambiguation index for creating the account. Likely only useful when
             * creating multiple pools in the same extrinsic.
             * * `root` - The account to set as [`PoolRoles::root`].
             * * `nominator` - The account to set as the [`PoolRoles::nominator`].
             * * `bouncer` - The account to set as the [`PoolRoles::bouncer`].
             *
             * # Note
             *
             * In addition to `amount`, the caller will transfer the existential deposit; so the caller
             * needs at have at least `amount + existential_deposit` transferrable.
             **/
            create: AugmentedSubmittable<
                (
                    amount: Compact<u128> | AnyNumber | Uint8Array,
                    root:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    nominator:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    bouncer:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u128>, MultiAddress, MultiAddress, MultiAddress]
            >;
            /**
             * Create a new delegation pool with a previously used pool id
             *
             * # Arguments
             *
             * same as `create` with the inclusion of
             * * `pool_id` - `A valid PoolId.
             **/
            createWithPoolId: AugmentedSubmittable<
                (
                    amount: Compact<u128> | AnyNumber | Uint8Array,
                    root:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    nominator:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    bouncer:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    poolId: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u128>, MultiAddress, MultiAddress, MultiAddress, u32]
            >;
            /**
             * Stake funds with a pool. The amount to bond is transferred from the member to the
             * pools account and immediately increases the pools bond.
             *
             * # Note
             *
             * * An account can only be a member of a single pool.
             * * An account cannot join the same pool multiple times.
             * * This call will *not* dust the member account, so the member must have at least
             * `existential deposit + amount` in their account.
             * * Only a pool with [`PoolState::Open`] can be joined
             **/
            join: AugmentedSubmittable<
                (
                    amount: Compact<u128> | AnyNumber | Uint8Array,
                    poolId: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [Compact<u128>, u32]
            >;
            /**
             * Nominate on behalf of the pool.
             *
             * The dispatch origin of this call must be signed by the pool nominator or the pool
             * root role.
             *
             * This directly forward the call to the staking pallet, on behalf of the pool bonded
             * account.
             **/
            nominate: AugmentedSubmittable<
                (
                    poolId: u32 | AnyNumber | Uint8Array,
                    validators: Vec<AccountId32> | (AccountId32 | string | Uint8Array)[],
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Vec<AccountId32>]
            >;
            /**
             * Call `withdraw_unbonded` for the pools account. This call can be made by any account.
             *
             * This is useful if their are too many unlocking chunks to call `unbond`, and some
             * can be cleared by withdrawing. In the case there are too many unlocking chunks, the user
             * would probably see an error like `NoMoreChunks` emitted from the staking system when
             * they attempt to unbond.
             **/
            poolWithdrawUnbonded: AugmentedSubmittable<
                (
                    poolId: u32 | AnyNumber | Uint8Array,
                    numSlashingSpans: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u32, u32]
            >;
            /**
             * Allows a pool member to set a claim permission to allow or disallow permissionless
             * bonding and withdrawing.
             *
             * By default, this is `Permissioned`, which implies only the pool member themselves can
             * claim their pending rewards. If a pool member wishes so, they can set this to
             * `PermissionlessAll` to allow any account to claim their rewards and bond extra to the
             * pool.
             *
             * # Arguments
             *
             * * `origin` - Member of a pool.
             * * `actor` - Account to claim reward. // improve this
             **/
            setClaimPermission: AugmentedSubmittable<
                (
                    permission:
                        | PalletNominationPoolsClaimPermission
                        | 'Permissioned'
                        | 'PermissionlessCompound'
                        | 'PermissionlessWithdraw'
                        | 'PermissionlessAll'
                        | number
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [PalletNominationPoolsClaimPermission]
            >;
            /**
             * Set the commission of a pool.
             * Both a commission percentage and a commission payee must be provided in the `current`
             * tuple. Where a `current` of `None` is provided, any current commission will be removed.
             *
             * - If a `None` is supplied to `new_commission`, existing commission will be removed.
             **/
            setCommission: AugmentedSubmittable<
                (
                    poolId: u32 | AnyNumber | Uint8Array,
                    newCommission:
                        | Option<ITuple<[Perbill, AccountId32]>>
                        | null
                        | Uint8Array
                        | ITuple<[Perbill, AccountId32]>
                        | [Perbill | AnyNumber | Uint8Array, AccountId32 | string | Uint8Array],
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Option<ITuple<[Perbill, AccountId32]>>]
            >;
            /**
             * Set the commission change rate for a pool.
             *
             * Initial change rate is not bounded, whereas subsequent updates can only be more
             * restrictive than the current.
             **/
            setCommissionChangeRate: AugmentedSubmittable<
                (
                    poolId: u32 | AnyNumber | Uint8Array,
                    changeRate:
                        | PalletNominationPoolsCommissionChangeRate
                        | { maxIncrease?: any; minDelay?: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u32, PalletNominationPoolsCommissionChangeRate]
            >;
            /**
             * Set the maximum commission of a pool.
             *
             * - Initial max can be set to any `Perbill`, and only smaller values thereafter.
             * - Current commission will be lowered in the event it is higher than a new max
             * commission.
             **/
            setCommissionMax: AugmentedSubmittable<
                (
                    poolId: u32 | AnyNumber | Uint8Array,
                    maxCommission: Perbill | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Perbill]
            >;
            /**
             * Update configurations for the nomination pools. The origin for this call must be
             * Root.
             *
             * # Arguments
             *
             * * `min_join_bond` - Set [`MinJoinBond`].
             * * `min_create_bond` - Set [`MinCreateBond`].
             * * `max_pools` - Set [`MaxPools`].
             * * `max_members` - Set [`MaxPoolMembers`].
             * * `max_members_per_pool` - Set [`MaxPoolMembersPerPool`].
             * * `global_max_commission` - Set [`GlobalMaxCommission`].
             **/
            setConfigs: AugmentedSubmittable<
                (
                    minJoinBond:
                        | PalletNominationPoolsConfigOpU128
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    minCreateBond:
                        | PalletNominationPoolsConfigOpU128
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    maxPools:
                        | PalletNominationPoolsConfigOpU32
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    maxMembers:
                        | PalletNominationPoolsConfigOpU32
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    maxMembersPerPool:
                        | PalletNominationPoolsConfigOpU32
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    globalMaxCommission:
                        | PalletNominationPoolsConfigOpPerbill
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [
                    PalletNominationPoolsConfigOpU128,
                    PalletNominationPoolsConfigOpU128,
                    PalletNominationPoolsConfigOpU32,
                    PalletNominationPoolsConfigOpU32,
                    PalletNominationPoolsConfigOpU32,
                    PalletNominationPoolsConfigOpPerbill,
                ]
            >;
            /**
             * Set a new metadata for the pool.
             *
             * The dispatch origin of this call must be signed by the bouncer, or the root role of the
             * pool.
             **/
            setMetadata: AugmentedSubmittable<
                (
                    poolId: u32 | AnyNumber | Uint8Array,
                    metadata: Bytes | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u32, Bytes]
            >;
            /**
             * Set a new state for the pool.
             *
             * If a pool is already in the `Destroying` state, then under no condition can its state
             * change again.
             *
             * The dispatch origin of this call must be either:
             *
             * 1. signed by the bouncer, or the root role of the pool,
             * 2. if the pool conditions to be open are NOT met (as described by `ok_to_be_open`), and
             * then the state of the pool can be permissionlessly changed to `Destroying`.
             **/
            setState: AugmentedSubmittable<
                (
                    poolId: u32 | AnyNumber | Uint8Array,
                    state: PalletNominationPoolsPoolState | 'Open' | 'Blocked' | 'Destroying' | number | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u32, PalletNominationPoolsPoolState]
            >;
            /**
             * Unbond up to `unbonding_points` of the `member_account`'s funds from the pool. It
             * implicitly collects the rewards one last time, since not doing so would mean some
             * rewards would be forfeited.
             *
             * Under certain conditions, this call can be dispatched permissionlessly (i.e. by any
             * account).
             *
             * # Conditions for a permissionless dispatch.
             *
             * * The pool is blocked and the caller is either the root or bouncer. This is refereed to
             * as a kick.
             * * The pool is destroying and the member is not the depositor.
             * * The pool is destroying, the member is the depositor and no other members are in the
             * pool.
             *
             * ## Conditions for permissioned dispatch (i.e. the caller is also the
             * `member_account`):
             *
             * * The caller is not the depositor.
             * * The caller is the depositor, the pool is destroying and no other members are in the
             * pool.
             *
             * # Note
             *
             * If there are too many unlocking chunks to unbond with the pool account,
             * [`Call::pool_withdraw_unbonded`] can be called to try and minimize unlocking chunks.
             * The [`StakingInterface::unbond`] will implicitly call [`Call::pool_withdraw_unbonded`]
             * to try to free chunks if necessary (ie. if unbound was called and no unlocking chunks
             * are available). However, it may not be possible to release the current unlocking chunks,
             * in which case, the result of this call will likely be the `NoMoreChunks` error from the
             * staking system.
             **/
            unbond: AugmentedSubmittable<
                (
                    memberAccount:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    unbondingPoints: Compact<u128> | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Compact<u128>]
            >;
            /**
             * Update the roles of the pool.
             *
             * The root is the only entity that can change any of the roles, including itself,
             * excluding the depositor, who can never change.
             *
             * It emits an event, notifying UIs of the role change. This event is quite relevant to
             * most pool members and they should be informed of changes to pool roles.
             **/
            updateRoles: AugmentedSubmittable<
                (
                    poolId: u32 | AnyNumber | Uint8Array,
                    newRoot:
                        | PalletNominationPoolsConfigOpAccountId32
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    newNominator:
                        | PalletNominationPoolsConfigOpAccountId32
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                    newBouncer:
                        | PalletNominationPoolsConfigOpAccountId32
                        | { Noop: any }
                        | { Set: any }
                        | { Remove: any }
                        | string
                        | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [
                    u32,
                    PalletNominationPoolsConfigOpAccountId32,
                    PalletNominationPoolsConfigOpAccountId32,
                    PalletNominationPoolsConfigOpAccountId32,
                ]
            >;
            /**
             * Withdraw unbonded funds from `member_account`. If no bonded funds can be unbonded, an
             * error is returned.
             *
             * Under certain conditions, this call can be dispatched permissionlessly (i.e. by any
             * account).
             *
             * # Conditions for a permissionless dispatch
             *
             * * The pool is in destroy mode and the target is not the depositor.
             * * The target is the depositor and they are the only member in the sub pools.
             * * The pool is blocked and the caller is either the root or bouncer.
             *
             * # Conditions for permissioned dispatch
             *
             * * The caller is the target and they are not the depositor.
             *
             * # Note
             *
             * If the target is the depositor, the pool will be destroyed.
             **/
            withdrawUnbonded: AugmentedSubmittable<
                (
                    memberAccount:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    numSlashingSpans: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, u32]
            >;
            /**
             * Generic tx
             **/
            [key: string]: SubmittableExtrinsicFunction<ApiType>;
        };
        proxy: {
            /**
             * Register a proxy account for the sender that is able to make calls on its behalf.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `proxy`: The account that the `caller` would like to make a proxy.
             * - `proxy_type`: The permissions allowed for this proxy account.
             * - `delay`: The announcement period required of the initial proxy. Will generally be
             * zero.
             **/
            addProxy: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    proxyType: Null | null,
                    delay: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Null, u32]
            >;
            /**
             * Publish the hash of a proxy-call that will be made in the future.
             *
             * This must be called some number of blocks before the corresponding `proxy` is attempted
             * if the delay associated with the proxy relationship is greater than zero.
             *
             * No more than `MaxPending` announcements may be made at any one time.
             *
             * This will take a deposit of `AnnouncementDepositFactor` as well as
             * `AnnouncementDepositBase` if there are no other pending announcements.
             *
             * The dispatch origin for this call must be _Signed_ and a proxy of `real`.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `call_hash`: The hash of the call to be made by the `real` account.
             **/
            announce: AugmentedSubmittable<
                (
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    callHash: H256 | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, H256]
            >;
            /**
             * Spawn a fresh new account that is guaranteed to be otherwise inaccessible, and
             * initialize it with a proxy of `proxy_type` for `origin` sender.
             *
             * Requires a `Signed` origin.
             *
             * - `proxy_type`: The type of the proxy that the sender will be registered as over the
             * new account. This will almost always be the most permissive `ProxyType` possible to
             * allow for maximum flexibility.
             * - `index`: A disambiguation index, in case this is called multiple times in the same
             * transaction (e.g. with `utility::batch`). Unless you're using `batch` you probably just
             * want to use `0`.
             * - `delay`: The announcement period required of the initial proxy. Will generally be
             * zero.
             *
             * Fails with `Duplicate` if this has already been called in this transaction, from the
             * same sender, with the same parameters.
             *
             * Fails if there are insufficient funds to pay for deposit.
             **/
            createPure: AugmentedSubmittable<
                (
                    proxyType: Null | null,
                    delay: u32 | AnyNumber | Uint8Array,
                    index: u16 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [Null, u32, u16]
            >;
            /**
             * Removes a previously spawned pure proxy.
             *
             * WARNING: **All access to this account will be lost.** Any funds held in it will be
             * inaccessible.
             *
             * Requires a `Signed` origin, and the sender account must have been created by a call to
             * `pure` with corresponding parameters.
             *
             * - `spawner`: The account that originally called `pure` to create this account.
             * - `index`: The disambiguation index originally passed to `pure`. Probably `0`.
             * - `proxy_type`: The proxy type originally passed to `pure`.
             * - `height`: The height of the chain when the call to `pure` was processed.
             * - `ext_index`: The extrinsic index in which the call to `pure` was processed.
             *
             * Fails with `NoPermission` in case the caller is not a previously created pure
             * account whose `pure` call has corresponding parameters.
             **/
            killPure: AugmentedSubmittable<
                (
                    spawner:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    proxyType: Null | null,
                    index: u16 | AnyNumber | Uint8Array,
                    height: Compact<u32> | AnyNumber | Uint8Array,
                    extIndex: Compact<u32> | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Null, u16, Compact<u32>, Compact<u32>]
            >;
            /**
             * Dispatch the given `call` from an account that the sender is authorised for through
             * `add_proxy`.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `force_proxy_type`: Specify the exact proxy type to be used and checked for this call.
             * - `call`: The call to be made by the `real` account.
             **/
            proxy: AugmentedSubmittable<
                (
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    forceProxyType: Option<Null> | null | Uint8Array | Null,
                    call: Call | IMethod | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Option<Null>, Call]
            >;
            /**
             * Dispatch the given `call` from an account that the sender is authorized for through
             * `add_proxy`.
             *
             * Removes any corresponding announcement(s).
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `force_proxy_type`: Specify the exact proxy type to be used and checked for this call.
             * - `call`: The call to be made by the `real` account.
             **/
            proxyAnnounced: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    forceProxyType: Option<Null> | null | Uint8Array | Null,
                    call: Call | IMethod | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, MultiAddress, Option<Null>, Call]
            >;
            /**
             * Remove the given announcement of a delegate.
             *
             * May be called by a target (proxied) account to remove a call that one of their delegates
             * (`delegate`) has announced they want to execute. The deposit is returned.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `delegate`: The account that previously announced the call.
             * - `call_hash`: The hash of the call to be made.
             **/
            rejectAnnouncement: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    callHash: H256 | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, H256]
            >;
            /**
             * Remove a given announcement.
             *
             * May be called by a proxy account to remove a call they previously announced and return
             * the deposit.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `real`: The account that the proxy will make a call on behalf of.
             * - `call_hash`: The hash of the call to be made by the `real` account.
             **/
            removeAnnouncement: AugmentedSubmittable<
                (
                    real:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    callHash: H256 | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, H256]
            >;
            /**
             * Unregister all proxy accounts for the sender.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * WARNING: This may be called on accounts created by `pure`, however if done, then
             * the unreserved fees will be inaccessible. **All access to this account will be lost.**
             **/
            removeProxies: AugmentedSubmittable<() => SubmittableExtrinsic<ApiType>, []>;
            /**
             * Unregister a proxy account for the sender.
             *
             * The dispatch origin for this call must be _Signed_.
             *
             * Parameters:
             * - `proxy`: The account that the `caller` would like to remove as a proxy.
             * - `proxy_type`: The permissions currently enabled for the removed proxy account.
             **/
            removeProxy: AugmentedSubmittable<
                (
                    delegate:
                        | MultiAddress
                        | { Id: any }
                        | { Index: any }
                        | { Raw: any }
                        | { Address32: any }
                        | { Address20: any }
                        | string
                        | Uint8Array,
                    proxyType: Null | null,
                    delay: u32 | AnyNumber | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [MultiAddress, Null, u32]
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
        utility: {
            /**
             * Send a call through an indexed pseudonym of the sender.
             *
             * Filter from origin are passed along. The call will be dispatched with an origin which
             * use the same filter as the origin of this call.
             *
             * NOTE: If you need to ensure that any account-based filtering is not honored (i.e.
             * because you expect `proxy` to have been used prior in the call stack and you do not want
             * the call restrictions to apply to any sub-accounts), then use `as_multi_threshold_1`
             * in the Multisig pallet instead.
             *
             * NOTE: Prior to version *12, this was called `as_limited_sub`.
             *
             * The dispatch origin for this call must be _Signed_.
             **/
            asDerivative: AugmentedSubmittable<
                (
                    index: u16 | AnyNumber | Uint8Array,
                    call: Call | IMethod | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [u16, Call]
            >;
            /**
             * Send a batch of dispatch calls.
             *
             * May be called from any origin except `None`.
             *
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not
             * exceed the constant: `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatched without checking origin filter. (This
             * includes bypassing `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             * - O(C) where C is the number of calls to be batched.
             *
             * This will return `Ok` in all circumstances. To determine the success of the batch, an
             * event is deposited. If a call failed and the batch was interrupted, then the
             * `BatchInterrupted` event is deposited, along with the number of successful calls made
             * and the error of the failed call. If all were successful, then the `BatchCompleted`
             * event is deposited.
             **/
            batch: AugmentedSubmittable<
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /**
             * Send a batch of dispatch calls and atomically execute them.
             * The whole transaction will rollback and fail if any of the calls failed.
             *
             * May be called from any origin except `None`.
             *
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not
             * exceed the constant: `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatched without checking origin filter. (This
             * includes bypassing `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             * - O(C) where C is the number of calls to be batched.
             **/
            batchAll: AugmentedSubmittable<
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /**
             * Dispatches a function call with a provided origin.
             *
             * The dispatch origin for this call must be _Root_.
             *
             * ## Complexity
             * - O(1).
             **/
            dispatchAs: AugmentedSubmittable<
                (
                    asOrigin: CreditcoinNodeRuntimeOriginCaller | { system: any } | { Void: any } | string | Uint8Array,
                    call: Call | IMethod | string | Uint8Array,
                ) => SubmittableExtrinsic<ApiType>,
                [CreditcoinNodeRuntimeOriginCaller, Call]
            >;
            /**
             * Send a batch of dispatch calls.
             * Unlike `batch`, it allows errors and won't interrupt.
             *
             * May be called from any origin except `None`.
             *
             * - `calls`: The calls to be dispatched from the same origin. The number of call must not
             * exceed the constant: `batched_calls_limit` (available in constant metadata).
             *
             * If origin is root then the calls are dispatch without checking origin filter. (This
             * includes bypassing `frame_system::Config::BaseCallFilter`).
             *
             * ## Complexity
             * - O(C) where C is the number of calls to be batched.
             **/
            forceBatch: AugmentedSubmittable<
                (calls: Vec<Call> | (Call | IMethod | string | Uint8Array)[]) => SubmittableExtrinsic<ApiType>,
                [Vec<Call>]
            >;
            /**
             * Dispatch a function call with a specified weight.
             *
             * This function does not check the weight of the call, and instead allows the
             * Root origin to specify the weight of the call.
             *
             * The dispatch origin for this call must be _Root_.
             **/
            withWeight: AugmentedSubmittable<
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
