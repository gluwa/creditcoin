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
     * Lookup31: pallet_transaction_payment::pallet::Event<T>
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
     * Lookup32: pallet_sudo::pallet::Event<T>
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
     * Lookup36: pallet_creditcoin::pallet::Event<T>
     **/
    PalletCreditcoinEvent: {
        _enum: {
            AddressRegistered: '(H256,PalletCreditcoinAddress)',
            CollectCoinsRegistered: '(H256,PalletCreditcoinCollectCoinsUnverifiedCollectedCoins)',
            TransferRegistered: '(H256,PalletCreditcoinTransfer)',
            TransferVerified: 'H256',
            CollectedCoinsMinted: '(H256,PalletCreditcoinCollectCoinsCollectedCoins)',
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
            CollectCoinsFailedVerification: '(H256,PalletCreditcoinOcwErrorsVerificationFailureCause)',
        },
    },
    /**
     * Lookup38: pallet_creditcoin::types::Address<sp_core::crypto::AccountId32>
     **/
    PalletCreditcoinAddress: {
        blockchain: 'PalletCreditcoinBlockchain',
        value: 'Bytes',
        owner: 'AccountId32',
    },
    /**
     * Lookup39: pallet_creditcoin::types::Blockchain
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
     * Lookup42: pallet_creditcoin::types::collect_coins::UnverifiedCollectedCoins
     **/
    PalletCreditcoinCollectCoinsUnverifiedCollectedCoins: {
        to: 'Bytes',
        txId: 'Bytes',
        contract: 'PalletCreditcoinOcwTasksCollectCoinsGCreContract',
    },
    /**
     * Lookup43: pallet_creditcoin::ocw::tasks::collect_coins::GCreContract
     **/
    PalletCreditcoinOcwTasksCollectCoinsGCreContract: {
        address: 'H160',
        chain: 'PalletCreditcoinBlockchain',
    },
    /**
     * Lookup47: pallet_creditcoin::types::transfer::Transfer<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
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
     * Lookup48: pallet_creditcoin::types::TransferKind
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
     * Lookup49: pallet_creditcoin::types::OrderId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinOrderId: {
        _enum: {
            Deal: 'PalletCreditcoinDealOrderId',
            Repayment: 'PalletCreditcoinRepaymentOrderId',
        },
    },
    /**
     * Lookup50: pallet_creditcoin::types::DealOrderId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinDealOrderId: '(u32,H256)',
    /**
     * Lookup51: pallet_creditcoin::types::RepaymentOrderId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinRepaymentOrderId: '(u32,H256)',
    /**
     * Lookup56: pallet_creditcoin::types::collect_coins::CollectedCoins<primitive_types::H256, Balance>
     **/
    PalletCreditcoinCollectCoinsCollectedCoins: {
        to: 'H256',
        amount: 'u128',
        txId: 'Bytes',
    },
    /**
     * Lookup57: pallet_creditcoin::types::AskOrderId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinAskOrderId: '(u32,H256)',
    /**
     * Lookup58: pallet_creditcoin::types::AskOrder<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256>
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
     * Lookup59: pallet_creditcoin::types::loan_terms::AskTerms
     **/
    PalletCreditcoinLoanTermsAskTerms: 'PalletCreditcoinLoanTerms',
    /**
     * Lookup60: pallet_creditcoin::types::loan_terms::LoanTerms
     **/
    PalletCreditcoinLoanTerms: {
        amount: 'U256',
        interestRate: 'PalletCreditcoinLoanTermsInterestRate',
        termLength: 'PalletCreditcoinLoanTermsDuration',
    },
    /**
     * Lookup61: pallet_creditcoin::types::loan_terms::InterestRate
     **/
    PalletCreditcoinLoanTermsInterestRate: {
        ratePerPeriod: 'u64',
        decimals: 'u64',
        period: 'PalletCreditcoinLoanTermsDuration',
        interestType: 'PalletCreditcoinLoanTermsInterestType',
    },
    /**
     * Lookup62: pallet_creditcoin::types::loan_terms::Duration
     **/
    PalletCreditcoinLoanTermsDuration: {
        secs: 'u64',
        nanos: 'u32',
    },
    /**
     * Lookup63: pallet_creditcoin::types::loan_terms::InterestType
     **/
    PalletCreditcoinLoanTermsInterestType: {
        _enum: ['Simple', 'Compound'],
    },
    /**
     * Lookup64: pallet_creditcoin::types::BidOrderId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinBidOrderId: '(u32,H256)',
    /**
     * Lookup65: pallet_creditcoin::types::BidOrder<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256>
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
     * Lookup66: pallet_creditcoin::types::loan_terms::BidTerms
     **/
    PalletCreditcoinLoanTermsBidTerms: 'PalletCreditcoinLoanTerms',
    /**
     * Lookup67: pallet_creditcoin::types::OfferId<BlockNum, primitive_types::H256>
     **/
    PalletCreditcoinOfferId: '(u32,H256)',
    /**
     * Lookup68: pallet_creditcoin::types::Offer<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256>
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
     * Lookup69: pallet_creditcoin::types::DealOrder<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
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
     * Lookup72: pallet_creditcoin::types::LegacySighash
     **/
    PalletCreditcoinLegacySighash: '[u8;60]',
    /**
     * Lookup74: pallet_creditcoin::ocw::errors::VerificationFailureCause
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
        ],
    },
    /**
     * Lookup75: pallet_rewards::pallet::Event<T>
     **/
    PalletRewardsEvent: {
        _enum: {
            RewardIssued: '(AccountId32,u128)',
        },
    },
    /**
     * Lookup76: pallet_scheduler::pallet::Event<T>
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
     * Lookup79: pallet_offchain_task_scheduler::pallet::Event<T>
     **/
    PalletOffchainTaskSchedulerEvent: 'Null',
    /**
     * Lookup80: frame_system::Phase
     **/
    FrameSystemPhase: {
        _enum: {
            ApplyExtrinsic: 'u32',
            Finalization: 'Null',
            Initialization: 'Null',
        },
    },
    /**
     * Lookup83: frame_system::LastRuntimeUpgradeInfo
     **/
    FrameSystemLastRuntimeUpgradeInfo: {
        specVersion: 'Compact<u32>',
        specName: 'Text',
    },
    /**
     * Lookup86: frame_system::pallet::Call<T>
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
     * Lookup90: frame_system::limits::BlockWeights
     **/
    FrameSystemLimitsBlockWeights: {
        baseBlock: 'SpWeightsWeightV2Weight',
        maxBlock: 'SpWeightsWeightV2Weight',
        perClass: 'FrameSupportDispatchPerDispatchClassWeightsPerClass',
    },
    /**
     * Lookup91: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
     **/
    FrameSupportDispatchPerDispatchClassWeightsPerClass: {
        normal: 'FrameSystemLimitsWeightsPerClass',
        operational: 'FrameSystemLimitsWeightsPerClass',
        mandatory: 'FrameSystemLimitsWeightsPerClass',
    },
    /**
     * Lookup92: frame_system::limits::WeightsPerClass
     **/
    FrameSystemLimitsWeightsPerClass: {
        baseExtrinsic: 'SpWeightsWeightV2Weight',
        maxExtrinsic: 'Option<SpWeightsWeightV2Weight>',
        maxTotal: 'Option<SpWeightsWeightV2Weight>',
        reserved: 'Option<SpWeightsWeightV2Weight>',
    },
    /**
     * Lookup94: frame_system::limits::BlockLength
     **/
    FrameSystemLimitsBlockLength: {
        max: 'FrameSupportDispatchPerDispatchClassU32',
    },
    /**
     * Lookup95: frame_support::dispatch::PerDispatchClass<T>
     **/
    FrameSupportDispatchPerDispatchClassU32: {
        normal: 'u32',
        operational: 'u32',
        mandatory: 'u32',
    },
    /**
     * Lookup96: sp_weights::RuntimeDbWeight
     **/
    SpWeightsRuntimeDbWeight: {
        read: 'u64',
        write: 'u64',
    },
    /**
     * Lookup97: sp_version::RuntimeVersion
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
     * Lookup103: frame_system::pallet::Error<T>
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
     * Lookup104: pallet_timestamp::pallet::Call<T>
     **/
    PalletTimestampCall: {
        _enum: {
            set: {
                now: 'Compact<u64>',
            },
        },
    },
    /**
     * Lookup106: pallet_balances::BalanceLock<Balance>
     **/
    PalletBalancesBalanceLock: {
        id: '[u8;8]',
        amount: 'u128',
        reasons: 'PalletBalancesReasons',
    },
    /**
     * Lookup107: pallet_balances::Reasons
     **/
    PalletBalancesReasons: {
        _enum: ['Fee', 'Misc', 'All'],
    },
    /**
     * Lookup110: pallet_balances::ReserveData<ReserveIdentifier, Balance>
     **/
    PalletBalancesReserveData: {
        id: '[u8;8]',
        amount: 'u128',
    },
    /**
     * Lookup112: pallet_balances::pallet::Call<T, I>
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
     * Lookup116: pallet_balances::pallet::Error<T, I>
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
     * Lookup118: pallet_transaction_payment::Releases
     **/
    PalletTransactionPaymentReleases: {
        _enum: ['V1Ancient', 'V2'],
    },
    /**
     * Lookup119: pallet_sudo::pallet::Call<T>
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
     * Lookup121: pallet_creditcoin::pallet::Call<T>
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
            request_collect_coins: {
                evmAddress: 'Bytes',
                txId: 'Bytes',
            },
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
            set_collect_coins_contract: {
                contract: 'PalletCreditcoinOcwTasksCollectCoinsGCreContract',
            },
            remove_authority: {
                who: 'AccountId32',
            },
        },
    },
    /**
     * Lookup122: sp_core::ecdsa::Public
     **/
    SpCoreEcdsaPublic: '[u8;33]',
    /**
     * Lookup124: sp_core::ecdsa::Signature
     **/
    SpCoreEcdsaSignature: '[u8;65]',
    /**
     * Lookup126: sp_runtime::MultiSigner
     **/
    SpRuntimeMultiSigner: {
        _enum: {
            Ed25519: 'SpCoreEd25519Public',
            Sr25519: 'SpCoreSr25519Public',
            Ecdsa: 'SpCoreEcdsaPublic',
        },
    },
    /**
     * Lookup127: sp_core::ed25519::Public
     **/
    SpCoreEd25519Public: '[u8;32]',
    /**
     * Lookup128: sp_core::sr25519::Public
     **/
    SpCoreSr25519Public: '[u8;32]',
    /**
     * Lookup129: sp_runtime::MultiSignature
     **/
    SpRuntimeMultiSignature: {
        _enum: {
            Ed25519: 'SpCoreEd25519Signature',
            Sr25519: 'SpCoreSr25519Signature',
            Ecdsa: 'SpCoreEcdsaSignature',
        },
    },
    /**
     * Lookup130: sp_core::ed25519::Signature
     **/
    SpCoreEd25519Signature: '[u8;64]',
    /**
     * Lookup132: sp_core::sr25519::Signature
     **/
    SpCoreSr25519Signature: '[u8;64]',
    /**
     * Lookup133: pallet_creditcoin::types::TaskOutput<sp_core::crypto::AccountId32, Balance, BlockNum, primitive_types::H256, Moment>
     **/
    PalletCreditcoinTaskOutput: {
        _enum: {
            VerifyTransfer: '(H256,PalletCreditcoinTransfer)',
            CollectCoins: '(H256,PalletCreditcoinCollectCoinsCollectedCoins)',
        },
    },
    /**
     * Lookup134: pallet_creditcoin::types::TaskId<primitive_types::H256>
     **/
    PalletCreditcoinTaskId: {
        _enum: {
            VerifyTransfer: 'H256',
            CollectCoins: 'H256',
        },
    },
    /**
     * Lookup135: pallet_difficulty::pallet::Call<T>
     **/
    PalletDifficultyCall: {
        _enum: {
            set_target_block_time: {
                targetTime: 'u64',
            },
            set_adjustment_period: {
                period: 'i64',
            },
        },
    },
    /**
     * Lookup137: pallet_scheduler::pallet::Call<T>
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
     * Lookup139: pallet_sudo::pallet::Error<T>
     **/
    PalletSudoError: {
        _enum: ['RequireSudo'],
    },
    /**
     * Lookup141: pallet_creditcoin::pallet::Error<T>
     **/
    PalletCreditcoinError: {
        _enum: [
            'AddressAlreadyRegistered',
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
        ],
    },
    /**
     * Lookup143: pallet_difficulty::DifficultyAndTimestamp<Moment>
     **/
    PalletDifficultyDifficultyAndTimestamp: {
        difficulty: 'U256',
        timestamp: 'u64',
    },
    /**
     * Lookup145: pallet_difficulty::pallet::Error<T>
     **/
    PalletDifficultyError: {
        _enum: ['ZeroTargetTime', 'ZeroAdjustmentPeriod', 'NegativeAdjustmentPeriod'],
    },
    /**
     * Lookup148: pallet_scheduler::Scheduled<Name, frame_support::traits::preimages::Bounded<creditcoin_node_runtime::RuntimeCall>, BlockNumber, creditcoin_node_runtime::OriginCaller, sp_core::crypto::AccountId32>
     **/
    PalletSchedulerScheduled: {
        maybeId: 'Option<[u8;32]>',
        priority: 'u8',
        call: 'FrameSupportPreimagesBounded',
        maybePeriodic: 'Option<(u32,u32)>',
        origin: 'CreditcoinNodeRuntimeOriginCaller',
    },
    /**
     * Lookup149: frame_support::traits::preimages::Bounded<creditcoin_node_runtime::RuntimeCall>
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
     * Lookup151: creditcoin_node_runtime::OriginCaller
     **/
    CreditcoinNodeRuntimeOriginCaller: {
        _enum: {
            system: 'FrameSupportDispatchRawOrigin',
            Void: 'SpCoreVoid',
        },
    },
    /**
     * Lookup152: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
     **/
    FrameSupportDispatchRawOrigin: {
        _enum: {
            Root: 'Null',
            Signed: 'AccountId32',
            None: 'Null',
        },
    },
    /**
     * Lookup153: sp_core::Void
     **/
    SpCoreVoid: 'Null',
    /**
     * Lookup155: pallet_scheduler::pallet::Error<T>
     **/
    PalletSchedulerError: {
        _enum: ['FailedToSchedule', 'NotFound', 'TargetBlockNumberInPast', 'RescheduleNoChange', 'Named'],
    },
    /**
     * Lookup156: pallet_creditcoin::types::Task<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
     **/
    PalletCreditcoinTask: {
        _enum: {
            VerifyTransfer: 'PalletCreditcoinTransferUnverifiedTransfer',
            CollectCoins: 'PalletCreditcoinCollectCoinsUnverifiedCollectedCoins',
        },
    },
    /**
     * Lookup157: pallet_creditcoin::types::transfer::UnverifiedTransfer<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
     **/
    PalletCreditcoinTransferUnverifiedTransfer: {
        transfer: 'PalletCreditcoinTransfer',
        fromExternal: 'Bytes',
        toExternal: 'Bytes',
        deadline: 'u32',
    },
    /**
     * Lookup158: pallet_offchain_task_scheduler::pallet::Error<T>
     **/
    PalletOffchainTaskSchedulerError: {
        _enum: ['OffchainSignedTxFailed', 'NoLocalAcctForSignedTx'],
    },
    /**
     * Lookup161: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
     **/
    FrameSystemExtensionsCheckNonZeroSender: 'Null',
    /**
     * Lookup162: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
     **/
    FrameSystemExtensionsCheckSpecVersion: 'Null',
    /**
     * Lookup163: frame_system::extensions::check_tx_version::CheckTxVersion<T>
     **/
    FrameSystemExtensionsCheckTxVersion: 'Null',
    /**
     * Lookup164: frame_system::extensions::check_genesis::CheckGenesis<T>
     **/
    FrameSystemExtensionsCheckGenesis: 'Null',
    /**
     * Lookup167: frame_system::extensions::check_nonce::CheckNonce<T>
     **/
    FrameSystemExtensionsCheckNonce: 'Compact<u32>',
    /**
     * Lookup168: frame_system::extensions::check_weight::CheckWeight<T>
     **/
    FrameSystemExtensionsCheckWeight: 'Null',
    /**
     * Lookup169: pallet_transaction_payment::ChargeTransactionPayment<T>
     **/
    PalletTransactionPaymentChargeTransactionPayment: 'Compact<u128>',
    /**
     * Lookup170: creditcoin_node_runtime::Runtime
     **/
    CreditcoinNodeRuntimeRuntime: 'Null',
};
