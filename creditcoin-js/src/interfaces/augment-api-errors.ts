// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import '@polkadot/api-base/types/errors';

import type { ApiTypes, AugmentedError } from '@polkadot/api-base/types';

export type __AugmentedError<ApiType extends ApiTypes> = AugmentedError<ApiType>;

declare module '@polkadot/api-base/types/errors' {
    interface AugmentedErrors<ApiType extends ApiTypes> {
        babe: {
            /**
             * A given equivocation report is valid but already previously reported.
             **/
            DuplicateOffenceReport: AugmentedError<ApiType>;
            /**
             * Submitted configuration is invalid.
             **/
            InvalidConfiguration: AugmentedError<ApiType>;
            /**
             * An equivocation proof provided as part of an equivocation report is invalid.
             **/
            InvalidEquivocationProof: AugmentedError<ApiType>;
            /**
             * A key ownership proof provided as part of an equivocation report is invalid.
             **/
            InvalidKeyOwnershipProof: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
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
             * Balance too low to send value.
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
             * The specified address has already been registered to another account.
             **/
            AddressAlreadyRegistered: AugmentedError<ApiType>;
            /**
             * The specified address has already been registered to this account.
             **/
            AddressAlreadyRegisteredByCaller: AugmentedError<ApiType>;
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
             * The onchain faucet address for the GATE swap mechanism has not been set using the set_gate_faucet extrinsic
             **/
            BurnGATEFaucetNotSet: AugmentedError<ApiType>;
            /**
             * The faucet has insufficient funds to complete this swap, please retry when the faucet has been reloaded
             **/
            BurnGATEInsufficientFaucetBalance: AugmentedError<ApiType>;
            /**
             * The coin collection has already been registered.
             **/
            CollectCoinsAlreadyRegistered: AugmentedError<ApiType>;
            /**
             * The currency has already been registered.
             **/
            CurrencyAlreadyRegistered: AugmentedError<ApiType>;
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
             * A valid external address could not be generated for the specified blockchain and recovered public key
             **/
            EthSignExternalAddressGenerationFailed: AugmentedError<ApiType>;
            /**
             * ECDSA public key recovery failed for an ownership proof using EthSign
             **/
            EthSignPublicKeyRecoveryFailed: AugmentedError<ApiType>;
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
            /**
             * A valid external address could not be generated for the specified blockchain and recovered public key
             **/
            PersonalSignExternalAddressGenerationFailed: AugmentedError<ApiType>;
            /**
             * ECDSA public key recovery failed for an ownership proof using PersonalSign
             **/
            PersonalSignPublicKeyRecoveryFailed: AugmentedError<ApiType>;
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
             * An unsupported blockchain was specified to register_address_v2
             **/
            UnsupportedBlockchain: AugmentedError<ApiType>;
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
        fastUnstake: {
            /**
             * The provided un-staker is already in Head, and cannot deregister.
             **/
            AlreadyHead: AugmentedError<ApiType>;
            /**
             * The bonded account has already been queued.
             **/
            AlreadyQueued: AugmentedError<ApiType>;
            /**
             * The call is not allowed at this point because the pallet is not active.
             **/
            CallNotAllowed: AugmentedError<ApiType>;
            /**
             * The provided Controller account was not found.
             *
             * This means that the given account is not bonded.
             **/
            NotController: AugmentedError<ApiType>;
            /**
             * The bonded account has active unlocking chunks.
             **/
            NotFullyBonded: AugmentedError<ApiType>;
            /**
             * The provided un-staker is not in the `Queue`.
             **/
            NotQueued: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        grandpa: {
            /**
             * Attempt to signal GRANDPA change with one already pending.
             **/
            ChangePending: AugmentedError<ApiType>;
            /**
             * A given equivocation report is valid but already previously reported.
             **/
            DuplicateOffenceReport: AugmentedError<ApiType>;
            /**
             * An equivocation proof provided as part of an equivocation report is invalid.
             **/
            InvalidEquivocationProof: AugmentedError<ApiType>;
            /**
             * A key ownership proof provided as part of an equivocation report is invalid.
             **/
            InvalidKeyOwnershipProof: AugmentedError<ApiType>;
            /**
             * Attempt to signal GRANDPA pause when the authority set isn't live
             * (either paused or already pending pause).
             **/
            PauseFailed: AugmentedError<ApiType>;
            /**
             * Attempt to signal GRANDPA resume when the authority set isn't paused
             * (either live or already pending resume).
             **/
            ResumeFailed: AugmentedError<ApiType>;
            /**
             * Cannot signal forced change so soon after last.
             **/
            TooSoon: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        identity: {
            /**
             * Account ID is already named.
             **/
            AlreadyClaimed: AugmentedError<ApiType>;
            /**
             * Empty index.
             **/
            EmptyIndex: AugmentedError<ApiType>;
            /**
             * Fee is changed.
             **/
            FeeChanged: AugmentedError<ApiType>;
            /**
             * The index is invalid.
             **/
            InvalidIndex: AugmentedError<ApiType>;
            /**
             * Invalid judgement.
             **/
            InvalidJudgement: AugmentedError<ApiType>;
            /**
             * The target is invalid.
             **/
            InvalidTarget: AugmentedError<ApiType>;
            /**
             * The provided judgement was for a different identity.
             **/
            JudgementForDifferentIdentity: AugmentedError<ApiType>;
            /**
             * Judgement given.
             **/
            JudgementGiven: AugmentedError<ApiType>;
            /**
             * Error that occurs when there is an issue paying for judgement.
             **/
            JudgementPaymentFailed: AugmentedError<ApiType>;
            /**
             * No identity found.
             **/
            NoIdentity: AugmentedError<ApiType>;
            /**
             * Account isn't found.
             **/
            NotFound: AugmentedError<ApiType>;
            /**
             * Account isn't named.
             **/
            NotNamed: AugmentedError<ApiType>;
            /**
             * Sub-account isn't owned by sender.
             **/
            NotOwned: AugmentedError<ApiType>;
            /**
             * Sender is not a sub-account.
             **/
            NotSub: AugmentedError<ApiType>;
            /**
             * Sticky judgement.
             **/
            StickyJudgement: AugmentedError<ApiType>;
            /**
             * Too many additional fields.
             **/
            TooManyFields: AugmentedError<ApiType>;
            /**
             * Maximum amount of registrars reached. Cannot add any more.
             **/
            TooManyRegistrars: AugmentedError<ApiType>;
            /**
             * Too many subs-accounts.
             **/
            TooManySubAccounts: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        imOnline: {
            /**
             * Duplicated heartbeat.
             **/
            DuplicatedHeartbeat: AugmentedError<ApiType>;
            /**
             * Non existent public key.
             **/
            InvalidKey: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        nominationPools: {
            /**
             * An account is already delegating in another pool. An account may only belong to one
             * pool at a time.
             **/
            AccountBelongsToOtherPool: AugmentedError<ApiType>;
            /**
             * Bonding extra is restricted to the exact pending reward amount.
             **/
            BondExtraRestricted: AugmentedError<ApiType>;
            /**
             * The pools state cannot be changed.
             **/
            CanNotChangeState: AugmentedError<ApiType>;
            /**
             * None of the funds can be withdrawn yet because the bonding duration has not passed.
             **/
            CannotWithdrawAny: AugmentedError<ApiType>;
            /**
             * The submitted changes to commission change rate are not allowed.
             **/
            CommissionChangeRateNotAllowed: AugmentedError<ApiType>;
            /**
             * Not enough blocks have surpassed since the last commission update.
             **/
            CommissionChangeThrottled: AugmentedError<ApiType>;
            /**
             * The supplied commission exceeds the max allowed commission.
             **/
            CommissionExceedsMaximum: AugmentedError<ApiType>;
            /**
             * Some error occurred that should never happen. This should be reported to the
             * maintainers.
             **/
            Defensive: AugmentedError<ApiType>;
            /**
             * The caller does not have adequate permissions.
             **/
            DoesNotHavePermission: AugmentedError<ApiType>;
            /**
             * The member is fully unbonded (and thus cannot access the bonded and reward pool
             * anymore to, for example, collect rewards).
             **/
            FullyUnbonding: AugmentedError<ApiType>;
            /**
             * Pool id provided is not correct/usable.
             **/
            InvalidPoolId: AugmentedError<ApiType>;
            /**
             * The pool's max commission cannot be set higher than the existing value.
             **/
            MaxCommissionRestricted: AugmentedError<ApiType>;
            /**
             * Too many members in the pool or system.
             **/
            MaxPoolMembers: AugmentedError<ApiType>;
            /**
             * The system is maxed out on pools.
             **/
            MaxPools: AugmentedError<ApiType>;
            /**
             * The member cannot unbond further chunks due to reaching the limit.
             **/
            MaxUnbondingLimit: AugmentedError<ApiType>;
            /**
             * Metadata exceeds [`Config::MaxMetadataLen`]
             **/
            MetadataExceedsMaxLen: AugmentedError<ApiType>;
            /**
             * The amount does not meet the minimum bond to either join or create a pool.
             *
             * The depositor can never unbond to a value less than
             * `Pallet::depositor_min_bond`. The caller does not have nominating
             * permissions for the pool. Members can never unbond to a value below `MinJoinBond`.
             **/
            MinimumBondNotMet: AugmentedError<ApiType>;
            /**
             * No commission current has been set.
             **/
            NoCommissionCurrentSet: AugmentedError<ApiType>;
            /**
             * There is no pending commission to claim.
             **/
            NoPendingCommission: AugmentedError<ApiType>;
            /**
             * A pool must be in [`PoolState::Destroying`] in order for the depositor to unbond or for
             * other members to be permissionlessly unbonded.
             **/
            NotDestroying: AugmentedError<ApiType>;
            /**
             * Either a) the caller cannot make a valid kick or b) the pool is not destroying.
             **/
            NotKickerOrDestroying: AugmentedError<ApiType>;
            /**
             * The caller does not have nominating permissions for the pool.
             **/
            NotNominator: AugmentedError<ApiType>;
            /**
             * The pool is not open to join
             **/
            NotOpen: AugmentedError<ApiType>;
            /**
             * The transaction could not be executed due to overflow risk for the pool.
             **/
            OverflowRisk: AugmentedError<ApiType>;
            /**
             * Partial unbonding now allowed permissionlessly.
             **/
            PartialUnbondNotAllowedPermissionlessly: AugmentedError<ApiType>;
            /**
             * Pool id currently in use.
             **/
            PoolIdInUse: AugmentedError<ApiType>;
            /**
             * An account is not a member.
             **/
            PoolMemberNotFound: AugmentedError<ApiType>;
            /**
             * A (bonded) pool id does not exist.
             **/
            PoolNotFound: AugmentedError<ApiType>;
            /**
             * A reward pool does not exist. In all cases this is a system logic error.
             **/
            RewardPoolNotFound: AugmentedError<ApiType>;
            /**
             * A sub pool does not exist.
             **/
            SubPoolsNotFound: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        proxy: {
            /**
             * Account is already a proxy.
             **/
            Duplicate: AugmentedError<ApiType>;
            /**
             * Call may not be made by proxy because it may escalate its privileges.
             **/
            NoPermission: AugmentedError<ApiType>;
            /**
             * Cannot add self as proxy.
             **/
            NoSelfProxy: AugmentedError<ApiType>;
            /**
             * Proxy registration not found.
             **/
            NotFound: AugmentedError<ApiType>;
            /**
             * Sender is not a proxy of the account to be proxied.
             **/
            NotProxy: AugmentedError<ApiType>;
            /**
             * There are too many proxies registered or too many announcements pending.
             **/
            TooMany: AugmentedError<ApiType>;
            /**
             * Announcement, if made at all, was made too recently.
             **/
            Unannounced: AugmentedError<ApiType>;
            /**
             * A call which is incompatible with the proxy type's filter was attempted.
             **/
            Unproxyable: AugmentedError<ApiType>;
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
        session: {
            /**
             * Registered duplicate key.
             **/
            DuplicatedKey: AugmentedError<ApiType>;
            /**
             * Invalid ownership proof.
             **/
            InvalidProof: AugmentedError<ApiType>;
            /**
             * Key setting account is not live, so it's impossible to associate keys.
             **/
            NoAccount: AugmentedError<ApiType>;
            /**
             * No associated validator ID for account.
             **/
            NoAssociatedValidatorId: AugmentedError<ApiType>;
            /**
             * No keys are associated with this account.
             **/
            NoKeys: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        staking: {
            /**
             * Stash is already bonded.
             **/
            AlreadyBonded: AugmentedError<ApiType>;
            /**
             * Rewards for this era have already been claimed for this validator.
             **/
            AlreadyClaimed: AugmentedError<ApiType>;
            /**
             * Controller is already paired.
             **/
            AlreadyPaired: AugmentedError<ApiType>;
            /**
             * Internal state has become somehow corrupted and the operation cannot continue.
             **/
            BadState: AugmentedError<ApiType>;
            /**
             * A nomination target was supplied that was blocked or otherwise not a validator.
             **/
            BadTarget: AugmentedError<ApiType>;
            /**
             * Some bound is not met.
             **/
            BoundNotMet: AugmentedError<ApiType>;
            /**
             * The user has enough bond and thus cannot be chilled forcefully by an external person.
             **/
            CannotChillOther: AugmentedError<ApiType>;
            /**
             * Commission is too low. Must be at least `MinCommission`.
             **/
            CommissionTooLow: AugmentedError<ApiType>;
            /**
             * Duplicate index.
             **/
            DuplicateIndex: AugmentedError<ApiType>;
            /**
             * Targets cannot be empty.
             **/
            EmptyTargets: AugmentedError<ApiType>;
            /**
             * Attempting to target a stash that still has funds.
             **/
            FundedTarget: AugmentedError<ApiType>;
            /**
             * Incorrect previous history depth input provided.
             **/
            IncorrectHistoryDepth: AugmentedError<ApiType>;
            /**
             * Incorrect number of slashing spans provided.
             **/
            IncorrectSlashingSpans: AugmentedError<ApiType>;
            /**
             * Cannot have a validator or nominator role, with value less than the minimum defined by
             * governance (see `MinValidatorBond` and `MinNominatorBond`). If unbonding is the
             * intention, `chill` first to remove one's role as validator/nominator.
             **/
            InsufficientBond: AugmentedError<ApiType>;
            /**
             * Invalid era to reward.
             **/
            InvalidEraToReward: AugmentedError<ApiType>;
            /**
             * Invalid number of nominations.
             **/
            InvalidNumberOfNominations: AugmentedError<ApiType>;
            /**
             * Slash record index out of bounds.
             **/
            InvalidSlashIndex: AugmentedError<ApiType>;
            /**
             * Can not schedule more unlock chunks.
             **/
            NoMoreChunks: AugmentedError<ApiType>;
            /**
             * Not a controller account.
             **/
            NotController: AugmentedError<ApiType>;
            /**
             * Items are not sorted and unique.
             **/
            NotSortedAndUnique: AugmentedError<ApiType>;
            /**
             * Not a stash account.
             **/
            NotStash: AugmentedError<ApiType>;
            /**
             * Can not rebond without unlocking chunks.
             **/
            NoUnlockChunk: AugmentedError<ApiType>;
            /**
             * There are too many nominators in the system. Governance needs to adjust the staking
             * settings to keep things safe for the runtime.
             **/
            TooManyNominators: AugmentedError<ApiType>;
            /**
             * Too many nomination targets supplied.
             **/
            TooManyTargets: AugmentedError<ApiType>;
            /**
             * There are too many validator candidates in the system. Governance needs to adjust the
             * staking settings to keep things safe for the runtime.
             **/
            TooManyValidators: AugmentedError<ApiType>;
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
        taskScheduler: {
            /**
             * The node is an authority but there is no account to create a
             * callback transaction. This is likely an internal error.
             **/
            NoLocalAcctForSignedTx: AugmentedError<ApiType>;
            /**
             * Failed to send an offchain callback transaction. This is likely
             * an internal error.
             **/
            OffchainSignedTxFailed: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        utility: {
            /**
             * Too many calls batched.
             **/
            TooManyCalls: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
        voterList: {
            /**
             * A error in the list interface implementation.
             **/
            List: AugmentedError<ApiType>;
            /**
             * Generic error
             **/
            [key: string]: AugmentedError<ApiType>;
        };
    } // AugmentedErrors
} // declare module
