// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
    /**
     * Lookup3: frame_system::AccountInfo<Index, pallet_balances::AccountData<Balance>>
     **/
    FrameSystemAccountInfo: {
        nonce: 'u32',
        consumers: 'u32',
        providers: 'u32',
        sufficients: 'u32',
        data: 'PalletBalancesAccountData',
    },
    /**
     * Lookup5: pallet_balances::AccountData<Balance>
     **/
    PalletBalancesAccountData: {
        free: 'u128',
        reserved: 'u128',
        miscFrozen: 'u128',
        feeFrozen: 'u128',
    },
    /**
     * Lookup7: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
     **/
    FrameSupportDispatchPerDispatchClassWeight: {
        normal: 'SpWeightsWeightV2Weight',
        operational: 'SpWeightsWeightV2Weight',
        mandatory: 'SpWeightsWeightV2Weight',
    },
    /**
     * Lookup8: sp_weights::weight_v2::Weight
     **/
    SpWeightsWeightV2Weight: {
        refTime: 'Compact<u64>',
        proofSize: 'Compact<u64>',
    },
    /**
     * Lookup13: sp_runtime::generic::digest::Digest
     **/
    SpRuntimeDigest: {
        logs: 'Vec<SpRuntimeDigestDigestItem>',
    },
    /**
     * Lookup15: sp_runtime::generic::digest::DigestItem
     **/
    SpRuntimeDigestDigestItem: {
        _enum: {
            Other: 'Bytes',
            __Unused1: 'Null',
            __Unused2: 'Null',
            __Unused3: 'Null',
            Consensus: '([u8;4],Bytes)',
            Seal: '([u8;4],Bytes)',
            PreRuntime: '([u8;4],Bytes)',
            __Unused7: 'Null',
            RuntimeEnvironmentUpdated: 'Null',
        },
    },
    /**
     * Lookup18: frame_system::EventRecord<creditcoin_node_runtime::RuntimeEvent, primitive_types::H256>
     **/
    FrameSystemEventRecord: {
        phase: 'FrameSystemPhase',
        event: 'Event',
        topics: 'Vec<H256>',
    },
    /**
     * Lookup20: frame_system::pallet::Event<T>
     **/
    FrameSystemEvent: {
        _enum: {
            ExtrinsicSuccess: {
                dispatchInfo: 'FrameSupportDispatchDispatchInfo',
            },
            ExtrinsicFailed: {
                dispatchError: 'SpRuntimeDispatchError',
                dispatchInfo: 'FrameSupportDispatchDispatchInfo',
            },
            CodeUpdated: 'Null',
            NewAccount: {
                account: 'AccountId32',
            },
            KilledAccount: {
                account: 'AccountId32',
            },
            Remarked: {
                _alias: {
                    hash_: 'hash',
                },
                sender: 'AccountId32',
                hash_: 'H256',
            },
        },
    },
    /**
     * Lookup21: frame_support::dispatch::DispatchInfo
     **/
    FrameSupportDispatchDispatchInfo: {
        weight: 'SpWeightsWeightV2Weight',
        class: 'FrameSupportDispatchDispatchClass',
        paysFee: 'FrameSupportDispatchPays',
    },
    /**
     * Lookup22: frame_support::dispatch::DispatchClass
     **/
    FrameSupportDispatchDispatchClass: {
        _enum: ['Normal', 'Operational', 'Mandatory'],
    },
    /**
     * Lookup23: frame_support::dispatch::Pays
     **/
    FrameSupportDispatchPays: {
        _enum: ['Yes', 'No'],
    },
    /**
     * Lookup24: sp_runtime::DispatchError
     **/
    SpRuntimeDispatchError: {
        _enum: {
            Other: 'Null',
            CannotLookup: 'Null',
            BadOrigin: 'Null',
            Module: 'SpRuntimeModuleError',
            ConsumerRemaining: 'Null',
            NoProviders: 'Null',
            TooManyConsumers: 'Null',
            Token: 'SpRuntimeTokenError',
            Arithmetic: 'SpArithmeticArithmeticError',
            Transactional: 'SpRuntimeTransactionalError',
            Exhausted: 'Null',
            Corruption: 'Null',
            Unavailable: 'Null',
        },
    },
    /**
     * Lookup25: sp_runtime::ModuleError
     **/
    SpRuntimeModuleError: {
        index: 'u8',
        error: '[u8;4]',
    },
    /**
     * Lookup26: sp_runtime::TokenError
     **/
    SpRuntimeTokenError: {
        _enum: ['NoFunds', 'WouldDie', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported'],
    },
    /**
     * Lookup27: sp_arithmetic::ArithmeticError
     **/
    SpArithmeticArithmeticError: {
        _enum: ['Underflow', 'Overflow', 'DivisionByZero'],
    },
    /**
     * Lookup28: sp_runtime::TransactionalError
     **/
    SpRuntimeTransactionalError: {
        _enum: ['LimitReached', 'NoLayer'],
    },
    /**
     * Lookup29: pallet_balances::pallet::Event<T, I>
     **/
    PalletBalancesEvent: {
        _enum: {
            Endowed: {
                account: 'AccountId32',
                freeBalance: 'u128',
            },
            DustLost: {
                account: 'AccountId32',
                amount: 'u128',
            },
            Transfer: {
                from: 'AccountId32',
                to: 'AccountId32',
                amount: 'u128',
            },
            BalanceSet: {
                who: 'AccountId32',
                free: 'u128',
                reserved: 'u128',
            },
            Reserved: {
                who: 'AccountId32',
                amount: 'u128',
            },
            Unreserved: {
                who: 'AccountId32',
                amount: 'u128',
            },
            ReserveRepatriated: {
                from: 'AccountId32',
                to: 'AccountId32',
                amount: 'u128',
                destinationStatus: 'FrameSupportTokensMiscBalanceStatus',
            },
            Deposit: {
                who: 'AccountId32',
                amount: 'u128',
            },
            Withdraw: {
                who: 'AccountId32',
                amount: 'u128',
            },
            Slashed: {
                who: 'AccountId32',
                amount: 'u128',
            },
        },
    },
    /**
     * Lookup30: frame_support::traits::tokens::misc::BalanceStatus
     **/
    FrameSupportTokensMiscBalanceStatus: {
        _enum: ['Free', 'Reserved'],
    },
    /**
     * Lookup31: pallet_staking::pallet::pallet::Event<T>
     **/
    PalletStakingPalletEvent: {
        _enum: {
            EraPaid: {
                eraIndex: 'u32',
                validatorPayout: 'u128',
                remainder: 'u128',
            },
            Rewarded: {
                stash: 'AccountId32',
                amount: 'u128',
            },
            Slashed: {
                staker: 'AccountId32',
                amount: 'u128',
            },
            SlashReported: {
                validator: 'AccountId32',
                fraction: 'Perbill',
                slashEra: 'u32',
            },
            OldSlashingReportDiscarded: {
                sessionIndex: 'u32',
            },
            StakersElected: 'Null',
            Bonded: {
                stash: 'AccountId32',
                amount: 'u128',
            },
            Unbonded: {
                stash: 'AccountId32',
                amount: 'u128',
            },
            Withdrawn: {
                stash: 'AccountId32',
                amount: 'u128',
            },
            Kicked: {
                nominator: 'AccountId32',
                stash: 'AccountId32',
            },
            StakingElectionFailed: 'Null',
            Chilled: {
                stash: 'AccountId32',
            },
            PayoutStarted: {
                eraIndex: 'u32',
                validatorStash: 'AccountId32',
            },
            ValidatorPrefsSet: {
                stash: 'AccountId32',
                prefs: 'PalletStakingValidatorPrefs',
            },
            ForceEra: {
                mode: 'PalletStakingForcing',
            },
        },
    },
    /**
     * Lookup33: pallet_staking::ValidatorPrefs
     **/
    PalletStakingValidatorPrefs: {
        commission: 'Compact<Perbill>',
        blocked: 'bool',
    },
    /**
     * Lookup36: pallet_staking::Forcing
     **/
    PalletStakingForcing: {
        _enum: ['NotForcing', 'ForceNew', 'ForceNone', 'ForceAlways'],
    },
    /**
     * Lookup37: pallet_offences::pallet::Event
     **/
    PalletOffencesEvent: {
        _enum: {
            Offence: {
                kind: '[u8;16]',
                timeslot: 'Bytes',
            },
        },
    },
    /**
     * Lookup39: pallet_session::pallet::Event
     **/
    PalletSessionEvent: {
        _enum: {
            NewSession: {
                sessionIndex: 'u32',
            },
        },
    },
    /**
     * Lookup40: pallet_grandpa::pallet::Event
     **/
    PalletGrandpaEvent: {
        _enum: {
            NewAuthorities: {
                authoritySet: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
            },
            Paused: 'Null',
            Resumed: 'Null',
        },
    },
    /**
     * Lookup43: sp_consensus_grandpa::app::Public
     **/
    SpConsensusGrandpaAppPublic: 'SpCoreEd25519Public',
    /**
     * Lookup44: sp_core::ed25519::Public
     **/
    SpCoreEd25519Public: '[u8;32]',
    /**
     * Lookup45: pallet_im_online::pallet::Event<T>
     **/
    PalletImOnlineEvent: {
        _enum: {
            HeartbeatReceived: {
                authorityId: 'PalletImOnlineSr25519AppSr25519Public',
            },
            AllGood: 'Null',
            SomeOffline: {
                offline: 'Vec<(AccountId32,PalletStakingExposure)>',
            },
        },
    },
    /**
     * Lookup46: pallet_im_online::sr25519::app_sr25519::Public
     **/
    PalletImOnlineSr25519AppSr25519Public: 'SpCoreSr25519Public',
    /**
     * Lookup47: sp_core::sr25519::Public
     **/
    SpCoreSr25519Public: '[u8;32]',
    /**
     * Lookup50: pallet_staking::Exposure<sp_core::crypto::AccountId32, Balance>
     **/
    PalletStakingExposure: {
        total: 'Compact<u128>',
        own: 'Compact<u128>',
        others: 'Vec<PalletStakingIndividualExposure>',
    },
    /**
     * Lookup53: pallet_staking::IndividualExposure<sp_core::crypto::AccountId32, Balance>
     **/
    PalletStakingIndividualExposure: {
        who: 'AccountId32',
        value: 'Compact<u128>',
    },
    /**
     * Lookup54: pallet_bags_list::pallet::Event<T, I>
     **/
    PalletBagsListEvent: {
        _enum: {
            Rebagged: {
                who: 'AccountId32',
                from: 'u64',
                to: 'u64',
            },
            ScoreUpdated: {
                who: 'AccountId32',
                newScore: 'u64',
            },
        },
    },
    /**
     * Lookup55: pallet_transaction_payment::pallet::Event<T>
     **/
    PalletTransactionPaymentEvent: {
        _enum: {
            TransactionFeePaid: {
                who: 'AccountId32',
                actualFee: 'u128',
                tip: 'u128',
            },
        },
    },
    /**
     * Lookup56: pallet_sudo::pallet::Event<T>
     **/
    PalletSudoEvent: {
        _enum: {
            Sudid: {
                sudoResult: 'Result<Null, SpRuntimeDispatchError>',
            },
            KeyChanged: {
                oldSudoer: 'Option<AccountId32>',
            },
            SudoAsDone: {
                sudoResult: 'Result<Null, SpRuntimeDispatchError>',
            },
        },
    },
    /**
     * Lookup60: pallet_creditcoin::pallet::Event<T>
     **/
    PalletCreditcoinEvent: {
        _enum: {
            AddressRegistered: '(H256,PalletCreditcoinAddress)',
            TransferRegistered: '(H256,PalletCreditcoinTransfer)',
            TransferVerified: 'H256',
            TransferProcessed: 'H256',
            AskOrderAdded: '(PalletCreditcoinAskOrderId,PalletCreditcoinAskOrder)',
            BidOrderAdded: '(PalletCreditcoinBidOrderId,PalletCreditcoinBidOrder)',
            OfferAdded: '(PalletCreditcoinOfferId,PalletCreditcoinOffer)',
            DealOrderAdded: '(PalletCreditcoinDealOrderId,PalletCreditcoinDealOrder)',
            DealOrderFunded: 'PalletCreditcoinDealOrderId',
            DealOrderLocked: 'PalletCreditcoinDealOrderId',
            DealOrderClosed: 'PalletCreditcoinDealOrderId',
            LoanExempted: 'PalletCreditcoinDealOrderId',
            LegacyWalletClaimed: '(AccountId32,PalletCreditcoinLegacySighash,u128)',
            TransferFailedVerification: '(H256,PalletCreditcoinOcwErrorsVerificationFailureCause)',
            Burned: 'u64',
        },
    },
    /**
     * Lookup62: pallet_creditcoin::types::Address<sp_core::crypto::AccountId32>
     **/
    PalletCreditcoinAddress: {
        blockchain: 'PalletCreditcoinBlockchain',
        value: 'Bytes',
        owner: 'AccountId32',
    },
    /**
     * Lookup63: pallet_creditcoin::types::Blockchain
     **/
    PalletCreditcoinBlockchain: {
        _enum: {
            Ethereum: 'Null',
            Rinkeby: 'Null',
            Luniverse: 'Null',
            Bitcoin: 'Null',
            Other: 'Bytes',
        },
    },
    /**
     * Lookup66: pallet_creditcoin::types::transfer::Transfer<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
     **/
    PalletCreditcoinTransfer: {
        blockchain: 'PalletCreditcoinBlockchain',
        kind: 'PalletCreditcoinTransferKind',
        from: 'H256',
        to: 'H256',
        orderId: 'PalletCreditcoinOrderId',
        amount: 'U256',
        txId: 'Bytes',
        block: 'u32',
        isProcessed: 'bool',
        accountId: 'AccountId32',
        timestamp: 'Option<u64>',
    },
    /**
     * Lookup67: pallet_creditcoin::types::TransferKind
     **/
    PalletCreditcoinTransferKind: {
        _enum: {
            Erc20: 'Bytes',
            Ethless: 'Bytes',
            Native: 'Null',
            Other: 'Bytes',
        },
    },
    /**
     * Lookup68: pallet_creditcoin::types::OrderId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinOrderId: {
        _enum: {
            Deal: 'PalletCreditcoinDealOrderId',
            Repayment: 'PalletCreditcoinRepaymentOrderId',
        },
    },
    /**
     * Lookup69: pallet_creditcoin::types::DealOrderId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinDealOrderId: '(u32,H256)',
    /**
     * Lookup70: pallet_creditcoin::types::RepaymentOrderId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinRepaymentOrderId: '(u32,H256)',
    /**
     * Lookup74: pallet_creditcoin::types::AskOrderId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinAskOrderId: '(u32,H256)',
    /**
     * Lookup75: pallet_creditcoin::types::AskOrder<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinAskOrder: {
        blockchain: 'PalletCreditcoinBlockchain',
        lenderAddressId: 'H256',
        terms: 'PalletCreditcoinLoanTermsAskTerms',
        expirationBlock: 'u32',
        block: 'u32',
        lender: 'AccountId32',
    },
    /**
     * Lookup76: pallet_creditcoin::types::loan_terms::AskTerms
     **/
    PalletCreditcoinLoanTermsAskTerms: 'PalletCreditcoinLoanTerms',
    /**
     * Lookup77: pallet_creditcoin::types::loan_terms::LoanTerms
     **/
    PalletCreditcoinLoanTerms: {
        amount: 'U256',
        interestRate: 'PalletCreditcoinLoanTermsInterestRate',
        termLength: 'PalletCreditcoinLoanTermsDuration',
    },
    /**
     * Lookup78: pallet_creditcoin::types::loan_terms::InterestRate
     **/
    PalletCreditcoinLoanTermsInterestRate: {
        ratePerPeriod: 'u64',
        decimals: 'u64',
        period: 'PalletCreditcoinLoanTermsDuration',
        interestType: 'PalletCreditcoinLoanTermsInterestType',
    },
    /**
     * Lookup79: pallet_creditcoin::types::loan_terms::Duration
     **/
    PalletCreditcoinLoanTermsDuration: {
        secs: 'u64',
        nanos: 'u32',
    },
    /**
     * Lookup80: pallet_creditcoin::types::loan_terms::InterestType
     **/
    PalletCreditcoinLoanTermsInterestType: {
        _enum: ['Simple', 'Compound'],
    },
    /**
     * Lookup81: pallet_creditcoin::types::BidOrderId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinBidOrderId: '(u32,H256)',
    /**
     * Lookup82: pallet_creditcoin::types::BidOrder<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinBidOrder: {
        blockchain: 'PalletCreditcoinBlockchain',
        borrowerAddressId: 'H256',
        terms: 'PalletCreditcoinLoanTermsBidTerms',
        expirationBlock: 'u32',
        block: 'u32',
        borrower: 'AccountId32',
    },
    /**
     * Lookup83: pallet_creditcoin::types::loan_terms::BidTerms
     **/
    PalletCreditcoinLoanTermsBidTerms: 'PalletCreditcoinLoanTerms',
    /**
     * Lookup84: pallet_creditcoin::types::OfferId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinOfferId: '(u32,H256)',
    /**
     * Lookup85: pallet_creditcoin::types::Offer<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinOffer: {
        blockchain: 'PalletCreditcoinBlockchain',
        askId: 'PalletCreditcoinAskOrderId',
        bidId: 'PalletCreditcoinBidOrderId',
        expirationBlock: 'u32',
        block: 'u32',
        lender: 'AccountId32',
    },
    /**
     * Lookup86: pallet_creditcoin::types::DealOrder<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
     **/
    PalletCreditcoinDealOrder: {
        blockchain: 'PalletCreditcoinBlockchain',
        offerId: 'PalletCreditcoinOfferId',
        lenderAddressId: 'H256',
        borrowerAddressId: 'H256',
        terms: 'PalletCreditcoinLoanTerms',
        expirationBlock: 'u32',
        timestamp: 'u64',
        block: 'Option<u32>',
        fundingTransferId: 'Option<H256>',
        repaymentTransferId: 'Option<H256>',
        lock: 'Option<AccountId32>',
        borrower: 'AccountId32',
    },
    /**
     * Lookup89: pallet_creditcoin::types::LegacySighash
     **/
    PalletCreditcoinLegacySighash: '[u8;60]',
    /**
     * Lookup91: pallet_creditcoin::ocw::errors::VerificationFailureCause
     **/
    PalletCreditcoinOcwErrorsVerificationFailureCause: {
        _enum: [
            'TaskNonexistent',
            'TaskFailed',
            'TaskPending',
            'TaskUnconfirmed',
            'TaskInFuture',
            'IncorrectContract',
            'MissingReceiver',
            'MissingSender',
            'AbiMismatch',
            'IncorrectInputLength',
            'EmptyInput',
            'IncorrectInputType',
            'IncorrectAmount',
            'IncorrectNonce',
            'IncorrectReceiver',
            'IncorrectSender',
            'InvalidAddress',
            'UnsupportedMethod',
            'TransactionNotFound',
            'InsufficientFaucetBalance',
        ],
    },
    /**
     * Lookup93: pallet_rewards::pallet::Event<T>
     **/
    PalletRewardsEvent: {
        _enum: {
            RewardIssued: '(AccountId32,u128)',
        },
    },
    /**
     * Lookup94: pallet_scheduler::pallet::Event<T>
     **/
    PalletSchedulerEvent: {
        _enum: {
            Scheduled: {
                when: 'u32',
                index: 'u32',
            },
            Canceled: {
                when: 'u32',
                index: 'u32',
            },
            Dispatched: {
                task: '(u32,u32)',
                id: 'Option<[u8;32]>',
                result: 'Result<Null, SpRuntimeDispatchError>',
            },
            CallUnavailable: {
                task: '(u32,u32)',
                id: 'Option<[u8;32]>',
            },
            PeriodicFailed: {
                task: '(u32,u32)',
                id: 'Option<[u8;32]>',
            },
            PermanentlyOverweight: {
                task: '(u32,u32)',
                id: 'Option<[u8;32]>',
            },
        },
    },
    /**
     * Lookup97: pallet_offchain_task_scheduler::pallet::Event<T>
     **/
    PalletOffchainTaskSchedulerEvent: 'Null',
    /**
     * Lookup98: pallet_identity::pallet::Event<T>
     **/
    PalletIdentityEvent: {
        _enum: {
            IdentitySet: {
                who: 'AccountId32',
            },
            IdentityCleared: {
                who: 'AccountId32',
                deposit: 'u128',
            },
            IdentityKilled: {
                who: 'AccountId32',
                deposit: 'u128',
            },
            JudgementRequested: {
                who: 'AccountId32',
                registrarIndex: 'u32',
            },
            JudgementUnrequested: {
                who: 'AccountId32',
                registrarIndex: 'u32',
            },
            JudgementGiven: {
                target: 'AccountId32',
                registrarIndex: 'u32',
            },
            RegistrarAdded: {
                registrarIndex: 'u32',
            },
            SubIdentityAdded: {
                sub: 'AccountId32',
                main: 'AccountId32',
                deposit: 'u128',
            },
            SubIdentityRemoved: {
                sub: 'AccountId32',
                main: 'AccountId32',
                deposit: 'u128',
            },
            SubIdentityRevoked: {
                sub: 'AccountId32',
                main: 'AccountId32',
                deposit: 'u128',
            },
        },
    },
    /**
     * Lookup99: pallet_utility::pallet::Event
     **/
    PalletUtilityEvent: {
        _enum: {
            BatchInterrupted: {
                index: 'u32',
                error: 'SpRuntimeDispatchError',
            },
            BatchCompleted: 'Null',
            BatchCompletedWithErrors: 'Null',
            ItemCompleted: 'Null',
            ItemFailed: {
                error: 'SpRuntimeDispatchError',
            },
            DispatchedAs: {
                result: 'Result<Null, SpRuntimeDispatchError>',
            },
        },
    },
    /**
     * Lookup100: pallet_proxy::pallet::Event<T>
     **/
    PalletProxyEvent: {
        _enum: {
            ProxyExecuted: {
                result: 'Result<Null, SpRuntimeDispatchError>',
            },
            PureCreated: {
                pure: 'AccountId32',
                who: 'AccountId32',
                proxyType: 'Null',
                disambiguationIndex: 'u16',
            },
            Announced: {
                real: 'AccountId32',
                proxy: 'AccountId32',
                callHash: 'H256',
            },
            ProxyAdded: {
                delegator: 'AccountId32',
                delegatee: 'AccountId32',
                proxyType: 'Null',
                delay: 'u32',
            },
            ProxyRemoved: {
                delegator: 'AccountId32',
                delegatee: 'AccountId32',
                proxyType: 'Null',
                delay: 'u32',
            },
        },
    },
    /**
     * Lookup102: pallet_fast_unstake::pallet::Event<T>
     **/
    PalletFastUnstakeEvent: {
        _enum: {
            Unstaked: {
                stash: 'AccountId32',
                result: 'Result<Null, SpRuntimeDispatchError>',
            },
            Slashed: {
                stash: 'AccountId32',
                amount: 'u128',
            },
            InternalError: 'Null',
            BatchChecked: {
                eras: 'Vec<u32>',
            },
            BatchFinished: {
                _alias: {
                    size_: 'size',
                },
                size_: 'u32',
            },
        },
    },
    /**
     * Lookup104: pallet_nomination_pools::pallet::Event<T>
     **/
    PalletNominationPoolsEvent: {
        _enum: {
            Created: {
                depositor: 'AccountId32',
                poolId: 'u32',
            },
            Bonded: {
                member: 'AccountId32',
                poolId: 'u32',
                bonded: 'u128',
                joined: 'bool',
            },
            PaidOut: {
                member: 'AccountId32',
                poolId: 'u32',
                payout: 'u128',
            },
            Unbonded: {
                member: 'AccountId32',
                poolId: 'u32',
                balance: 'u128',
                points: 'u128',
                era: 'u32',
            },
            Withdrawn: {
                member: 'AccountId32',
                poolId: 'u32',
                balance: 'u128',
                points: 'u128',
            },
            Destroyed: {
                poolId: 'u32',
            },
            StateChanged: {
                poolId: 'u32',
                newState: 'PalletNominationPoolsPoolState',
            },
            MemberRemoved: {
                poolId: 'u32',
                member: 'AccountId32',
            },
            RolesUpdated: {
                root: 'Option<AccountId32>',
                bouncer: 'Option<AccountId32>',
                nominator: 'Option<AccountId32>',
            },
            PoolSlashed: {
                poolId: 'u32',
                balance: 'u128',
            },
            UnbondingPoolSlashed: {
                poolId: 'u32',
                era: 'u32',
                balance: 'u128',
            },
            PoolCommissionUpdated: {
                poolId: 'u32',
                current: 'Option<(Perbill,AccountId32)>',
            },
            PoolMaxCommissionUpdated: {
                poolId: 'u32',
                maxCommission: 'Perbill',
            },
            PoolCommissionChangeRateUpdated: {
                poolId: 'u32',
                changeRate: 'PalletNominationPoolsCommissionChangeRate',
            },
            PoolCommissionClaimed: {
                poolId: 'u32',
                commission: 'u128',
            },
        },
    },
    /**
     * Lookup105: pallet_nomination_pools::PoolState
     **/
    PalletNominationPoolsPoolState: {
        _enum: ['Open', 'Blocked', 'Destroying'],
    },
    /**
     * Lookup108: pallet_nomination_pools::CommissionChangeRate<BlockNumber>
     **/
    PalletNominationPoolsCommissionChangeRate: {
        maxIncrease: 'Perbill',
        minDelay: 'u32',
    },
    /**
     * Lookup109: frame_system::Phase
     **/
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: 'u32',
            Finalization: 'Null',
            Initialization: 'Null',
        },
    },
    /**
     * Lookup112: frame_system::LastRuntimeUpgradeInfo
     **/
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: 'Compact<u32>',
        specName: 'Text',
    },
    /**
     * Lookup115: frame_system::pallet::Call<T>
     **/
    FrameSystemCall: {
        _enum: {
            remark: {
                remark: 'Bytes',
            },
            set_heap_pages: {
                pages: 'u64',
            },
            set_code: {
                code: 'Bytes',
            },
            set_code_without_checks: {
                code: 'Bytes',
            },
            set_storage: {
                items: 'Vec<(Bytes,Bytes)>',
            },
            kill_storage: {
                _alias: {
                    keys_: 'keys',
                },
                keys_: 'Vec<Bytes>',
            },
            kill_prefix: {
                prefix: 'Bytes',
                subkeys: 'u32',
            },
            remark_with_event: {
                remark: 'Bytes',
            },
        },
    },
    /**
     * Lookup119: frame_system::limits::BlockWeights
     **/
    FrameSystemLimitsBlockWeights: {
        baseBlock: 'SpWeightsWeightV2Weight',
        maxBlock: 'SpWeightsWeightV2Weight',
        perClass: 'FrameSupportDispatchPerDispatchClassWeightsPerClass',
    },
    /**
     * Lookup120: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
     **/
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: 'FrameSystemLimitsWeightsPerClass',
        operational: 'FrameSystemLimitsWeightsPerClass',
        mandatory: 'FrameSystemLimitsWeightsPerClass',
    },
    /**
     * Lookup121: frame_system::limits::WeightsPerClass
     **/
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: 'SpWeightsWeightV2Weight',
        maxExtrinsic: 'Option<SpWeightsWeightV2Weight>',
        maxTotal: 'Option<SpWeightsWeightV2Weight>',
        reserved: 'Option<SpWeightsWeightV2Weight>',
    },
    /**
     * Lookup123: frame_system::limits::BlockLength
     **/
    FrameSystemLimitsBlockLength: {
        max: 'FrameSupportDispatchPerDispatchClassU32',
    },
    /**
     * Lookup124: frame_support::dispatch::PerDispatchClass<T>
     **/
    FrameSupportDispatchPerDispatchClassU32: {
        normal: 'u32',
        operational: 'u32',
        mandatory: 'u32',
    },
    /**
     * Lookup125: sp_weights::RuntimeDbWeight
     **/
    SpWeightsRuntimeDbWeight: {
        read: 'u64',
        write: 'u64',
    },
    /**
     * Lookup126: sp_version::RuntimeVersion
     **/
    SpVersionRuntimeVersion: {
        specName: 'Text',
        implName: 'Text',
        authoringVersion: 'u32',
        specVersion: 'u32',
        implVersion: 'u32',
        apis: 'Vec<([u8;8],u32)>',
        transactionVersion: 'u32',
        stateVersion: 'u8',
    },
    /**
     * Lookup131: frame_system::pallet::Error<T>
     **/
    FrameSystemError: {
        _enum: [
            'InvalidSpecName',
            'SpecVersionNeedsToIncrease',
            'FailedToExtractRuntimeVersion',
            'NonDefaultComposite',
            'NonZeroRefCount',
            'CallFiltered',
        ],
    },
    /**
     * Lookup132: pallet_timestamp::pallet::Call<T>
     **/
    PalletTimestampCall: {
        _enum: {
            set: {
                now: 'Compact<u64>',
            },
        },
    },
    /**
     * Lookup135: sp_consensus_babe::app::Public
     **/
    SpConsensusBabeAppPublic: 'SpCoreSr25519Public',
    /**
     * Lookup138: sp_consensus_babe::digests::NextConfigDescriptor
     **/
    SpConsensusBabeDigestsNextConfigDescriptor: {
        _enum: {
            __Unused0: 'Null',
            V1: {
                c: '(u64,u64)',
                allowedSlots: 'SpConsensusBabeAllowedSlots',
            },
        },
    },
    /**
     * Lookup140: sp_consensus_babe::AllowedSlots
     **/
    SpConsensusBabeAllowedSlots: {
        _enum: ['PrimarySlots', 'PrimaryAndSecondaryPlainSlots', 'PrimaryAndSecondaryVRFSlots'],
    },
    /**
     * Lookup144: sp_consensus_babe::digests::PreDigest
     **/
    SpConsensusBabeDigestsPreDigest: {
        _enum: {
            __Unused0: 'Null',
            Primary: 'SpConsensusBabeDigestsPrimaryPreDigest',
            SecondaryPlain: 'SpConsensusBabeDigestsSecondaryPlainPreDigest',
            SecondaryVRF: 'SpConsensusBabeDigestsSecondaryVRFPreDigest',
        },
    },
    /**
     * Lookup145: sp_consensus_babe::digests::PrimaryPreDigest
     **/
    SpConsensusBabeDigestsPrimaryPreDigest: {
        authorityIndex: 'u32',
        slot: 'u64',
        vrfOutput: '[u8;32]',
        vrfProof: '[u8;64]',
    },
    /**
     * Lookup147: sp_consensus_babe::digests::SecondaryPlainPreDigest
     **/
    SpConsensusBabeDigestsSecondaryPlainPreDigest: {
        authorityIndex: 'u32',
        slot: 'u64',
    },
    /**
     * Lookup148: sp_consensus_babe::digests::SecondaryVRFPreDigest
     **/
    SpConsensusBabeDigestsSecondaryVRFPreDigest: {
        authorityIndex: 'u32',
        slot: 'u64',
        vrfOutput: '[u8;32]',
        vrfProof: '[u8;64]',
    },
    /**
     * Lookup149: sp_consensus_babe::BabeEpochConfiguration
     **/
    SpConsensusBabeBabeEpochConfiguration: {
        c: '(u64,u64)',
        allowedSlots: 'SpConsensusBabeAllowedSlots',
    },
    /**
     * Lookup153: pallet_babe::pallet::Call<T>
     **/
    PalletBabeCall: {
        _enum: {
            report_equivocation: {
                equivocationProof: 'SpConsensusSlotsEquivocationProof',
                keyOwnerProof: 'SpSessionMembershipProof',
            },
            report_equivocation_unsigned: {
                equivocationProof: 'SpConsensusSlotsEquivocationProof',
                keyOwnerProof: 'SpSessionMembershipProof',
            },
            plan_config_change: {
                config: 'SpConsensusBabeDigestsNextConfigDescriptor',
            },
        },
    },
    /**
     * Lookup154: sp_consensus_slots::EquivocationProof<sp_runtime::generic::header::Header<Number, sp_runtime::traits::BlakeTwo256>, sp_consensus_babe::app::Public>
     **/
    SpConsensusSlotsEquivocationProof: {
        offender: 'SpConsensusBabeAppPublic',
        slot: 'u64',
        firstHeader: 'SpRuntimeHeader',
        secondHeader: 'SpRuntimeHeader',
    },
    /**
     * Lookup155: sp_runtime::generic::header::Header<Number, sp_runtime::traits::BlakeTwo256>
     **/
    SpRuntimeHeader: {
        parentHash: 'H256',
        number: 'Compact<u32>',
        stateRoot: 'H256',
        extrinsicsRoot: 'H256',
        digest: 'SpRuntimeDigest',
    },
    /**
     * Lookup156: sp_runtime::traits::BlakeTwo256
     **/
    SpRuntimeBlakeTwo256: 'Null',
    /**
     * Lookup157: sp_session::MembershipProof
     **/
    SpSessionMembershipProof: {
        session: 'u32',
        trieNodes: 'Vec<Bytes>',
        validatorCount: 'u32',
    },
    /**
     * Lookup158: pallet_babe::pallet::Error<T>
     **/
    PalletBabeError: {
        _enum: [
            'InvalidEquivocationProof',
            'InvalidKeyOwnershipProof',
            'DuplicateOffenceReport',
            'InvalidConfiguration',
        ],
    },
    /**
     * Lookup160: pallet_balances::BalanceLock<Balance>
     **/
    PalletBalancesBalanceLock: {
        id: '[u8;8]',
        amount: 'u128',
        reasons: 'PalletBalancesReasons',
    },
    /**
     * Lookup161: pallet_balances::Reasons
     **/
    PalletBalancesReasons: {
        _enum: ['Fee', 'Misc', 'All'],
    },
    /**
     * Lookup164: pallet_balances::ReserveData<ReserveIdentifier, Balance>
     **/
    PalletBalancesReserveData: {
        id: '[u8;8]',
        amount: 'u128',
    },
    /**
     * Lookup166: pallet_balances::pallet::Call<T, I>
     **/
    PalletBalancesCall: {
        _enum: {
            transfer: {
                dest: 'MultiAddress',
                value: 'Compact<u128>',
            },
            set_balance: {
                who: 'MultiAddress',
                newFree: 'Compact<u128>',
                newReserved: 'Compact<u128>',
            },
            force_transfer: {
                source: 'MultiAddress',
                dest: 'MultiAddress',
                value: 'Compact<u128>',
            },
            transfer_keep_alive: {
                dest: 'MultiAddress',
                value: 'Compact<u128>',
            },
            transfer_all: {
                dest: 'MultiAddress',
                keepAlive: 'bool',
            },
            force_unreserve: {
                who: 'MultiAddress',
                amount: 'u128',
            },
        },
    },
    /**
     * Lookup170: pallet_balances::pallet::Error<T, I>
     **/
    PalletBalancesError: {
        _enum: [
            'VestingBalance',
            'LiquidityRestrictions',
            'InsufficientBalance',
            'ExistentialDeposit',
            'KeepAlive',
            'ExistingVestingSchedule',
            'DeadAccount',
            'TooManyReserves',
        ],
    },
    /**
     * Lookup172: pallet_staking::StakingLedger<T>
     **/
    PalletStakingStakingLedger: {
        stash: 'AccountId32',
        total: 'Compact<u128>',
        active: 'Compact<u128>',
        unlocking: 'Vec<PalletStakingUnlockChunk>',
        claimedRewards: 'Vec<u32>',
    },
    /**
     * Lookup174: pallet_staking::UnlockChunk<Balance>
     **/
    PalletStakingUnlockChunk: {
        value: 'Compact<u128>',
        era: 'Compact<u32>',
    },
    /**
     * Lookup177: pallet_staking::RewardDestination<sp_core::crypto::AccountId32>
     **/
    PalletStakingRewardDestination: {
        _enum: {
            Staked: 'Null',
            Stash: 'Null',
            Controller: 'Null',
            Account: 'AccountId32',
            None: 'Null',
        },
    },
    /**
     * Lookup178: pallet_staking::Nominations<T>
     **/
    PalletStakingNominations: {
        targets: 'Vec<AccountId32>',
        submittedIn: 'u32',
        suppressed: 'bool',
    },
    /**
     * Lookup180: pallet_staking::ActiveEraInfo
     **/
    PalletStakingActiveEraInfo: {
        index: 'u32',
        start: 'Option<u64>',
    },
    /**
     * Lookup182: pallet_staking::EraRewardPoints<sp_core::crypto::AccountId32>
     **/
    PalletStakingEraRewardPoints: {
        total: 'u32',
        individual: 'BTreeMap<AccountId32, u32>',
    },
    /**
     * Lookup187: pallet_staking::UnappliedSlash<sp_core::crypto::AccountId32, Balance>
     **/
    PalletStakingUnappliedSlash: {
        validator: 'AccountId32',
        own: 'u128',
        others: 'Vec<(AccountId32,u128)>',
        reporters: 'Vec<AccountId32>',
        payout: 'u128',
    },
    /**
     * Lookup191: pallet_staking::slashing::SlashingSpans
     **/
    PalletStakingSlashingSlashingSpans: {
        spanIndex: 'u32',
        lastStart: 'u32',
        lastNonzeroSlash: 'u32',
        prior: 'Vec<u32>',
    },
    /**
     * Lookup192: pallet_staking::slashing::SpanRecord<Balance>
     **/
    PalletStakingSlashingSpanRecord: {
        slashed: 'u128',
        paidOut: 'u128',
    },
    /**
     * Lookup196: pallet_staking::pallet::pallet::Call<T>
     **/
    PalletStakingPalletCall: {
        _enum: {
            bond: {
                controller: 'MultiAddress',
                value: 'Compact<u128>',
                payee: 'PalletStakingRewardDestination',
            },
            bond_extra: {
                maxAdditional: 'Compact<u128>',
            },
            unbond: {
                value: 'Compact<u128>',
            },
            withdraw_unbonded: {
                numSlashingSpans: 'u32',
            },
            validate: {
                prefs: 'PalletStakingValidatorPrefs',
            },
            nominate: {
                targets: 'Vec<MultiAddress>',
            },
            chill: 'Null',
            set_payee: {
                payee: 'PalletStakingRewardDestination',
            },
            set_controller: {
                controller: 'MultiAddress',
            },
            set_validator_count: {
                _alias: {
                    new_: 'new',
                },
                new_: 'Compact<u32>',
            },
            increase_validator_count: {
                additional: 'Compact<u32>',
            },
            scale_validator_count: {
                factor: 'Percent',
            },
            force_no_eras: 'Null',
            force_new_era: 'Null',
            set_invulnerables: {
                invulnerables: 'Vec<AccountId32>',
            },
            force_unstake: {
                stash: 'AccountId32',
                numSlashingSpans: 'u32',
            },
            force_new_era_always: 'Null',
            cancel_deferred_slash: {
                era: 'u32',
                slashIndices: 'Vec<u32>',
            },
            payout_stakers: {
                validatorStash: 'AccountId32',
                era: 'u32',
            },
            rebond: {
                value: 'Compact<u128>',
            },
            reap_stash: {
                stash: 'AccountId32',
                numSlashingSpans: 'u32',
            },
            kick: {
                who: 'Vec<MultiAddress>',
            },
            set_staking_configs: {
                minNominatorBond: 'PalletStakingPalletConfigOpU128',
                minValidatorBond: 'PalletStakingPalletConfigOpU128',
                maxNominatorCount: 'PalletStakingPalletConfigOpU32',
                maxValidatorCount: 'PalletStakingPalletConfigOpU32',
                chillThreshold: 'PalletStakingPalletConfigOpPercent',
                minCommission: 'PalletStakingPalletConfigOpPerbill',
            },
            chill_other: {
                controller: 'AccountId32',
            },
            force_apply_min_commission: {
                validatorStash: 'AccountId32',
            },
            set_min_commission: {
                _alias: {
                    new_: 'new',
                },
                new_: 'Perbill',
            },
        },
    },
    /**
     * Lookup198: pallet_staking::pallet::pallet::ConfigOp<T>
     **/
    PalletStakingPalletConfigOpU128: {
        _enum: {
            Noop: 'Null',
            Set: 'u128',
            Remove: 'Null',
        },
    },
    /**
     * Lookup199: pallet_staking::pallet::pallet::ConfigOp<T>
     **/
    PalletStakingPalletConfigOpU32: {
        _enum: {
            Noop: 'Null',
            Set: 'u32',
            Remove: 'Null',
        },
    },
    /**
     * Lookup200: pallet_staking::pallet::pallet::ConfigOp<sp_arithmetic::per_things::Percent>
     **/
    PalletStakingPalletConfigOpPercent: {
        _enum: {
            Noop: 'Null',
            Set: 'Percent',
            Remove: 'Null',
        },
    },
    /**
     * Lookup201: pallet_staking::pallet::pallet::ConfigOp<sp_arithmetic::per_things::Perbill>
     **/
    PalletStakingPalletConfigOpPerbill: {
        _enum: {
            Noop: 'Null',
            Set: 'Perbill',
            Remove: 'Null',
        },
    },
    /**
     * Lookup202: pallet_staking::pallet::pallet::Error<T>
     **/
    PalletStakingPalletError: {
        _enum: [
            'NotController',
            'NotStash',
            'AlreadyBonded',
            'AlreadyPaired',
            'EmptyTargets',
            'DuplicateIndex',
            'InvalidSlashIndex',
            'InsufficientBond',
            'NoMoreChunks',
            'NoUnlockChunk',
            'FundedTarget',
            'InvalidEraToReward',
            'InvalidNumberOfNominations',
            'NotSortedAndUnique',
            'AlreadyClaimed',
            'IncorrectHistoryDepth',
            'IncorrectSlashingSpans',
            'BadState',
            'TooManyTargets',
            'BadTarget',
            'CannotChillOther',
            'TooManyNominators',
            'TooManyValidators',
            'CommissionTooLow',
            'BoundNotMet',
        ],
    },
    /**
     * Lookup203: sp_staking::offence::OffenceDetails<sp_core::crypto::AccountId32, Offender>
     **/
    SpStakingOffenceOffenceDetails: {
        offender: '(AccountId32,PalletStakingExposure)',
        reporters: 'Vec<AccountId32>',
    },
    /**
     * Lookup208: creditcoin_node_runtime::opaque::SessionKeys
     **/
    CreditcoinNodeRuntimeOpaqueSessionKeys: {
        grandpa: 'SpConsensusGrandpaAppPublic',
        babe: 'SpConsensusBabeAppPublic',
        imOnline: 'PalletImOnlineSr25519AppSr25519Public',
    },
    /**
     * Lookup210: sp_core::crypto::KeyTypeId
     **/
    SpCoreCryptoKeyTypeId: '[u8;4]',
    /**
     * Lookup211: pallet_session::pallet::Call<T>
     **/
    PalletSessionCall: {
        _enum: {
            set_keys: {
                _alias: {
                    keys_: 'keys',
                },
                keys_: 'CreditcoinNodeRuntimeOpaqueSessionKeys',
                proof: 'Bytes',
            },
            purge_keys: 'Null',
        },
    },
    /**
     * Lookup212: pallet_session::pallet::Error<T>
     **/
    PalletSessionError: {
        _enum: ['InvalidProof', 'NoAssociatedValidatorId', 'DuplicatedKey', 'NoKeys', 'NoAccount'],
    },
    /**
     * Lookup213: pallet_grandpa::StoredState<N>
     **/
    PalletGrandpaStoredState: {
        _enum: {
            Live: 'Null',
            PendingPause: {
                scheduledAt: 'u32',
                delay: 'u32',
            },
            Paused: 'Null',
            PendingResume: {
                scheduledAt: 'u32',
                delay: 'u32',
            },
        },
    },
    /**
     * Lookup214: pallet_grandpa::StoredPendingChange<N, Limit>
     **/
    PalletGrandpaStoredPendingChange: {
        scheduledAt: 'u32',
        delay: 'u32',
        nextAuthorities: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
        forced: 'Option<u32>',
    },
    /**
     * Lookup216: pallet_grandpa::pallet::Call<T>
     **/
    PalletGrandpaCall: {
        _enum: {
            report_equivocation: {
                equivocationProof: 'SpConsensusGrandpaEquivocationProof',
                keyOwnerProof: 'SpSessionMembershipProof',
            },
            report_equivocation_unsigned: {
                equivocationProof: 'SpConsensusGrandpaEquivocationProof',
                keyOwnerProof: 'SpSessionMembershipProof',
            },
            note_stalled: {
                delay: 'u32',
                bestFinalizedBlockNumber: 'u32',
            },
        },
    },
    /**
     * Lookup217: sp_consensus_grandpa::EquivocationProof<primitive_types::H256, N>
     **/
    SpConsensusGrandpaEquivocationProof: {
        setId: 'u64',
        equivocation: 'SpConsensusGrandpaEquivocation',
    },
    /**
     * Lookup218: sp_consensus_grandpa::Equivocation<primitive_types::H256, N>
     **/
    SpConsensusGrandpaEquivocation: {
        _enum: {
            Prevote: 'FinalityGrandpaEquivocationPrevote',
            Precommit: 'FinalityGrandpaEquivocationPrecommit',
        },
    },
    /**
     * Lookup219: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Prevote<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
     **/
    FinalityGrandpaEquivocationPrevote: {
        roundNumber: 'u64',
        identity: 'SpConsensusGrandpaAppPublic',
        first: '(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)',
        second: '(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)',
    },
    /**
     * Lookup220: finality_grandpa::Prevote<primitive_types::H256, N>
     **/
    FinalityGrandpaPrevote: {
        targetHash: 'H256',
        targetNumber: 'u32',
    },
    /**
     * Lookup221: sp_consensus_grandpa::app::Signature
     **/
    SpConsensusGrandpaAppSignature: 'SpCoreEd25519Signature',
    /**
     * Lookup222: sp_core::ed25519::Signature
     **/
    SpCoreEd25519Signature: '[u8;64]',
    /**
     * Lookup224: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Precommit<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
     **/
    FinalityGrandpaEquivocationPrecommit: {
        roundNumber: 'u64',
        identity: 'SpConsensusGrandpaAppPublic',
        first: '(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)',
        second: '(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)',
    },
    /**
     * Lookup225: finality_grandpa::Precommit<primitive_types::H256, N>
     **/
    FinalityGrandpaPrecommit: {
        targetHash: 'H256',
        targetNumber: 'u32',
    },
    /**
     * Lookup227: pallet_grandpa::pallet::Error<T>
     **/
    PalletGrandpaError: {
        _enum: [
            'PauseFailed',
            'ResumeFailed',
            'ChangePending',
            'TooSoon',
            'InvalidKeyOwnershipProof',
            'InvalidEquivocationProof',
            'DuplicateOffenceReport',
        ],
    },
    /**
     * Lookup231: pallet_im_online::BoundedOpaqueNetworkState<PeerIdEncodingLimit, MultiAddrEncodingLimit, AddressesLimit>
     **/
    PalletImOnlineBoundedOpaqueNetworkState: {
        peerId: 'Bytes',
        externalAddresses: 'Vec<Bytes>',
    },
    /**
     * Lookup235: pallet_im_online::pallet::Call<T>
     **/
    PalletImOnlineCall: {
        _enum: {
            heartbeat: {
                heartbeat: 'PalletImOnlineHeartbeat',
                signature: 'PalletImOnlineSr25519AppSr25519Signature',
            },
        },
    },
    /**
     * Lookup236: pallet_im_online::Heartbeat<BlockNumber>
     **/
    PalletImOnlineHeartbeat: {
        blockNumber: 'u32',
        networkState: 'SpCoreOffchainOpaqueNetworkState',
        sessionIndex: 'u32',
        authorityIndex: 'u32',
        validatorsLen: 'u32',
    },
    /**
     * Lookup237: sp_core::offchain::OpaqueNetworkState
     **/
    SpCoreOffchainOpaqueNetworkState: {
        peerId: 'OpaquePeerId',
        externalAddresses: 'Vec<OpaqueMultiaddr>',
    },
    /**
     * Lookup241: pallet_im_online::sr25519::app_sr25519::Signature
     **/
    PalletImOnlineSr25519AppSr25519Signature: 'SpCoreSr25519Signature',
    /**
     * Lookup242: sp_core::sr25519::Signature
     **/
    SpCoreSr25519Signature: '[u8;64]',
    /**
     * Lookup243: pallet_im_online::pallet::Error<T>
     **/
    PalletImOnlineError: {
        _enum: ['InvalidKey', 'DuplicatedHeartbeat'],
    },
    /**
     * Lookup244: pallet_bags_list::list::Node<T, I>
     **/
    PalletBagsListListNode: {
        id: 'AccountId32',
        prev: 'Option<AccountId32>',
        next: 'Option<AccountId32>',
        bagUpper: 'u64',
        score: 'u64',
    },
    /**
     * Lookup245: pallet_bags_list::list::Bag<T, I>
     **/
    PalletBagsListListBag: {
        head: 'Option<AccountId32>',
        tail: 'Option<AccountId32>',
    },
    /**
     * Lookup246: pallet_bags_list::pallet::Call<T, I>
     **/
    PalletBagsListCall: {
        _enum: {
            rebag: {
                dislocated: 'MultiAddress',
            },
            put_in_front_of: {
                lighter: 'MultiAddress',
            },
        },
    },
    /**
     * Lookup248: pallet_bags_list::pallet::Error<T, I>
     **/
    PalletBagsListError: {
        _enum: {
            List: 'PalletBagsListListListError',
        },
    },
    /**
     * Lookup249: pallet_bags_list::list::ListError
     **/
    PalletBagsListListListError: {
        _enum: ['Duplicate', 'NotHeavier', 'NotInSameBag', 'NodeNotFound'],
    },
    /**
     * Lookup251: pallet_transaction_payment::Releases
     **/
    PalletTransactionPaymentReleases: {
        _enum: ['V1Ancient', 'V2'],
    },
    /**
     * Lookup252: pallet_sudo::pallet::Call<T>
     **/
    PalletSudoCall: {
        _enum: {
            sudo: {
                call: 'Call',
            },
            sudo_unchecked_weight: {
                call: 'Call',
                weight: 'SpWeightsWeightV2Weight',
            },
            set_key: {
                _alias: {
                    new_: 'new',
                },
                new_: 'MultiAddress',
            },
            sudo_as: {
                who: 'MultiAddress',
                call: 'Call',
            },
        },
    },
    /**
     * Lookup254: pallet_creditcoin::pallet::Call<T>
     **/
    PalletCreditcoinCall: {
        _enum: {
            claim_legacy_wallet: {
                publicKey: 'SpCoreEcdsaPublic',
            },
            register_address: {
                blockchain: 'PalletCreditcoinBlockchain',
                address: 'Bytes',
                ownershipProof: 'SpCoreEcdsaSignature',
            },
            add_ask_order: {
                addressId: 'H256',
                terms: 'PalletCreditcoinLoanTerms',
                expirationBlock: 'u32',
                guid: 'Bytes',
            },
            add_bid_order: {
                addressId: 'H256',
                terms: 'PalletCreditcoinLoanTerms',
                expirationBlock: 'u32',
                guid: 'Bytes',
            },
            add_offer: {
                askOrderId: 'PalletCreditcoinAskOrderId',
                bidOrderId: 'PalletCreditcoinBidOrderId',
                expirationBlock: 'u32',
            },
            add_deal_order: {
                offerId: 'PalletCreditcoinOfferId',
                expirationBlock: 'u32',
            },
            lock_deal_order: {
                dealOrderId: 'PalletCreditcoinDealOrderId',
            },
            fund_deal_order: {
                dealOrderId: 'PalletCreditcoinDealOrderId',
                transferId: 'H256',
            },
            register_deal_order: {
                lenderAddressId: 'H256',
                borrowerAddressId: 'H256',
                terms: 'PalletCreditcoinLoanTerms',
                expirationBlock: 'u32',
                askGuid: 'Bytes',
                bidGuid: 'Bytes',
                borrowerKey: 'SpRuntimeMultiSigner',
                borrowerSignature: 'SpRuntimeMultiSignature',
            },
            close_deal_order: {
                dealOrderId: 'PalletCreditcoinDealOrderId',
                transferId: 'H256',
            },
            __Unused10: 'Null',
            register_funding_transfer: {
                transferKind: 'PalletCreditcoinTransferKind',
                dealOrderId: 'PalletCreditcoinDealOrderId',
                blockchainTxId: 'Bytes',
            },
            register_repayment_transfer: {
                transferKind: 'PalletCreditcoinTransferKind',
                repaymentAmount: 'U256',
                dealOrderId: 'PalletCreditcoinDealOrderId',
                blockchainTxId: 'Bytes',
            },
            exempt: {
                dealOrderId: 'PalletCreditcoinDealOrderId',
            },
            __Unused14: 'Null',
            __Unused15: 'Null',
            persist_task_output: {
                deadline: 'u32',
                taskOutput: 'PalletCreditcoinTaskOutput',
            },
            fail_task: {
                deadline: 'u32',
                taskId: 'PalletCreditcoinTaskId',
                cause: 'PalletCreditcoinOcwErrorsVerificationFailureCause',
            },
            add_authority: {
                who: 'AccountId32',
            },
            __Unused19: 'Null',
            __Unused20: 'Null',
            remove_authority: {
                who: 'AccountId32',
            },
            register_address_v2: {
                blockchain: 'PalletCreditcoinBlockchain',
                address: 'Bytes',
                ownershipProof: 'PalletCreditcoinOwnershipProof',
            },
        },
    },
    /**
     * Lookup255: sp_core::ecdsa::Public
     **/
    SpCoreEcdsaPublic: '[u8;33]',
    /**
     * Lookup257: sp_core::ecdsa::Signature
     **/
    SpCoreEcdsaSignature: '[u8;65]',
    /**
     * Lookup259: sp_runtime::MultiSigner
     **/
    SpRuntimeMultiSigner: {
        _enum: {
            Ed25519: 'SpCoreEd25519Public',
            Sr25519: 'SpCoreSr25519Public',
            Ecdsa: 'SpCoreEcdsaPublic',
        },
    },
    /**
     * Lookup260: sp_runtime::MultiSignature
     **/
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: 'SpCoreEd25519Signature',
            Sr25519: 'SpCoreSr25519Signature',
            Ecdsa: 'SpCoreEcdsaSignature',
        },
    },
    /**
     * Lookup261: pallet_creditcoin::types::TaskOutput<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
     **/
    PalletCreditcoinTaskOutput: {
        _enum: {
            VerifyTransfer: '(H256,PalletCreditcoinTransfer)',
        },
    },
    /**
     * Lookup262: pallet_creditcoin::types::TaskId<primitive_types::H256>
     **/
    PalletCreditcoinTaskId: {
        _enum: {
            VerifyTransfer: 'H256',
        },
    },
    /**
     * Lookup263: pallet_creditcoin::types::OwnershipProof
     **/
    PalletCreditcoinOwnershipProof: {
        _enum: {
            PersonalSign: 'SpCoreEcdsaSignature',
            EthSign: 'SpCoreEcdsaSignature',
        },
    },
    /**
     * Lookup264: pallet_scheduler::pallet::Call<T>
     **/
    PalletSchedulerCall: {
        _enum: {
            schedule: {
                when: 'u32',
                maybePeriodic: 'Option<(u32,u32)>',
                priority: 'u8',
                call: 'Call',
            },
            cancel: {
                when: 'u32',
                index: 'u32',
            },
            schedule_named: {
                id: '[u8;32]',
                when: 'u32',
                maybePeriodic: 'Option<(u32,u32)>',
                priority: 'u8',
                call: 'Call',
            },
            cancel_named: {
                id: '[u8;32]',
            },
            schedule_after: {
                after: 'u32',
                maybePeriodic: 'Option<(u32,u32)>',
                priority: 'u8',
                call: 'Call',
            },
            schedule_named_after: {
                id: '[u8;32]',
                after: 'u32',
                maybePeriodic: 'Option<(u32,u32)>',
                priority: 'u8',
                call: 'Call',
            },
        },
    },
    /**
     * Lookup266: pallet_identity::pallet::Call<T>
     **/
    PalletIdentityCall: {
        _enum: {
            add_registrar: {
                account: 'MultiAddress',
            },
            set_identity: {
                info: 'PalletIdentityIdentityInfo',
            },
            set_subs: {
                subs: 'Vec<(AccountId32,Data)>',
            },
            clear_identity: 'Null',
            request_judgement: {
                regIndex: 'Compact<u32>',
                maxFee: 'Compact<u128>',
            },
            cancel_request: {
                regIndex: 'u32',
            },
            set_fee: {
                index: 'Compact<u32>',
                fee: 'Compact<u128>',
            },
            set_account_id: {
                _alias: {
                    new_: 'new',
                },
                index: 'Compact<u32>',
                new_: 'MultiAddress',
            },
            set_fields: {
                index: 'Compact<u32>',
                fields: 'PalletIdentityBitFlags',
            },
            provide_judgement: {
                regIndex: 'Compact<u32>',
                target: 'MultiAddress',
                judgement: 'PalletIdentityJudgement',
                identity: 'H256',
            },
            kill_identity: {
                target: 'MultiAddress',
            },
            add_sub: {
                sub: 'MultiAddress',
                data: 'Data',
            },
            rename_sub: {
                sub: 'MultiAddress',
                data: 'Data',
            },
            remove_sub: {
                sub: 'MultiAddress',
            },
            quit_sub: 'Null',
        },
    },
    /**
     * Lookup267: pallet_identity::types::IdentityInfo<FieldLimit>
     **/
    PalletIdentityIdentityInfo: {
        additional: 'Vec<(Data,Data)>',
        display: 'Data',
        legal: 'Data',
        web: 'Data',
        riot: 'Data',
        email: 'Data',
        pgpFingerprint: 'Option<[u8;20]>',
        image: 'Data',
        twitter: 'Data',
    },
    /**
     * Lookup303: pallet_identity::types::BitFlags<pallet_identity::types::IdentityField>
     **/
    PalletIdentityBitFlags: {
        _bitLength: 64,
        Display: 1,
        Legal: 2,
        Web: 4,
        Riot: 8,
        Email: 16,
        PgpFingerprint: 32,
        Image: 64,
        Twitter: 128,
    },
    /**
     * Lookup304: pallet_identity::types::IdentityField
     **/
    PalletIdentityIdentityField: {
        _enum: [
            '__Unused0',
            'Display',
            'Legal',
            '__Unused3',
            'Web',
            '__Unused5',
            '__Unused6',
            '__Unused7',
            'Riot',
            '__Unused9',
            '__Unused10',
            '__Unused11',
            '__Unused12',
            '__Unused13',
            '__Unused14',
            '__Unused15',
            'Email',
            '__Unused17',
            '__Unused18',
            '__Unused19',
            '__Unused20',
            '__Unused21',
            '__Unused22',
            '__Unused23',
            '__Unused24',
            '__Unused25',
            '__Unused26',
            '__Unused27',
            '__Unused28',
            '__Unused29',
            '__Unused30',
            '__Unused31',
            'PgpFingerprint',
            '__Unused33',
            '__Unused34',
            '__Unused35',
            '__Unused36',
            '__Unused37',
            '__Unused38',
            '__Unused39',
            '__Unused40',
            '__Unused41',
            '__Unused42',
            '__Unused43',
            '__Unused44',
            '__Unused45',
            '__Unused46',
            '__Unused47',
            '__Unused48',
            '__Unused49',
            '__Unused50',
            '__Unused51',
            '__Unused52',
            '__Unused53',
            '__Unused54',
            '__Unused55',
            '__Unused56',
            '__Unused57',
            '__Unused58',
            '__Unused59',
            '__Unused60',
            '__Unused61',
            '__Unused62',
            '__Unused63',
            'Image',
            '__Unused65',
            '__Unused66',
            '__Unused67',
            '__Unused68',
            '__Unused69',
            '__Unused70',
            '__Unused71',
            '__Unused72',
            '__Unused73',
            '__Unused74',
            '__Unused75',
            '__Unused76',
            '__Unused77',
            '__Unused78',
            '__Unused79',
            '__Unused80',
            '__Unused81',
            '__Unused82',
            '__Unused83',
            '__Unused84',
            '__Unused85',
            '__Unused86',
            '__Unused87',
            '__Unused88',
            '__Unused89',
            '__Unused90',
            '__Unused91',
            '__Unused92',
            '__Unused93',
            '__Unused94',
            '__Unused95',
            '__Unused96',
            '__Unused97',
            '__Unused98',
            '__Unused99',
            '__Unused100',
            '__Unused101',
            '__Unused102',
            '__Unused103',
            '__Unused104',
            '__Unused105',
            '__Unused106',
            '__Unused107',
            '__Unused108',
            '__Unused109',
            '__Unused110',
            '__Unused111',
            '__Unused112',
            '__Unused113',
            '__Unused114',
            '__Unused115',
            '__Unused116',
            '__Unused117',
            '__Unused118',
            '__Unused119',
            '__Unused120',
            '__Unused121',
            '__Unused122',
            '__Unused123',
            '__Unused124',
            '__Unused125',
            '__Unused126',
            '__Unused127',
            'Twitter',
        ],
    },
    /**
     * Lookup305: pallet_identity::types::Judgement<Balance>
     **/
    PalletIdentityJudgement: {
        _enum: {
            Unknown: 'Null',
            FeePaid: 'u128',
            Reasonable: 'Null',
            KnownGood: 'Null',
            OutOfDate: 'Null',
            LowQuality: 'Null',
            Erroneous: 'Null',
        },
    },
    /**
     * Lookup306: pallet_utility::pallet::Call<T>
     **/
    PalletUtilityCall: {
        _enum: {
            batch: {
                calls: 'Vec<Call>',
            },
            as_derivative: {
                index: 'u16',
                call: 'Call',
            },
            batch_all: {
                calls: 'Vec<Call>',
            },
            dispatch_as: {
                asOrigin: 'CreditcoinNodeRuntimeOriginCaller',
                call: 'Call',
            },
            force_batch: {
                calls: 'Vec<Call>',
            },
            with_weight: {
                call: 'Call',
                weight: 'SpWeightsWeightV2Weight',
            },
        },
    },
    /**
     * Lookup308: creditcoin_node_runtime::OriginCaller
     **/
    CreditcoinNodeRuntimeOriginCaller: {
        _enum: {
            system: 'FrameSupportDispatchRawOrigin',
            Void: 'SpCoreVoid',
        },
    },
    /**
     * Lookup309: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
     **/
    FrameSupportDispatchRawOrigin: {
        _enum: {
            Root: 'Null',
            Signed: 'AccountId32',
            None: 'Null',
        },
    },
    /**
     * Lookup310: sp_core::Void
     **/
    SpCoreVoid: 'Null',
    /**
     * Lookup311: pallet_proxy::pallet::Call<T>
     **/
    PalletProxyCall: {
        _enum: {
            proxy: {
                real: 'MultiAddress',
                forceProxyType: 'Option<Null>',
                call: 'Call',
            },
            add_proxy: {
                delegate: 'MultiAddress',
                proxyType: 'Null',
                delay: 'u32',
            },
            remove_proxy: {
                delegate: 'MultiAddress',
                proxyType: 'Null',
                delay: 'u32',
            },
            remove_proxies: 'Null',
            create_pure: {
                proxyType: 'Null',
                delay: 'u32',
                index: 'u16',
            },
            kill_pure: {
                spawner: 'MultiAddress',
                proxyType: 'Null',
                index: 'u16',
                height: 'Compact<u32>',
                extIndex: 'Compact<u32>',
            },
            announce: {
                real: 'MultiAddress',
                callHash: 'H256',
            },
            remove_announcement: {
                real: 'MultiAddress',
                callHash: 'H256',
            },
            reject_announcement: {
                delegate: 'MultiAddress',
                callHash: 'H256',
            },
            proxy_announced: {
                delegate: 'MultiAddress',
                real: 'MultiAddress',
                forceProxyType: 'Option<Null>',
                call: 'Call',
            },
        },
    },
    /**
     * Lookup313: pallet_fast_unstake::pallet::Call<T>
     **/
    PalletFastUnstakeCall: {
        _enum: {
            register_fast_unstake: 'Null',
            deregister: 'Null',
            control: {
                erasToCheck: 'u32',
            },
        },
    },
    /**
     * Lookup314: pallet_nomination_pools::pallet::Call<T>
     **/
    PalletNominationPoolsCall: {
        _enum: {
            join: {
                amount: 'Compact<u128>',
                poolId: 'u32',
            },
            bond_extra: {
                extra: 'PalletNominationPoolsBondExtra',
            },
            claim_payout: 'Null',
            unbond: {
                memberAccount: 'MultiAddress',
                unbondingPoints: 'Compact<u128>',
            },
            pool_withdraw_unbonded: {
                poolId: 'u32',
                numSlashingSpans: 'u32',
            },
            withdraw_unbonded: {
                memberAccount: 'MultiAddress',
                numSlashingSpans: 'u32',
            },
            create: {
                amount: 'Compact<u128>',
                root: 'MultiAddress',
                nominator: 'MultiAddress',
                bouncer: 'MultiAddress',
            },
            create_with_pool_id: {
                amount: 'Compact<u128>',
                root: 'MultiAddress',
                nominator: 'MultiAddress',
                bouncer: 'MultiAddress',
                poolId: 'u32',
            },
            nominate: {
                poolId: 'u32',
                validators: 'Vec<AccountId32>',
            },
            set_state: {
                poolId: 'u32',
                state: 'PalletNominationPoolsPoolState',
            },
            set_metadata: {
                poolId: 'u32',
                metadata: 'Bytes',
            },
            set_configs: {
                minJoinBond: 'PalletNominationPoolsConfigOpU128',
                minCreateBond: 'PalletNominationPoolsConfigOpU128',
                maxPools: 'PalletNominationPoolsConfigOpU32',
                maxMembers: 'PalletNominationPoolsConfigOpU32',
                maxMembersPerPool: 'PalletNominationPoolsConfigOpU32',
                globalMaxCommission: 'PalletNominationPoolsConfigOpPerbill',
            },
            update_roles: {
                poolId: 'u32',
                newRoot: 'PalletNominationPoolsConfigOpAccountId32',
                newNominator: 'PalletNominationPoolsConfigOpAccountId32',
                newBouncer: 'PalletNominationPoolsConfigOpAccountId32',
            },
            chill: {
                poolId: 'u32',
            },
            bond_extra_other: {
                member: 'MultiAddress',
                extra: 'PalletNominationPoolsBondExtra',
            },
            set_claim_permission: {
                permission: 'PalletNominationPoolsClaimPermission',
            },
            claim_payout_other: {
                other: 'AccountId32',
            },
            set_commission: {
                poolId: 'u32',
                newCommission: 'Option<(Perbill,AccountId32)>',
            },
            set_commission_max: {
                poolId: 'u32',
                maxCommission: 'Perbill',
            },
            set_commission_change_rate: {
                poolId: 'u32',
                changeRate: 'PalletNominationPoolsCommissionChangeRate',
            },
            claim_commission: {
                poolId: 'u32',
            },
        },
    },
    /**
     * Lookup315: pallet_nomination_pools::BondExtra<Balance>
     **/
    PalletNominationPoolsBondExtra: {
        _enum: {
            FreeBalance: 'u128',
            Rewards: 'Null',
        },
    },
    /**
     * Lookup316: pallet_nomination_pools::ConfigOp<T>
     **/
    PalletNominationPoolsConfigOpU128: {
        _enum: {
            Noop: 'Null',
            Set: 'u128',
            Remove: 'Null',
        },
    },
    /**
     * Lookup317: pallet_nomination_pools::ConfigOp<T>
     **/
    PalletNominationPoolsConfigOpU32: {
        _enum: {
            Noop: 'Null',
            Set: 'u32',
            Remove: 'Null',
        },
    },
    /**
     * Lookup318: pallet_nomination_pools::ConfigOp<sp_arithmetic::per_things::Perbill>
     **/
    PalletNominationPoolsConfigOpPerbill: {
        _enum: {
            Noop: 'Null',
            Set: 'Perbill',
            Remove: 'Null',
        },
    },
    /**
     * Lookup319: pallet_nomination_pools::ConfigOp<sp_core::crypto::AccountId32>
     **/
    PalletNominationPoolsConfigOpAccountId32: {
        _enum: {
            Noop: 'Null',
            Set: 'AccountId32',
            Remove: 'Null',
        },
    },
    /**
     * Lookup320: pallet_nomination_pools::ClaimPermission
     **/
    PalletNominationPoolsClaimPermission: {
        _enum: ['Permissioned', 'PermissionlessCompound', 'PermissionlessWithdraw', 'PermissionlessAll'],
    },
    /**
     * Lookup321: pallet_sudo::pallet::Error<T>
     **/
    PalletSudoError: {
        _enum: ['RequireSudo'],
    },
    /**
     * Lookup324: pallet_creditcoin::types::collect_coins::CollectedCoins<primitive_types::H256, Balance>
     **/
    PalletCreditcoinCollectCoinsCollectedCoins: {
        to: 'H256',
        amount: 'u128',
        txId: 'Bytes',
        contractType: 'PalletCreditcoinCollectCoinsContractType',
    },
    /**
     * Lookup325: pallet_creditcoin::types::collect_coins::ContractType
     **/
    PalletCreditcoinCollectCoinsContractType: {
        _enum: ['GCRE', 'GATE'],
    },
    /**
     * Lookup326: pallet_creditcoin::ocw::tasks::collect_coins::DeployedContract
     **/
    PalletCreditcoinOcwTasksCollectCoinsDeployedContract: {
        address: 'H160',
        chain: 'PalletCreditcoinBlockchain',
    },
    /**
     * Lookup328: pallet_creditcoin::types::cleanup::StorageCleanupState<BlockNumber>
     **/
    PalletCreditcoinCleanupStorageCleanupState: {
        askOrders: 'PalletCreditcoinCleanupStorageItemCleanupState',
        bidOrders: 'PalletCreditcoinCleanupStorageItemCleanupState',
        offers: 'PalletCreditcoinCleanupStorageItemCleanupState',
    },
    /**
     * Lookup329: pallet_creditcoin::types::cleanup::StorageItemCleanupState<BlockNumber>
     **/
    PalletCreditcoinCleanupStorageItemCleanupState: {
        onBlock: 'u32',
        cursor: 'Option<Bytes>',
    },
    /**
     * Lookup332: pallet_creditcoin::types::BurnInfo<sp_core::crypto::AccountId32, Balance>
     **/
    PalletCreditcoinBurnInfo: {
        account: 'AccountId32',
        amount: 'u128',
        collector: 'AccountId32',
    },
    /**
     * Lookup333: pallet_creditcoin::pallet::Error<T>
     **/
    PalletCreditcoinError: {
        _enum: [
            'AddressAlreadyRegistered',
            'AddressAlreadyRegisteredByCaller',
            'NonExistentAddress',
            'NonExistentDealOrder',
            'NonExistentAskOrder',
            'NonExistentBidOrder',
            'NonExistentOffer',
            'NonExistentTransfer',
            'TransferAlreadyRegistered',
            'CollectCoinsAlreadyRegistered',
            'TransferAccountMismatch',
            'TransferDealOrderMismatch',
            'TransferAmountMismatch',
            'TransferAlreadyProcessed',
            'TransferAmountInsufficient',
            'MalformedTransfer',
            'UnsupportedTransferKind',
            'InsufficientAuthority',
            'DuplicateId',
            'NotAddressOwner',
            'OffchainSignedTxFailed',
            'NoLocalAcctForSignedTx',
            'RepaymentOrderNonZeroGain',
            'AddressBlockchainMismatch',
            'AlreadyAuthority',
            'NotAnAuthority',
            'DuplicateOffer',
            'DealNotFunded',
            'DealOrderAlreadyFunded',
            'DealOrderAlreadyClosed',
            'DealOrderAlreadyLocked',
            'DealOrderMustBeLocked',
            'DuplicateDealOrder',
            'DealOrderExpired',
            'AskOrderExpired',
            'BidOrderExpired',
            'OfferExpired',
            'AskBidMismatch',
            'SameOwner',
            'InvalidSignature',
            'NotBorrower',
            'MalformedDealOrder',
            'NotLender',
            'RepaymentOrderUnsupported',
            'NotLegacyWalletOwner',
            'LegacyWalletNotFound',
            'LegacyBalanceKeeperMissing',
            'GuidAlreadyUsed',
            'InvalidTermLength',
            'MalformedExternalAddress',
            'AddressFormatNotSupported',
            'OwnershipNotSatisfied',
            'CurrencyAlreadyRegistered',
            'EthSignExternalAddressGenerationFailed',
            'EthSignPublicKeyRecoveryFailed',
            'PersonalSignExternalAddressGenerationFailed',
            'PersonalSignPublicKeyRecoveryFailed',
            'UnsupportedBlockchain',
            'BurnGATEFaucetNotSet',
            'BurnGATEInsufficientFaucetBalance',
            'BurnSettlementError',
            'BurnInsufficientFunds',
        ],
    },
    /**
     * Lookup335: pallet_difficulty::DifficultyAndTimestamp<Moment>
     **/
    PalletDifficultyDifficultyAndTimestamp: {
        difficulty: 'U256',
        timestamp: 'u64',
    },
    /**
     * Lookup338: pallet_difficulty::pallet::Error<T>
     **/
    PalletDifficultyError: {
        _enum: ['ZeroTargetTime', 'ZeroAdjustmentPeriod', 'NegativeAdjustmentPeriod'],
    },
    /**
     * Lookup341: pallet_scheduler::Scheduled<Name, frame_support::traits::preimages::Bounded<creditcoin_node_runtime::RuntimeCall>, BlockNumber, creditcoin_node_runtime::OriginCaller, sp_core::crypto::AccountId32>
     **/
    PalletSchedulerScheduled: {
        maybeId: 'Option<[u8;32]>',
        priority: 'u8',
        call: 'FrameSupportPreimagesBounded',
        maybePeriodic: 'Option<(u32,u32)>',
        origin: 'CreditcoinNodeRuntimeOriginCaller',
    },
    /**
     * Lookup342: frame_support::traits::preimages::Bounded<creditcoin_node_runtime::RuntimeCall>
     **/
    FrameSupportPreimagesBounded: {
        _enum: {
            Legacy: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
            },
            Inline: 'Bytes',
            Lookup: {
                _alias: {
                    hash_: 'hash',
                },
                hash_: 'H256',
                len: 'u32',
            },
        },
    },
    /**
     * Lookup345: pallet_scheduler::pallet::Error<T>
     **/
    PalletSchedulerError: {
        _enum: ['FailedToSchedule', 'NotFound', 'TargetBlockNumberInPast', 'RescheduleNoChange', 'Named'],
    },
    /**
     * Lookup346: pallet_creditcoin::types::Task<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
     **/
    PalletCreditcoinTask: {
        _enum: {
            VerifyTransfer: 'PalletCreditcoinTransferUnverifiedTransfer',
        },
    },
    /**
     * Lookup347: pallet_creditcoin::types::transfer::UnverifiedTransfer<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
     **/
    PalletCreditcoinTransferUnverifiedTransfer: {
        transfer: 'PalletCreditcoinTransfer',
        fromExternal: 'Bytes',
        toExternal: 'Bytes',
        deadline: 'u32',
    },
    /**
     * Lookup348: pallet_offchain_task_scheduler::pallet::Error<T>
     **/
    PalletOffchainTaskSchedulerError: {
        _enum: ['OffchainSignedTxFailed', 'NoLocalAcctForSignedTx'],
    },
    /**
     * Lookup349: pallet_identity::types::Registration<Balance, MaxJudgements, MaxAdditionalFields>
     **/
    PalletIdentityRegistration: {
        judgements: 'Vec<(u32,PalletIdentityJudgement)>',
        deposit: 'u128',
        info: 'PalletIdentityIdentityInfo',
    },
    /**
     * Lookup357: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32>
     **/
    PalletIdentityRegistrarInfo: {
        account: 'AccountId32',
        fee: 'u128',
        fields: 'PalletIdentityBitFlags',
    },
    /**
     * Lookup359: pallet_identity::pallet::Error<T>
     **/
    PalletIdentityError: {
        _enum: [
            'TooManySubAccounts',
            'NotFound',
            'NotNamed',
            'EmptyIndex',
            'FeeChanged',
            'NoIdentity',
            'StickyJudgement',
            'JudgementGiven',
            'InvalidJudgement',
            'InvalidIndex',
            'InvalidTarget',
            'TooManyFields',
            'TooManyRegistrars',
            'AlreadyClaimed',
            'NotSub',
            'NotOwned',
            'JudgementForDifferentIdentity',
            'JudgementPaymentFailed',
        ],
    },
    /**
     * Lookup360: pallet_utility::pallet::Error<T>
     **/
    PalletUtilityError: {
        _enum: ['TooManyCalls'],
    },
    /**
     * Lookup363: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, ProxyType, BlockNumber>
     **/
    PalletProxyProxyDefinition: {
        delegate: 'AccountId32',
        proxyType: 'Null',
        delay: 'u32',
    },
    /**
     * Lookup367: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
     **/
    PalletProxyAnnouncement: {
        real: 'AccountId32',
        callHash: 'H256',
        height: 'u32',
    },
    /**
     * Lookup369: pallet_proxy::pallet::Error<T>
     **/
    PalletProxyError: {
        _enum: [
            'TooMany',
            'NotFound',
            'NotProxy',
            'Unproxyable',
            'Duplicate',
            'NoPermission',
            'Unannounced',
            'NoSelfProxy',
        ],
    },
    /**
     * Lookup370: pallet_fast_unstake::types::UnstakeRequest<T>
     **/
    PalletFastUnstakeUnstakeRequest: {
        stashes: 'Vec<(AccountId32,u128)>',
        checked: 'Vec<u32>',
    },
    /**
     * Lookup373: pallet_fast_unstake::pallet::Error<T>
     **/
    PalletFastUnstakeError: {
        _enum: ['NotController', 'AlreadyQueued', 'NotFullyBonded', 'NotQueued', 'AlreadyHead', 'CallNotAllowed'],
    },
    /**
     * Lookup374: pallet_nomination_pools::PoolMember<T>
     **/
    PalletNominationPoolsPoolMember: {
        poolId: 'u32',
        points: 'u128',
        lastRecordedRewardCounter: 'u128',
        unbondingEras: 'BTreeMap<u32, u128>',
    },
    /**
     * Lookup379: pallet_nomination_pools::BondedPoolInner<T>
     **/
    PalletNominationPoolsBondedPoolInner: {
        commission: 'PalletNominationPoolsCommission',
        memberCounter: 'u32',
        points: 'u128',
        roles: 'PalletNominationPoolsPoolRoles',
        state: 'PalletNominationPoolsPoolState',
    },
    /**
     * Lookup380: pallet_nomination_pools::Commission<T>
     **/
    PalletNominationPoolsCommission: {
        current: 'Option<(Perbill,AccountId32)>',
        max: 'Option<Perbill>',
        changeRate: 'Option<PalletNominationPoolsCommissionChangeRate>',
        throttleFrom: 'Option<u32>',
    },
    /**
     * Lookup383: pallet_nomination_pools::PoolRoles<sp_core::crypto::AccountId32>
     **/
    PalletNominationPoolsPoolRoles: {
        depositor: 'AccountId32',
        root: 'Option<AccountId32>',
        nominator: 'Option<AccountId32>',
        bouncer: 'Option<AccountId32>',
    },
    /**
     * Lookup384: pallet_nomination_pools::RewardPool<T>
     **/
    PalletNominationPoolsRewardPool: {
        lastRecordedRewardCounter: 'u128',
        lastRecordedTotalPayouts: 'u128',
        totalRewardsClaimed: 'u128',
        totalCommissionPending: 'u128',
        totalCommissionClaimed: 'u128',
    },
    /**
     * Lookup385: pallet_nomination_pools::SubPools<T>
     **/
    PalletNominationPoolsSubPools: {
        noEra: 'PalletNominationPoolsUnbondPool',
        withEra: 'BTreeMap<u32, PalletNominationPoolsUnbondPool>',
    },
    /**
     * Lookup386: pallet_nomination_pools::UnbondPool<T>
     **/
    PalletNominationPoolsUnbondPool: {
        points: 'u128',
        balance: 'u128',
    },
    /**
     * Lookup392: frame_support::PalletId
     **/
    FrameSupportPalletId: '[u8;8]',
    /**
     * Lookup393: pallet_nomination_pools::pallet::Error<T>
     **/
    PalletNominationPoolsError: {
        _enum: {
            PoolNotFound: 'Null',
            PoolMemberNotFound: 'Null',
            RewardPoolNotFound: 'Null',
            SubPoolsNotFound: 'Null',
            AccountBelongsToOtherPool: 'Null',
            FullyUnbonding: 'Null',
            MaxUnbondingLimit: 'Null',
            CannotWithdrawAny: 'Null',
            MinimumBondNotMet: 'Null',
            OverflowRisk: 'Null',
            NotDestroying: 'Null',
            NotNominator: 'Null',
            NotKickerOrDestroying: 'Null',
            NotOpen: 'Null',
            MaxPools: 'Null',
            MaxPoolMembers: 'Null',
            CanNotChangeState: 'Null',
            DoesNotHavePermission: 'Null',
            MetadataExceedsMaxLen: 'Null',
            Defensive: 'PalletNominationPoolsDefensiveError',
            PartialUnbondNotAllowedPermissionlessly: 'Null',
            MaxCommissionRestricted: 'Null',
            CommissionExceedsMaximum: 'Null',
            CommissionChangeThrottled: 'Null',
            CommissionChangeRateNotAllowed: 'Null',
            NoPendingCommission: 'Null',
            NoCommissionCurrentSet: 'Null',
            PoolIdInUse: 'Null',
            InvalidPoolId: 'Null',
            BondExtraRestricted: 'Null',
        },
    },
    /**
     * Lookup394: pallet_nomination_pools::pallet::DefensiveError
     **/
    PalletNominationPoolsDefensiveError: {
        _enum: [
            'NotEnoughSpaceInUnbondPool',
            'PoolNotFound',
            'RewardPoolNotFound',
            'SubPoolsNotFound',
            'BondedStashKilledPrematurely',
        ],
    },
    /**
     * Lookup397: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
     **/
    FrameSystemExtensionsCheckNonZeroSender: 'Null',
    /**
     * Lookup398: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
     **/
    FrameSystemExtensionsCheckSpecVersion: 'Null',
    /**
     * Lookup399: frame_system::extensions::check_tx_version::CheckTxVersion<T>
     **/
    FrameSystemExtensionsCheckTxVersion: 'Null',
    /**
     * Lookup400: frame_system::extensions::check_genesis::CheckGenesis<T>
     **/
    FrameSystemExtensionsCheckGenesis: 'Null',
    /**
     * Lookup403: frame_system::extensions::check_nonce::CheckNonce<T>
     **/
    FrameSystemExtensionsCheckNonce: 'Compact<u32>',
    /**
     * Lookup404: frame_system::extensions::check_weight::CheckWeight<T>
     **/
    FrameSystemExtensionsCheckWeight: 'Null',
    /**
     * Lookup405: pallet_transaction_payment::ChargeTransactionPayment<T>
     **/
    PalletTransactionPaymentChargeTransactionPayment: 'Compact<u128>',
    /**
     * Lookup406: creditcoin_node_runtime::Runtime
     **/
    CreditcoinNodeRuntimeRuntime: 'Null',
};
