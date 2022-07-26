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
    data: 'PalletBalancesAccountData'
  },
  /**
   * Lookup5: pallet_balances::AccountData<Balance>
   **/
  PalletBalancesAccountData: {
    free: 'u128',
    reserved: 'u128',
    miscFrozen: 'u128',
    feeFrozen: 'u128'
  },
  /**
   * Lookup7: frame_support::weights::PerDispatchClass<T>
   **/
  FrameSupportWeightsPerDispatchClassU64: {
    normal: 'u64',
    operational: 'u64',
    mandatory: 'u64'
  },
  /**
   * Lookup11: sp_runtime::generic::digest::Digest
   **/
  SpRuntimeDigest: {
    logs: 'Vec<SpRuntimeDigestDigestItem>'
  },
  /**
   * Lookup13: sp_runtime::generic::digest::DigestItem
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
      RuntimeEnvironmentUpdated: 'Null'
    }
  },
  /**
   * Lookup16: frame_system::EventRecord<creditcoin_node_runtime::Event, primitive_types::H256>
   **/
  FrameSystemEventRecord: {
    phase: 'FrameSystemPhase',
    event: 'Event',
    topics: 'Vec<H256>'
  },
  /**
   * Lookup18: frame_system::pallet::Event<T>
   **/
  FrameSystemEvent: {
    _enum: {
      ExtrinsicSuccess: {
        dispatchInfo: 'FrameSupportWeightsDispatchInfo',
      },
      ExtrinsicFailed: {
        dispatchError: 'SpRuntimeDispatchError',
        dispatchInfo: 'FrameSupportWeightsDispatchInfo',
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
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup19: frame_support::weights::DispatchInfo
   **/
  FrameSupportWeightsDispatchInfo: {
    weight: 'u64',
    class: 'FrameSupportWeightsDispatchClass',
    paysFee: 'FrameSupportWeightsPays'
  },
  /**
   * Lookup20: frame_support::weights::DispatchClass
   **/
  FrameSupportWeightsDispatchClass: {
    _enum: ['Normal', 'Operational', 'Mandatory']
  },
  /**
   * Lookup21: frame_support::weights::Pays
   **/
  FrameSupportWeightsPays: {
    _enum: ['Yes', 'No']
  },
  /**
   * Lookup22: sp_runtime::DispatchError
   **/
  SpRuntimeDispatchError: {
    _enum: {
      Other: 'Null',
      CannotLookup: 'Null',
      BadOrigin: 'Null',
      Module: {
        index: 'u8',
        error: 'u8',
      },
      ConsumerRemaining: 'Null',
      NoProviders: 'Null',
      TooManyConsumers: 'Null',
      Token: 'SpRuntimeTokenError',
      Arithmetic: 'SpRuntimeArithmeticError'
    }
  },
  /**
   * Lookup23: sp_runtime::TokenError
   **/
  SpRuntimeTokenError: {
    _enum: ['NoFunds', 'WouldDie', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported']
  },
  /**
   * Lookup24: sp_runtime::ArithmeticError
   **/
  SpRuntimeArithmeticError: {
    _enum: ['Underflow', 'Overflow', 'DivisionByZero']
  },
  /**
   * Lookup25: pallet_balances::pallet::Event<T, I>
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
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup26: frame_support::traits::tokens::misc::BalanceStatus
   **/
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ['Free', 'Reserved']
  },
  /**
   * Lookup27: pallet_sudo::pallet::Event<T>
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
        sudoResult: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup31: pallet_creditcoin::pallet::Event<T>
   **/
  PalletCreditcoinEvent: {
    _enum: {
      AddressRegistered: '(H256,PalletCreditcoinAddress)',
      CollectCoinsRegistered: '(H256,PalletCreditcoinUnverifiedCollectedCoins)',
      TransferRegistered: '(H256,PalletCreditcoinTransfer)',
      TransferVerified: 'H256',
      CollectedCoinsMinted: '(H256,PalletCreditcoinCollectedCoins)',
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
      CurrencyRegistered: '(H256,PalletCreditcoinPlatformCurrency)'
    }
  },
  /**
   * Lookup33: pallet_creditcoin::types::Address<sp_core::crypto::AccountId32>
   **/
  PalletCreditcoinAddress: {
    blockchain: 'PalletCreditcoinPlatformBlockchain',
    value: 'Bytes',
    owner: 'AccountId32'
  },
  /**
   * Lookup34: pallet_creditcoin::types::platform::Blockchain
   **/
  PalletCreditcoinPlatformBlockchain: {
    _enum: {
      Evm: 'PalletCreditcoinPlatformEvmInfo'
    }
  },
  /**
   * Lookup35: pallet_creditcoin::types::platform::EvmInfo
   **/
  PalletCreditcoinPlatformEvmInfo: {
    chainId: 'PalletCreditcoinPlatformEvmChainId'
  },
  /**
   * Lookup36: pallet_creditcoin::types::platform::EvmChainId
   **/
  PalletCreditcoinPlatformEvmChainId: 'Compact<u64>',
  /**
   * Lookup40: pallet_creditcoin::types::UnverifiedCollectedCoins
   **/
  PalletCreditcoinUnverifiedCollectedCoins: {
    to: 'Bytes',
    txId: 'Bytes'
  },
  /**
   * Lookup42: pallet_creditcoin::types::Transfer<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
   **/
  PalletCreditcoinTransfer: {
    blockchain: 'PalletCreditcoinPlatformBlockchain',
    kind: 'PalletCreditcoinPlatformTransferKind',
    from: 'H256',
    to: 'H256',
    dealOrderId: 'PalletCreditcoinDealOrderId',
    amount: 'U256',
    txId: 'Bytes',
    block: 'u32',
    isProcessed: 'bool',
    accountId: 'AccountId32',
    timestamp: 'Option<u64>'
  },
  /**
   * Lookup43: pallet_creditcoin::types::platform::TransferKind
   **/
  PalletCreditcoinPlatformTransferKind: {
    _enum: {
      Evm: 'PalletCreditcoinPlatformEvmTransferKind'
    }
  },
  /**
   * Lookup44: pallet_creditcoin::types::platform::EvmTransferKind
   **/
  PalletCreditcoinPlatformEvmTransferKind: {
    _enum: ['Erc20', 'Ethless']
  },
  /**
   * Lookup45: pallet_creditcoin::types::DealOrderId<BlockNum, primitive_types::H256>
   **/
  PalletCreditcoinDealOrderId: '(u32,H256)',
  /**
   * Lookup50: pallet_creditcoin::types::CollectedCoins<primitive_types::H256, Balance>
   **/
  PalletCreditcoinCollectedCoins: {
    to: 'H256',
    amount: 'u128',
    txId: 'Bytes'
  },
  /**
   * Lookup51: pallet_creditcoin::types::AskOrderId<BlockNum, primitive_types::H256>
   **/
  PalletCreditcoinAskOrderId: '(u32,H256)',
  /**
   * Lookup52: pallet_creditcoin::types::AskOrder<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256>
   **/
  PalletCreditcoinAskOrder: {
    lenderAddressId: 'H256',
    terms: 'PalletCreditcoinLoanTermsAskTerms',
    expirationBlock: 'u32',
    block: 'u32',
    lender: 'AccountId32'
  },
  /**
   * Lookup53: pallet_creditcoin::types::loan_terms::AskTerms<primitive_types::H256>
   **/
  PalletCreditcoinLoanTermsAskTerms: 'PalletCreditcoinLoanTerms',
  /**
   * Lookup54: pallet_creditcoin::types::loan_terms::LoanTerms<primitive_types::H256>
   **/
  PalletCreditcoinLoanTerms: {
    amount: 'U256',
    interestRate: 'PalletCreditcoinLoanTermsInterestRate',
    termLength: 'PalletCreditcoinLoanTermsDuration',
    currency: 'H256'
  },
  /**
   * Lookup55: pallet_creditcoin::types::loan_terms::InterestRate
   **/
  PalletCreditcoinLoanTermsInterestRate: {
    ratePerPeriod: 'u64',
    decimals: 'u64',
    period: 'PalletCreditcoinLoanTermsDuration',
    interestType: 'PalletCreditcoinLoanTermsInterestType'
  },
  /**
   * Lookup56: pallet_creditcoin::types::loan_terms::Duration
   **/
  PalletCreditcoinLoanTermsDuration: {
    secs: 'u64',
    nanos: 'u32'
  },
  /**
   * Lookup57: pallet_creditcoin::types::loan_terms::InterestType
   **/
  PalletCreditcoinLoanTermsInterestType: {
    _enum: ['Simple', 'Compound']
  },
  /**
   * Lookup59: pallet_creditcoin::types::BidOrderId<BlockNum, primitive_types::H256>
   **/
  PalletCreditcoinBidOrderId: '(u32,H256)',
  /**
   * Lookup60: pallet_creditcoin::types::BidOrder<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256>
   **/
  PalletCreditcoinBidOrder: {
    borrowerAddressId: 'H256',
    terms: 'PalletCreditcoinLoanTermsBidTerms',
    expirationBlock: 'u32',
    block: 'u32',
    borrower: 'AccountId32'
  },
  /**
   * Lookup61: pallet_creditcoin::types::loan_terms::BidTerms<primitive_types::H256>
   **/
  PalletCreditcoinLoanTermsBidTerms: 'PalletCreditcoinLoanTerms',
  /**
   * Lookup62: pallet_creditcoin::types::OfferId<BlockNum, primitive_types::H256>
   **/
  PalletCreditcoinOfferId: '(u32,H256)',
  /**
   * Lookup63: pallet_creditcoin::types::Offer<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256>
   **/
  PalletCreditcoinOffer: {
    askId: 'PalletCreditcoinAskOrderId',
    bidId: 'PalletCreditcoinBidOrderId',
    expirationBlock: 'u32',
    block: 'u32',
    lender: 'AccountId32'
  },
  /**
   * Lookup64: pallet_creditcoin::types::DealOrder<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
   **/
  PalletCreditcoinDealOrder: {
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
    borrower: 'AccountId32'
  },
  /**
   * Lookup67: pallet_creditcoin::types::LegacySighash
   **/
  PalletCreditcoinLegacySighash: '[u8;60]',
  /**
   * Lookup69: pallet_creditcoin::ocw::errors::VerificationFailureCause
   **/
  PalletCreditcoinOcwErrorsVerificationFailureCause: {
    _enum: ['TaskNonexistent', 'TaskFailed', 'TaskPending', 'TaskUnconfirmed', 'TaskInFuture', 'IncorrectContract', 'MissingReceiver', 'MissingSender', 'AbiMismatch', 'IncorrectInputLength', 'EmptyInput', 'IncorrectInputType', 'IncorrectAmount', 'IncorrectNonce', 'IncorrectReceiver', 'IncorrectSender', 'InvalidAddress', 'UnsupportedMethod']
  },
  /**
   * Lookup70: pallet_creditcoin::types::platform::Currency
   **/
  PalletCreditcoinPlatformCurrency: {
    _enum: {
      Evm: '(PalletCreditcoinPlatformEvmCurrencyType,PalletCreditcoinPlatformEvmInfo)'
    }
  },
  /**
   * Lookup71: pallet_creditcoin::types::platform::EvmCurrencyType
   **/
  PalletCreditcoinPlatformEvmCurrencyType: {
    _enum: {
      SmartContract: '(Bytes,Vec<PalletCreditcoinPlatformEvmTransferKind>)'
    }
  },
  /**
   * Lookup74: pallet_rewards::pallet::Event<T>
   **/
  PalletRewardsEvent: {
    _enum: {
      RewardIssued: '(AccountId32,u128)'
    }
  },
  /**
   * Lookup75: frame_system::Phase
   **/
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: 'u32',
      Finalization: 'Null',
      Initialization: 'Null'
    }
  },
  /**
   * Lookup79: frame_system::LastRuntimeUpgradeInfo
   **/
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: 'Compact<u32>',
    specName: 'Text'
  },
  /**
   * Lookup82: frame_system::pallet::Call<T>
   **/
  FrameSystemCall: {
    _enum: {
      fill_block: {
        ratio: 'Perbill',
      },
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
        remark: 'Bytes'
      }
    }
  },
  /**
   * Lookup87: frame_system::limits::BlockWeights
   **/
  FrameSystemLimitsBlockWeights: {
    baseBlock: 'u64',
    maxBlock: 'u64',
    perClass: 'FrameSupportWeightsPerDispatchClassWeightsPerClass'
  },
  /**
   * Lookup88: frame_support::weights::PerDispatchClass<frame_system::limits::WeightsPerClass>
   **/
  FrameSupportWeightsPerDispatchClassWeightsPerClass: {
    normal: 'FrameSystemLimitsWeightsPerClass',
    operational: 'FrameSystemLimitsWeightsPerClass',
    mandatory: 'FrameSystemLimitsWeightsPerClass'
  },
  /**
   * Lookup89: frame_system::limits::WeightsPerClass
   **/
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: 'u64',
    maxExtrinsic: 'Option<u64>',
    maxTotal: 'Option<u64>',
    reserved: 'Option<u64>'
  },
  /**
   * Lookup90: frame_system::limits::BlockLength
   **/
  FrameSystemLimitsBlockLength: {
    max: 'FrameSupportWeightsPerDispatchClassU32'
  },
  /**
   * Lookup91: frame_support::weights::PerDispatchClass<T>
   **/
  FrameSupportWeightsPerDispatchClassU32: {
    normal: 'u32',
    operational: 'u32',
    mandatory: 'u32'
  },
  /**
   * Lookup92: frame_support::weights::RuntimeDbWeight
   **/
  FrameSupportWeightsRuntimeDbWeight: {
    read: 'u64',
    write: 'u64'
  },
  /**
   * Lookup93: sp_version::RuntimeVersion
   **/
  SpVersionRuntimeVersion: {
    specName: 'Text',
    implName: 'Text',
    authoringVersion: 'u32',
    specVersion: 'u32',
    implVersion: 'u32',
    apis: 'Vec<([u8;8],u32)>',
    transactionVersion: 'u32',
    stateVersion: 'u8'
  },
  /**
   * Lookup99: frame_system::pallet::Error<T>
   **/
  FrameSystemError: {
    _enum: ['InvalidSpecName', 'SpecVersionNeedsToIncrease', 'FailedToExtractRuntimeVersion', 'NonDefaultComposite', 'NonZeroRefCount', 'CallFiltered']
  },
  /**
   * Lookup100: pallet_timestamp::pallet::Call<T>
   **/
  PalletTimestampCall: {
    _enum: {
      set: {
        now: 'Compact<u64>'
      }
    }
  },
  /**
   * Lookup102: pallet_balances::BalanceLock<Balance>
   **/
  PalletBalancesBalanceLock: {
    id: '[u8;8]',
    amount: 'u128',
    reasons: 'PalletBalancesReasons'
  },
  /**
   * Lookup103: pallet_balances::Reasons
   **/
  PalletBalancesReasons: {
    _enum: ['Fee', 'Misc', 'All']
  },
  /**
   * Lookup106: pallet_balances::ReserveData<ReserveIdentifier, Balance>
   **/
  PalletBalancesReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup108: pallet_balances::Releases
   **/
  PalletBalancesReleases: {
    _enum: ['V1_0_0', 'V2_0_0']
  },
  /**
   * Lookup109: pallet_balances::pallet::Call<T, I>
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
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup114: pallet_balances::pallet::Error<T, I>
   **/
  PalletBalancesError: {
    _enum: ['VestingBalance', 'LiquidityRestrictions', 'InsufficientBalance', 'ExistentialDeposit', 'KeepAlive', 'ExistingVestingSchedule', 'DeadAccount', 'TooManyReserves']
  },
  /**
   * Lookup116: pallet_transaction_payment::Releases
   **/
  PalletTransactionPaymentReleases: {
    _enum: ['V1Ancient', 'V2']
  },
  /**
   * Lookup118: frame_support::weights::WeightToFeeCoefficient<Balance>
   **/
  FrameSupportWeightsWeightToFeeCoefficient: {
    coeffInteger: 'u128',
    coeffFrac: 'Perbill',
    negative: 'bool',
    degree: 'u8'
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
        weight: 'u64',
      },
      set_key: {
        _alias: {
          new_: 'new',
        },
        new_: 'MultiAddress',
      },
      sudo_as: {
        who: 'MultiAddress',
        call: 'Call'
      }
    }
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
        blockchain: 'PalletCreditcoinPlatformBlockchain',
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
      register_funding_transfer_legacy: {
        transferKind: 'PalletCreditcoinLegacyTransferKind',
        dealOrderId: 'PalletCreditcoinDealOrderId',
        blockchainTxId: 'Bytes',
      },
      register_repayment_transfer_legacy: {
        transferKind: 'PalletCreditcoinLegacyTransferKind',
        repaymentAmount: 'U256',
        dealOrderId: 'PalletCreditcoinDealOrderId',
        blockchainTxId: 'Bytes',
      },
      register_funding_transfer: {
        transferKind: 'PalletCreditcoinPlatformTransferKind',
        dealOrderId: 'PalletCreditcoinDealOrderId',
        blockchainTxId: 'Bytes',
      },
      register_repayment_transfer: {
        transferKind: 'PalletCreditcoinPlatformTransferKind',
        repaymentAmount: 'U256',
        dealOrderId: 'PalletCreditcoinDealOrderId',
        blockchainTxId: 'Bytes',
      },
      exempt: {
        dealOrderId: 'PalletCreditcoinDealOrderId',
      },
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
      register_currency: {
        currency: 'PalletCreditcoinPlatformCurrency'
      }
    }
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
      Ecdsa: 'SpCoreEcdsaPublic'
    }
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
      Ecdsa: 'SpCoreEcdsaSignature'
    }
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
   * Lookup133: pallet_creditcoin::types::LegacyTransferKind
   **/
  PalletCreditcoinLegacyTransferKind: {
    _enum: {
      Erc20: 'Bytes',
      Ethless: 'Bytes',
      Native: 'Null',
      Other: 'Bytes'
    }
  },
  /**
   * Lookup134: pallet_creditcoin::types::TaskOutput<sp_core::crypto::AccountId32, Balance, BlockNum, primitive_types::H256, Moment>
   **/
  PalletCreditcoinTaskOutput: {
    _enum: {
      VerifyTransfer: '(H256,PalletCreditcoinTransfer)',
      CollectCoins: '(H256,PalletCreditcoinCollectedCoins)'
    }
  },
  /**
   * Lookup135: pallet_creditcoin::types::TaskId<primitive_types::H256>
   **/
  PalletCreditcoinTaskId: {
    _enum: {
      VerifyTransfer: 'H256',
      CollectCoins: 'H256'
    }
  },
  /**
   * Lookup136: pallet_difficulty::pallet::Call<T>
   **/
  PalletDifficultyCall: {
    _enum: {
      set_target_block_time: {
        targetTime: 'u64',
      },
      set_adjustment_period: {
        period: 'i64'
      }
    }
  },
  /**
   * Lookup138: pallet_sudo::pallet::Error<T>
   **/
  PalletSudoError: {
    _enum: ['RequireSudo']
  },
  /**
   * Lookup140: pallet_creditcoin::types::Task<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
   **/
  PalletCreditcoinTask: {
    _enum: {
      VerifyTransfer: 'PalletCreditcoinUnverifiedTransfer',
      CollectCoins: 'PalletCreditcoinUnverifiedCollectedCoins'
    }
  },
  /**
   * Lookup141: pallet_creditcoin::types::UnverifiedTransfer<sp_core::crypto::AccountId32, BlockNum, primitive_types::H256, Moment>
   **/
  PalletCreditcoinUnverifiedTransfer: {
    transfer: 'PalletCreditcoinTransfer',
    fromExternal: 'Bytes',
    toExternal: 'Bytes',
    deadline: 'u32',
    currencyToCheck: 'PalletCreditcoinCurrencyOrLegacyTransferKind'
  },
  /**
   * Lookup142: pallet_creditcoin::types::CurrencyOrLegacyTransferKind
   **/
  PalletCreditcoinCurrencyOrLegacyTransferKind: {
    _enum: {
      Currency: 'PalletCreditcoinPlatformCurrency',
      TransferKind: 'PalletCreditcoinLegacyTransferKind'
    }
  },
  /**
   * Lookup144: pallet_creditcoin::pallet::Error<T>
   **/
  PalletCreditcoinError: {
    _enum: ['AddressAlreadyRegistered', 'NonExistentAddress', 'NonExistentDealOrder', 'NonExistentAskOrder', 'NonExistentBidOrder', 'NonExistentOffer', 'NonExistentTransfer', 'TransferAlreadyRegistered', 'CollectCoinsAlreadyRegistered', 'BalanceOverflow', 'TransferAccountMismatch', 'TransferDealOrderMismatch', 'TransferAmountMismatch', 'TransferAlreadyProcessed', 'TransferAmountInsufficient', 'MalformedTransfer', 'UnsupportedTransferKind', 'InsufficientAuthority', 'DuplicateId', 'NotAddressOwner', 'OffchainSignedTxFailed', 'NoLocalAcctForSignedTx', 'RepaymentOrderNonZeroGain', 'AddressPlatformMismatch', 'AlreadyAuthority', 'DuplicateOffer', 'DealNotFunded', 'DealOrderAlreadyFunded', 'DealOrderAlreadyClosed', 'DealOrderAlreadyLocked', 'DealOrderMustBeLocked', 'DuplicateDealOrder', 'DealOrderExpired', 'AskOrderExpired', 'BidOrderExpired', 'OfferExpired', 'AskBidMismatch', 'SameOwner', 'InvalidSignature', 'NotBorrower', 'MalformedDealOrder', 'NotLender', 'RepaymentOrderUnsupported', 'NotLegacyWalletOwner', 'LegacyWalletNotFound', 'LegacyBalanceKeeperMissing', 'GuidAlreadyUsed', 'InvalidTermLength', 'MalformedExternalAddress', 'AddressFormatNotSupported', 'OwnershipNotSatisfied', 'CurrencyAlreadyRegistered', 'DeprecatedExtrinsic', 'CurrencyNotRegistered']
  },
  /**
   * Lookup146: pallet_difficulty::DifficultyAndTimestamp<Moment>
   **/
  PalletDifficultyDifficultyAndTimestamp: {
    difficulty: 'U256',
    timestamp: 'u64'
  },
  /**
   * Lookup148: pallet_difficulty::pallet::Error<T>
   **/
  PalletDifficultyError: {
    _enum: ['ZeroTargetTime', 'ZeroAdjustmentPeriod', 'NegativeAdjustmentPeriod']
  },
  /**
   * Lookup151: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
   **/
  FrameSystemExtensionsCheckSpecVersion: 'Null',
  /**
   * Lookup152: frame_system::extensions::check_tx_version::CheckTxVersion<T>
   **/
  FrameSystemExtensionsCheckTxVersion: 'Null',
  /**
   * Lookup153: frame_system::extensions::check_genesis::CheckGenesis<T>
   **/
  FrameSystemExtensionsCheckGenesis: 'Null',
  /**
   * Lookup156: frame_system::extensions::check_nonce::CheckNonce<T>
   **/
  FrameSystemExtensionsCheckNonce: 'Compact<u32>',
  /**
   * Lookup157: frame_system::extensions::check_weight::CheckWeight<T>
   **/
  FrameSystemExtensionsCheckWeight: 'Null',
  /**
   * Lookup158: pallet_transaction_payment::ChargeTransactionPayment<T>
   **/
  PalletTransactionPaymentChargeTransactionPayment: 'Compact<u128>',
  /**
   * Lookup159: creditcoin_node_runtime::Runtime
   **/
  CreditcoinNodeRuntimeRuntime: 'Null'
};
