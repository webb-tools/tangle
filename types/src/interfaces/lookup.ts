// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

/* eslint-disable sort-keys */

export default {
  /**
   * Lookup3: frame_system::AccountInfo<Nonce, pallet_balances::types::AccountData<Balance>>
   **/
  FrameSystemAccountInfo: {
    nonce: 'u32',
    consumers: 'u32',
    providers: 'u32',
    sufficients: 'u32',
    data: 'PalletBalancesAccountData'
  },
  /**
   * Lookup5: pallet_balances::types::AccountData<Balance>
   **/
  PalletBalancesAccountData: {
    free: 'u128',
    reserved: 'u128',
    frozen: 'u128',
    flags: 'u128'
  },
  /**
   * Lookup9: frame_support::dispatch::PerDispatchClass<sp_weights::weight_v2::Weight>
   **/
  FrameSupportDispatchPerDispatchClassWeight: {
    normal: 'SpWeightsWeightV2Weight',
    operational: 'SpWeightsWeightV2Weight',
    mandatory: 'SpWeightsWeightV2Weight'
  },
  /**
   * Lookup10: sp_weights::weight_v2::Weight
   **/
  SpWeightsWeightV2Weight: {
    refTime: 'Compact<u64>',
    proofSize: 'Compact<u64>'
  },
  /**
   * Lookup15: sp_runtime::generic::digest::Digest
   **/
  SpRuntimeDigest: {
    logs: 'Vec<SpRuntimeDigestDigestItem>'
  },
  /**
   * Lookup17: sp_runtime::generic::digest::DigestItem
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
   * Lookup20: frame_system::EventRecord<tangle_testnet_runtime::RuntimeEvent, primitive_types::H256>
   **/
  FrameSystemEventRecord: {
    phase: 'FrameSystemPhase',
    event: 'Event',
    topics: 'Vec<H256>'
  },
  /**
   * Lookup22: frame_system::pallet::Event<T>
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
      UpgradeAuthorized: {
        codeHash: 'H256',
        checkVersion: 'bool'
      }
    }
  },
  /**
   * Lookup23: frame_support::dispatch::DispatchInfo
   **/
  FrameSupportDispatchDispatchInfo: {
    weight: 'SpWeightsWeightV2Weight',
    class: 'FrameSupportDispatchDispatchClass',
    paysFee: 'FrameSupportDispatchPays'
  },
  /**
   * Lookup24: frame_support::dispatch::DispatchClass
   **/
  FrameSupportDispatchDispatchClass: {
    _enum: ['Normal', 'Operational', 'Mandatory']
  },
  /**
   * Lookup25: frame_support::dispatch::Pays
   **/
  FrameSupportDispatchPays: {
    _enum: ['Yes', 'No']
  },
  /**
   * Lookup26: sp_runtime::DispatchError
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
      RootNotAllowed: 'Null'
    }
  },
  /**
   * Lookup27: sp_runtime::ModuleError
   **/
  SpRuntimeModuleError: {
    index: 'u8',
    error: '[u8;4]'
  },
  /**
   * Lookup28: sp_runtime::TokenError
   **/
  SpRuntimeTokenError: {
    _enum: ['FundsUnavailable', 'OnlyProvider', 'BelowMinimum', 'CannotCreate', 'UnknownAsset', 'Frozen', 'Unsupported', 'CannotCreateHold', 'NotExpendable', 'Blocked']
  },
  /**
   * Lookup29: sp_arithmetic::ArithmeticError
   **/
  SpArithmeticArithmeticError: {
    _enum: ['Underflow', 'Overflow', 'DivisionByZero']
  },
  /**
   * Lookup30: sp_runtime::TransactionalError
   **/
  SpRuntimeTransactionalError: {
    _enum: ['LimitReached', 'NoLayer']
  },
  /**
   * Lookup31: pallet_sudo::pallet::Event<T>
   **/
  PalletSudoEvent: {
    _enum: {
      Sudid: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>',
      },
      KeyChanged: {
        _alias: {
          new_: 'new',
        },
        old: 'Option<AccountId32>',
        new_: 'AccountId32',
      },
      KeyRemoved: 'Null',
      SudoAsDone: {
        sudoResult: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup35: pallet_assets::pallet::Event<T, I>
   **/
  PalletAssetsEvent: {
    _enum: {
      Created: {
        assetId: 'u128',
        creator: 'AccountId32',
        owner: 'AccountId32',
      },
      Issued: {
        assetId: 'u128',
        owner: 'AccountId32',
        amount: 'u128',
      },
      Transferred: {
        assetId: 'u128',
        from: 'AccountId32',
        to: 'AccountId32',
        amount: 'u128',
      },
      Burned: {
        assetId: 'u128',
        owner: 'AccountId32',
        balance: 'u128',
      },
      TeamChanged: {
        assetId: 'u128',
        issuer: 'AccountId32',
        admin: 'AccountId32',
        freezer: 'AccountId32',
      },
      OwnerChanged: {
        assetId: 'u128',
        owner: 'AccountId32',
      },
      Frozen: {
        assetId: 'u128',
        who: 'AccountId32',
      },
      Thawed: {
        assetId: 'u128',
        who: 'AccountId32',
      },
      AssetFrozen: {
        assetId: 'u128',
      },
      AssetThawed: {
        assetId: 'u128',
      },
      AccountsDestroyed: {
        assetId: 'u128',
        accountsDestroyed: 'u32',
        accountsRemaining: 'u32',
      },
      ApprovalsDestroyed: {
        assetId: 'u128',
        approvalsDestroyed: 'u32',
        approvalsRemaining: 'u32',
      },
      DestructionStarted: {
        assetId: 'u128',
      },
      Destroyed: {
        assetId: 'u128',
      },
      ForceCreated: {
        assetId: 'u128',
        owner: 'AccountId32',
      },
      MetadataSet: {
        assetId: 'u128',
        name: 'Bytes',
        symbol: 'Bytes',
        decimals: 'u8',
        isFrozen: 'bool',
      },
      MetadataCleared: {
        assetId: 'u128',
      },
      ApprovedTransfer: {
        assetId: 'u128',
        source: 'AccountId32',
        delegate: 'AccountId32',
        amount: 'u128',
      },
      ApprovalCancelled: {
        assetId: 'u128',
        owner: 'AccountId32',
        delegate: 'AccountId32',
      },
      TransferredApproved: {
        assetId: 'u128',
        owner: 'AccountId32',
        delegate: 'AccountId32',
        destination: 'AccountId32',
        amount: 'u128',
      },
      AssetStatusChanged: {
        assetId: 'u128',
      },
      AssetMinBalanceChanged: {
        assetId: 'u128',
        newMinBalance: 'u128',
      },
      Touched: {
        assetId: 'u128',
        who: 'AccountId32',
        depositor: 'AccountId32',
      },
      Blocked: {
        assetId: 'u128',
        who: 'AccountId32',
      },
      Deposited: {
        assetId: 'u128',
        who: 'AccountId32',
        amount: 'u128',
      },
      Withdrawn: {
        assetId: 'u128',
        who: 'AccountId32',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup36: pallet_balances::pallet::Event<T, I>
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
      Minted: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Burned: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Suspended: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Restored: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Upgraded: {
        who: 'AccountId32',
      },
      Issued: {
        amount: 'u128',
      },
      Rescinded: {
        amount: 'u128',
      },
      Locked: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Unlocked: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Frozen: {
        who: 'AccountId32',
        amount: 'u128',
      },
      Thawed: {
        who: 'AccountId32',
        amount: 'u128',
      },
      TotalIssuanceForced: {
        _alias: {
          new_: 'new',
        },
        old: 'u128',
        new_: 'u128'
      }
    }
  },
  /**
   * Lookup37: frame_support::traits::tokens::misc::BalanceStatus
   **/
  FrameSupportTokensMiscBalanceStatus: {
    _enum: ['Free', 'Reserved']
  },
  /**
   * Lookup38: pallet_transaction_payment::pallet::Event<T>
   **/
  PalletTransactionPaymentEvent: {
    _enum: {
      TransactionFeePaid: {
        who: 'AccountId32',
        actualFee: 'u128',
        tip: 'u128'
      }
    }
  },
  /**
   * Lookup39: pallet_grandpa::pallet::Event
   **/
  PalletGrandpaEvent: {
    _enum: {
      NewAuthorities: {
        authoritySet: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
      },
      Paused: 'Null',
      Resumed: 'Null'
    }
  },
  /**
   * Lookup42: sp_consensus_grandpa::app::Public
   **/
  SpConsensusGrandpaAppPublic: '[u8;32]',
  /**
   * Lookup43: pallet_indices::pallet::Event<T>
   **/
  PalletIndicesEvent: {
    _enum: {
      IndexAssigned: {
        who: 'AccountId32',
        index: 'u32',
      },
      IndexFreed: {
        index: 'u32',
      },
      IndexFrozen: {
        index: 'u32',
        who: 'AccountId32'
      }
    }
  },
  /**
   * Lookup44: pallet_democracy::pallet::Event<T>
   **/
  PalletDemocracyEvent: {
    _enum: {
      Proposed: {
        proposalIndex: 'u32',
        deposit: 'u128',
      },
      Tabled: {
        proposalIndex: 'u32',
        deposit: 'u128',
      },
      ExternalTabled: 'Null',
      Started: {
        refIndex: 'u32',
        threshold: 'PalletDemocracyVoteThreshold',
      },
      Passed: {
        refIndex: 'u32',
      },
      NotPassed: {
        refIndex: 'u32',
      },
      Cancelled: {
        refIndex: 'u32',
      },
      Delegated: {
        who: 'AccountId32',
        target: 'AccountId32',
      },
      Undelegated: {
        account: 'AccountId32',
      },
      Vetoed: {
        who: 'AccountId32',
        proposalHash: 'H256',
        until: 'u64',
      },
      Blacklisted: {
        proposalHash: 'H256',
      },
      Voted: {
        voter: 'AccountId32',
        refIndex: 'u32',
        vote: 'PalletDemocracyVoteAccountVote',
      },
      Seconded: {
        seconder: 'AccountId32',
        propIndex: 'u32',
      },
      ProposalCanceled: {
        propIndex: 'u32',
      },
      MetadataSet: {
        _alias: {
          hash_: 'hash',
        },
        owner: 'PalletDemocracyMetadataOwner',
        hash_: 'H256',
      },
      MetadataCleared: {
        _alias: {
          hash_: 'hash',
        },
        owner: 'PalletDemocracyMetadataOwner',
        hash_: 'H256',
      },
      MetadataTransferred: {
        _alias: {
          hash_: 'hash',
        },
        prevOwner: 'PalletDemocracyMetadataOwner',
        owner: 'PalletDemocracyMetadataOwner',
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup45: pallet_democracy::vote_threshold::VoteThreshold
   **/
  PalletDemocracyVoteThreshold: {
    _enum: ['SuperMajorityApprove', 'SuperMajorityAgainst', 'SimpleMajority']
  },
  /**
   * Lookup46: pallet_democracy::vote::AccountVote<Balance>
   **/
  PalletDemocracyVoteAccountVote: {
    _enum: {
      Standard: {
        vote: 'Vote',
        balance: 'u128',
      },
      Split: {
        aye: 'u128',
        nay: 'u128'
      }
    }
  },
  /**
   * Lookup48: pallet_democracy::types::MetadataOwner
   **/
  PalletDemocracyMetadataOwner: {
    _enum: {
      External: 'Null',
      Proposal: 'u32',
      Referendum: 'u32'
    }
  },
  /**
   * Lookup49: pallet_collective::pallet::Event<T, I>
   **/
  PalletCollectiveEvent: {
    _enum: {
      Proposed: {
        account: 'AccountId32',
        proposalIndex: 'u32',
        proposalHash: 'H256',
        threshold: 'u32',
      },
      Voted: {
        account: 'AccountId32',
        proposalHash: 'H256',
        voted: 'bool',
        yes: 'u32',
        no: 'u32',
      },
      Approved: {
        proposalHash: 'H256',
      },
      Disapproved: {
        proposalHash: 'H256',
      },
      Executed: {
        proposalHash: 'H256',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      MemberExecuted: {
        proposalHash: 'H256',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      Closed: {
        proposalHash: 'H256',
        yes: 'u32',
        no: 'u32'
      }
    }
  },
  /**
   * Lookup50: pallet_vesting::pallet::Event<T>
   **/
  PalletVestingEvent: {
    _enum: {
      VestingUpdated: {
        account: 'AccountId32',
        unvested: 'u128',
      },
      VestingCompleted: {
        account: 'AccountId32'
      }
    }
  },
  /**
   * Lookup51: pallet_elections_phragmen::pallet::Event<T>
   **/
  PalletElectionsPhragmenEvent: {
    _enum: {
      NewTerm: {
        newMembers: 'Vec<(AccountId32,u128)>',
      },
      EmptyTerm: 'Null',
      ElectionError: 'Null',
      MemberKicked: {
        member: 'AccountId32',
      },
      Renounced: {
        candidate: 'AccountId32',
      },
      CandidateSlashed: {
        candidate: 'AccountId32',
        amount: 'u128',
      },
      SeatHolderSlashed: {
        seatHolder: 'AccountId32',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup54: pallet_election_provider_multi_phase::pallet::Event<T>
   **/
  PalletElectionProviderMultiPhaseEvent: {
    _enum: {
      SolutionStored: {
        compute: 'PalletElectionProviderMultiPhaseElectionCompute',
        origin: 'Option<AccountId32>',
        prevEjected: 'bool',
      },
      ElectionFinalized: {
        compute: 'PalletElectionProviderMultiPhaseElectionCompute',
        score: 'SpNposElectionsElectionScore',
      },
      ElectionFailed: 'Null',
      Rewarded: {
        account: 'AccountId32',
        value: 'u128',
      },
      Slashed: {
        account: 'AccountId32',
        value: 'u128',
      },
      PhaseTransitioned: {
        from: 'PalletElectionProviderMultiPhasePhase',
        to: 'PalletElectionProviderMultiPhasePhase',
        round: 'u32'
      }
    }
  },
  /**
   * Lookup55: pallet_election_provider_multi_phase::ElectionCompute
   **/
  PalletElectionProviderMultiPhaseElectionCompute: {
    _enum: ['OnChain', 'Signed', 'Unsigned', 'Fallback', 'Emergency']
  },
  /**
   * Lookup56: sp_npos_elections::ElectionScore
   **/
  SpNposElectionsElectionScore: {
    minimalStake: 'u128',
    sumStake: 'u128',
    sumStakeSquared: 'u128'
  },
  /**
   * Lookup57: pallet_election_provider_multi_phase::Phase<Bn>
   **/
  PalletElectionProviderMultiPhasePhase: {
    _enum: {
      Off: 'Null',
      Signed: 'Null',
      Unsigned: '(bool,u64)',
      Emergency: 'Null'
    }
  },
  /**
   * Lookup59: pallet_staking::pallet::pallet::Event<T>
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
        dest: 'PalletStakingRewardDestination',
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
      SnapshotVotersSizeExceeded: {
        _alias: {
          size_: 'size',
        },
        size_: 'u32',
      },
      SnapshotTargetsSizeExceeded: {
        _alias: {
          size_: 'size',
        },
        size_: 'u32',
      },
      ForceEra: {
        mode: 'PalletStakingForcing',
      },
      ControllerBatchDeprecated: {
        failures: 'u32'
      }
    }
  },
  /**
   * Lookup60: pallet_staking::RewardDestination<sp_core::crypto::AccountId32>
   **/
  PalletStakingRewardDestination: {
    _enum: {
      Staked: 'Null',
      Stash: 'Null',
      Controller: 'Null',
      Account: 'AccountId32',
      None: 'Null'
    }
  },
  /**
   * Lookup62: pallet_staking::ValidatorPrefs
   **/
  PalletStakingValidatorPrefs: {
    commission: 'Compact<Perbill>',
    blocked: 'bool'
  },
  /**
   * Lookup64: pallet_staking::Forcing
   **/
  PalletStakingForcing: {
    _enum: ['NotForcing', 'ForceNew', 'ForceNone', 'ForceAlways']
  },
  /**
   * Lookup65: pallet_session::pallet::Event
   **/
  PalletSessionEvent: {
    _enum: {
      NewSession: {
        sessionIndex: 'u32'
      }
    }
  },
  /**
   * Lookup66: pallet_treasury::pallet::Event<T, I>
   **/
  PalletTreasuryEvent: {
    _enum: {
      Spending: {
        budgetRemaining: 'u128',
      },
      Awarded: {
        proposalIndex: 'u32',
        award: 'u128',
        account: 'AccountId32',
      },
      Burnt: {
        burntFunds: 'u128',
      },
      Rollover: {
        rolloverBalance: 'u128',
      },
      Deposit: {
        value: 'u128',
      },
      SpendApproved: {
        proposalIndex: 'u32',
        amount: 'u128',
        beneficiary: 'AccountId32',
      },
      UpdatedInactive: {
        reactivated: 'u128',
        deactivated: 'u128',
      },
      AssetSpendApproved: {
        index: 'u32',
        assetKind: 'Null',
        amount: 'u128',
        beneficiary: 'AccountId32',
        validFrom: 'u64',
        expireAt: 'u64',
      },
      AssetSpendVoided: {
        index: 'u32',
      },
      Paid: {
        index: 'u32',
        paymentId: 'Null',
      },
      PaymentFailed: {
        index: 'u32',
        paymentId: 'Null',
      },
      SpendProcessed: {
        index: 'u32'
      }
    }
  },
  /**
   * Lookup67: pallet_bounties::pallet::Event<T, I>
   **/
  PalletBountiesEvent: {
    _enum: {
      BountyProposed: {
        index: 'u32',
      },
      BountyRejected: {
        index: 'u32',
        bond: 'u128',
      },
      BountyBecameActive: {
        index: 'u32',
      },
      BountyAwarded: {
        index: 'u32',
        beneficiary: 'AccountId32',
      },
      BountyClaimed: {
        index: 'u32',
        payout: 'u128',
        beneficiary: 'AccountId32',
      },
      BountyCanceled: {
        index: 'u32',
      },
      BountyExtended: {
        index: 'u32',
      },
      BountyApproved: {
        index: 'u32',
      },
      CuratorProposed: {
        bountyId: 'u32',
        curator: 'AccountId32',
      },
      CuratorUnassigned: {
        bountyId: 'u32',
      },
      CuratorAccepted: {
        bountyId: 'u32',
        curator: 'AccountId32'
      }
    }
  },
  /**
   * Lookup68: pallet_child_bounties::pallet::Event<T>
   **/
  PalletChildBountiesEvent: {
    _enum: {
      Added: {
        index: 'u32',
        childIndex: 'u32',
      },
      Awarded: {
        index: 'u32',
        childIndex: 'u32',
        beneficiary: 'AccountId32',
      },
      Claimed: {
        index: 'u32',
        childIndex: 'u32',
        payout: 'u128',
        beneficiary: 'AccountId32',
      },
      Canceled: {
        index: 'u32',
        childIndex: 'u32'
      }
    }
  },
  /**
   * Lookup69: pallet_bags_list::pallet::Event<T, I>
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
        newScore: 'u64'
      }
    }
  },
  /**
   * Lookup70: pallet_nomination_pools::pallet::Event<T>
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
      PoolCommissionClaimPermissionUpdated: {
        poolId: 'u32',
        permission: 'Option<PalletNominationPoolsCommissionClaimPermission>',
      },
      PoolCommissionClaimed: {
        poolId: 'u32',
        commission: 'u128',
      },
      MinBalanceDeficitAdjusted: {
        poolId: 'u32',
        amount: 'u128',
      },
      MinBalanceExcessAdjusted: {
        poolId: 'u32',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup71: pallet_nomination_pools::PoolState
   **/
  PalletNominationPoolsPoolState: {
    _enum: ['Open', 'Blocked', 'Destroying']
  },
  /**
   * Lookup74: pallet_nomination_pools::CommissionChangeRate<BlockNumber>
   **/
  PalletNominationPoolsCommissionChangeRate: {
    maxIncrease: 'Perbill',
    minDelay: 'u64'
  },
  /**
   * Lookup76: pallet_nomination_pools::CommissionClaimPermission<sp_core::crypto::AccountId32>
   **/
  PalletNominationPoolsCommissionClaimPermission: {
    _enum: {
      Permissionless: 'Null',
      Account: 'AccountId32'
    }
  },
  /**
   * Lookup77: pallet_scheduler::pallet::Event<T>
   **/
  PalletSchedulerEvent: {
    _enum: {
      Scheduled: {
        when: 'u64',
        index: 'u32',
      },
      Canceled: {
        when: 'u64',
        index: 'u32',
      },
      Dispatched: {
        task: '(u64,u32)',
        id: 'Option<[u8;32]>',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      RetrySet: {
        task: '(u64,u32)',
        id: 'Option<[u8;32]>',
        period: 'u64',
        retries: 'u8',
      },
      RetryCancelled: {
        task: '(u64,u32)',
        id: 'Option<[u8;32]>',
      },
      CallUnavailable: {
        task: '(u64,u32)',
        id: 'Option<[u8;32]>',
      },
      PeriodicFailed: {
        task: '(u64,u32)',
        id: 'Option<[u8;32]>',
      },
      RetryFailed: {
        task: '(u64,u32)',
        id: 'Option<[u8;32]>',
      },
      PermanentlyOverweight: {
        task: '(u64,u32)',
        id: 'Option<[u8;32]>'
      }
    }
  },
  /**
   * Lookup80: pallet_preimage::pallet::Event<T>
   **/
  PalletPreimageEvent: {
    _enum: {
      Noted: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      Requested: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      Cleared: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256'
      }
    }
  },
  /**
   * Lookup81: pallet_offences::pallet::Event
   **/
  PalletOffencesEvent: {
    _enum: {
      Offence: {
        kind: '[u8;16]',
        timeslot: 'Bytes'
      }
    }
  },
  /**
   * Lookup83: pallet_tx_pause::pallet::Event<T>
   **/
  PalletTxPauseEvent: {
    _enum: {
      CallPaused: {
        fullName: '(Bytes,Bytes)',
      },
      CallUnpaused: {
        fullName: '(Bytes,Bytes)'
      }
    }
  },
  /**
   * Lookup86: pallet_im_online::pallet::Event<T>
   **/
  PalletImOnlineEvent: {
    _enum: {
      HeartbeatReceived: {
        authorityId: 'PalletImOnlineSr25519AppSr25519Public',
      },
      AllGood: 'Null',
      SomeOffline: {
        offline: 'Vec<(AccountId32,SpStakingExposure)>'
      }
    }
  },
  /**
   * Lookup87: pallet_im_online::sr25519::app_sr25519::Public
   **/
  PalletImOnlineSr25519AppSr25519Public: '[u8;32]',
  /**
   * Lookup90: sp_staking::Exposure<sp_core::crypto::AccountId32, Balance>
   **/
  SpStakingExposure: {
    total: 'Compact<u128>',
    own: 'Compact<u128>',
    others: 'Vec<SpStakingIndividualExposure>'
  },
  /**
   * Lookup93: sp_staking::IndividualExposure<sp_core::crypto::AccountId32, Balance>
   **/
  SpStakingIndividualExposure: {
    who: 'AccountId32',
    value: 'Compact<u128>'
  },
  /**
   * Lookup94: pallet_identity::pallet::Event<T>
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
      AuthorityAdded: {
        authority: 'AccountId32',
      },
      AuthorityRemoved: {
        authority: 'AccountId32',
      },
      UsernameSet: {
        who: 'AccountId32',
        username: 'Bytes',
      },
      UsernameQueued: {
        who: 'AccountId32',
        username: 'Bytes',
        expiration: 'u64',
      },
      PreapprovalExpired: {
        whose: 'AccountId32',
      },
      PrimaryUsernameSet: {
        who: 'AccountId32',
        username: 'Bytes',
      },
      DanglingUsernameRemoved: {
        who: 'AccountId32',
        username: 'Bytes'
      }
    }
  },
  /**
   * Lookup96: pallet_utility::pallet::Event
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
        result: 'Result<Null, SpRuntimeDispatchError>'
      }
    }
  },
  /**
   * Lookup97: pallet_multisig::pallet::Event<T>
   **/
  PalletMultisigEvent: {
    _enum: {
      NewMultisig: {
        approving: 'AccountId32',
        multisig: 'AccountId32',
        callHash: '[u8;32]',
      },
      MultisigApproval: {
        approving: 'AccountId32',
        timepoint: 'PalletMultisigTimepoint',
        multisig: 'AccountId32',
        callHash: '[u8;32]',
      },
      MultisigExecuted: {
        approving: 'AccountId32',
        timepoint: 'PalletMultisigTimepoint',
        multisig: 'AccountId32',
        callHash: '[u8;32]',
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      MultisigCancelled: {
        cancelling: 'AccountId32',
        timepoint: 'PalletMultisigTimepoint',
        multisig: 'AccountId32',
        callHash: '[u8;32]'
      }
    }
  },
  /**
   * Lookup98: pallet_multisig::Timepoint<BlockNumber>
   **/
  PalletMultisigTimepoint: {
    height: 'u64',
    index: 'u32'
  },
  /**
   * Lookup99: pallet_ethereum::pallet::Event
   **/
  PalletEthereumEvent: {
    _enum: {
      Executed: {
        from: 'H160',
        to: 'H160',
        transactionHash: 'H256',
        exitReason: 'EvmCoreErrorExitReason',
        extraData: 'Bytes'
      }
    }
  },
  /**
   * Lookup102: evm_core::error::ExitReason
   **/
  EvmCoreErrorExitReason: {
    _enum: {
      Succeed: 'EvmCoreErrorExitSucceed',
      Error: 'EvmCoreErrorExitError',
      Revert: 'EvmCoreErrorExitRevert',
      Fatal: 'EvmCoreErrorExitFatal'
    }
  },
  /**
   * Lookup103: evm_core::error::ExitSucceed
   **/
  EvmCoreErrorExitSucceed: {
    _enum: ['Stopped', 'Returned', 'Suicided']
  },
  /**
   * Lookup104: evm_core::error::ExitError
   **/
  EvmCoreErrorExitError: {
    _enum: {
      StackUnderflow: 'Null',
      StackOverflow: 'Null',
      InvalidJump: 'Null',
      InvalidRange: 'Null',
      DesignatedInvalid: 'Null',
      CallTooDeep: 'Null',
      CreateCollision: 'Null',
      CreateContractLimit: 'Null',
      OutOfOffset: 'Null',
      OutOfGas: 'Null',
      OutOfFund: 'Null',
      PCUnderflow: 'Null',
      CreateEmpty: 'Null',
      Other: 'Text',
      MaxNonce: 'Null',
      InvalidCode: 'u8'
    }
  },
  /**
   * Lookup108: evm_core::error::ExitRevert
   **/
  EvmCoreErrorExitRevert: {
    _enum: ['Reverted']
  },
  /**
   * Lookup109: evm_core::error::ExitFatal
   **/
  EvmCoreErrorExitFatal: {
    _enum: {
      NotSupported: 'Null',
      UnhandledInterrupt: 'Null',
      CallErrorAsFatal: 'EvmCoreErrorExitError',
      Other: 'Text'
    }
  },
  /**
   * Lookup110: pallet_evm::pallet::Event<T>
   **/
  PalletEvmEvent: {
    _enum: {
      Log: {
        log: 'EthereumLog',
      },
      Created: {
        address: 'H160',
      },
      CreatedFailed: {
        address: 'H160',
      },
      Executed: {
        address: 'H160',
      },
      ExecutedFailed: {
        address: 'H160'
      }
    }
  },
  /**
   * Lookup111: ethereum::log::Log
   **/
  EthereumLog: {
    address: 'H160',
    topics: 'Vec<H256>',
    data: 'Bytes'
  },
  /**
   * Lookup113: pallet_base_fee::pallet::Event
   **/
  PalletBaseFeeEvent: {
    _enum: {
      NewBaseFeePerGas: {
        fee: 'U256',
      },
      BaseFeeOverflow: 'Null',
      NewElasticity: {
        elasticity: 'Permill'
      }
    }
  },
  /**
   * Lookup117: pallet_airdrop_claims::pallet::Event<T>
   **/
  PalletAirdropClaimsEvent: {
    _enum: {
      Claimed: {
        recipient: 'AccountId32',
        source: 'PalletAirdropClaimsUtilsMultiAddress',
        amount: 'u128'
      }
    }
  },
  /**
   * Lookup118: pallet_airdrop_claims::utils::MultiAddress
   **/
  PalletAirdropClaimsUtilsMultiAddress: {
    _enum: {
      EVM: 'PalletAirdropClaimsUtilsEthereumAddress',
      Native: 'AccountId32'
    }
  },
  /**
   * Lookup119: pallet_airdrop_claims::utils::ethereum_address::EthereumAddress
   **/
  PalletAirdropClaimsUtilsEthereumAddress: '[u8;20]',
  /**
   * Lookup120: pallet_proxy::pallet::Event<T>
   **/
  PalletProxyEvent: {
    _enum: {
      ProxyExecuted: {
        result: 'Result<Null, SpRuntimeDispatchError>',
      },
      PureCreated: {
        pure: 'AccountId32',
        who: 'AccountId32',
        proxyType: 'TangleTestnetRuntimeProxyType',
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
        proxyType: 'TangleTestnetRuntimeProxyType',
        delay: 'u64',
      },
      ProxyRemoved: {
        delegator: 'AccountId32',
        delegatee: 'AccountId32',
        proxyType: 'TangleTestnetRuntimeProxyType',
        delay: 'u64'
      }
    }
  },
  /**
   * Lookup121: tangle_testnet_runtime::ProxyType
   **/
  TangleTestnetRuntimeProxyType: {
    _enum: ['Any', 'NonTransfer', 'Governance', 'Staking']
  },
  /**
   * Lookup123: pallet_multi_asset_delegation::pallet::Event<T>
   **/
  PalletMultiAssetDelegationEvent: {
    _enum: {
      OperatorJoined: {
        who: 'AccountId32',
      },
      OperatorLeavingScheduled: {
        who: 'AccountId32',
      },
      OperatorLeaveCancelled: {
        who: 'AccountId32',
      },
      OperatorLeaveExecuted: {
        who: 'AccountId32',
      },
      OperatorBondMore: {
        who: 'AccountId32',
        additionalBond: 'u128',
      },
      OperatorBondLessScheduled: {
        who: 'AccountId32',
        unstakeAmount: 'u128',
      },
      OperatorBondLessExecuted: {
        who: 'AccountId32',
      },
      OperatorBondLessCancelled: {
        who: 'AccountId32',
      },
      OperatorWentOffline: {
        who: 'AccountId32',
      },
      OperatorWentOnline: {
        who: 'AccountId32',
      },
      Deposited: {
        who: 'AccountId32',
        amount: 'u128',
        assetId: 'TanglePrimitivesServicesAsset',
      },
      Scheduledwithdraw: {
        who: 'AccountId32',
        amount: 'u128',
        assetId: 'TanglePrimitivesServicesAsset',
      },
      Executedwithdraw: {
        who: 'AccountId32',
      },
      Cancelledwithdraw: {
        who: 'AccountId32',
      },
      Delegated: {
        who: 'AccountId32',
        operator: 'AccountId32',
        amount: 'u128',
        assetId: 'TanglePrimitivesServicesAsset',
      },
      ScheduledDelegatorBondLess: {
        who: 'AccountId32',
        operator: 'AccountId32',
        amount: 'u128',
        assetId: 'TanglePrimitivesServicesAsset',
      },
      ExecutedDelegatorBondLess: {
        who: 'AccountId32',
      },
      CancelledDelegatorBondLess: {
        who: 'AccountId32',
      },
      OperatorSlashed: {
        who: 'AccountId32',
        amount: 'u128',
      },
      DelegatorSlashed: {
        who: 'AccountId32',
        amount: 'u128',
      },
      EvmReverted: {
        from: 'H160',
        to: 'H160',
        data: 'Bytes',
        reason: 'Bytes'
      }
    }
  },
  /**
   * Lookup124: tangle_primitives::services::Asset<AssetId>
   **/
  TanglePrimitivesServicesAsset: {
    _enum: {
      Custom: 'u128',
      Erc20: 'H160'
    }
  },
  /**
   * Lookup125: pallet_services::module::Event<T>
   **/
  PalletServicesModuleEvent: {
    _enum: {
      BlueprintCreated: {
        owner: 'AccountId32',
        blueprintId: 'u64',
      },
      PreRegistration: {
        operator: 'AccountId32',
        blueprintId: 'u64',
      },
      Registered: {
        provider: 'AccountId32',
        blueprintId: 'u64',
        preferences: 'TanglePrimitivesServicesOperatorPreferences',
        registrationArgs: 'Vec<TanglePrimitivesServicesField>',
      },
      Unregistered: {
        operator: 'AccountId32',
        blueprintId: 'u64',
      },
      PriceTargetsUpdated: {
        operator: 'AccountId32',
        blueprintId: 'u64',
        priceTargets: 'TanglePrimitivesServicesPriceTargets',
      },
      ServiceRequested: {
        owner: 'AccountId32',
        requestId: 'u64',
        blueprintId: 'u64',
        pendingApprovals: 'Vec<AccountId32>',
        approved: 'Vec<AccountId32>',
        assets: 'Vec<u128>',
      },
      ServiceRequestApproved: {
        operator: 'AccountId32',
        requestId: 'u64',
        blueprintId: 'u64',
        pendingApprovals: 'Vec<AccountId32>',
        approved: 'Vec<AccountId32>',
      },
      ServiceRequestRejected: {
        operator: 'AccountId32',
        requestId: 'u64',
        blueprintId: 'u64',
      },
      ServiceInitiated: {
        owner: 'AccountId32',
        requestId: 'u64',
        serviceId: 'u64',
        blueprintId: 'u64',
        assets: 'Vec<u128>',
      },
      ServiceTerminated: {
        owner: 'AccountId32',
        serviceId: 'u64',
        blueprintId: 'u64',
      },
      JobCalled: {
        caller: 'AccountId32',
        serviceId: 'u64',
        callId: 'u64',
        job: 'u8',
        args: 'Vec<TanglePrimitivesServicesField>',
      },
      JobResultSubmitted: {
        operator: 'AccountId32',
        serviceId: 'u64',
        callId: 'u64',
        job: 'u8',
        result: 'Vec<TanglePrimitivesServicesField>',
      },
      EvmReverted: {
        from: 'H160',
        to: 'H160',
        data: 'Bytes',
        reason: 'Bytes',
      },
      UnappliedSlash: {
        index: 'u32',
        operator: 'AccountId32',
        amount: 'u128',
        serviceId: 'u64',
        blueprintId: 'u64',
        era: 'u32',
      },
      SlashDiscarded: {
        index: 'u32',
        operator: 'AccountId32',
        amount: 'u128',
        serviceId: 'u64',
        blueprintId: 'u64',
        era: 'u32',
      },
      MasterBlueprintServiceManagerRevised: {
        revision: 'u32',
        address: 'H160'
      }
    }
  },
  /**
   * Lookup126: tangle_primitives::services::OperatorPreferences
   **/
  TanglePrimitivesServicesOperatorPreferences: {
    key: '[u8;65]',
    priceTargets: 'TanglePrimitivesServicesPriceTargets'
  },
  /**
   * Lookup128: tangle_primitives::services::PriceTargets
   **/
  TanglePrimitivesServicesPriceTargets: {
    cpu: 'u64',
    mem: 'u64',
    storageHdd: 'u64',
    storageSsd: 'u64',
    storageNvme: 'u64'
  },
  /**
   * Lookup130: tangle_primitives::services::field::Field<C, sp_core::crypto::AccountId32>
   **/
  TanglePrimitivesServicesField: {
    _enum: {
      None: 'Null',
      Bool: 'bool',
      Uint8: 'u8',
      Int8: 'i8',
      Uint16: 'u16',
      Int16: 'i16',
      Uint32: 'u32',
      Int32: 'i32',
      Uint64: 'u64',
      Int64: 'i64',
      String: 'Bytes',
      Bytes: 'Bytes',
      Array: 'Vec<TanglePrimitivesServicesField>',
      List: 'Vec<TanglePrimitivesServicesField>',
      Struct: '(Bytes,Vec<(Bytes,TanglePrimitivesServicesField)>)',
      __Unused15: 'Null',
      __Unused16: 'Null',
      __Unused17: 'Null',
      __Unused18: 'Null',
      __Unused19: 'Null',
      __Unused20: 'Null',
      __Unused21: 'Null',
      __Unused22: 'Null',
      __Unused23: 'Null',
      __Unused24: 'Null',
      __Unused25: 'Null',
      __Unused26: 'Null',
      __Unused27: 'Null',
      __Unused28: 'Null',
      __Unused29: 'Null',
      __Unused30: 'Null',
      __Unused31: 'Null',
      __Unused32: 'Null',
      __Unused33: 'Null',
      __Unused34: 'Null',
      __Unused35: 'Null',
      __Unused36: 'Null',
      __Unused37: 'Null',
      __Unused38: 'Null',
      __Unused39: 'Null',
      __Unused40: 'Null',
      __Unused41: 'Null',
      __Unused42: 'Null',
      __Unused43: 'Null',
      __Unused44: 'Null',
      __Unused45: 'Null',
      __Unused46: 'Null',
      __Unused47: 'Null',
      __Unused48: 'Null',
      __Unused49: 'Null',
      __Unused50: 'Null',
      __Unused51: 'Null',
      __Unused52: 'Null',
      __Unused53: 'Null',
      __Unused54: 'Null',
      __Unused55: 'Null',
      __Unused56: 'Null',
      __Unused57: 'Null',
      __Unused58: 'Null',
      __Unused59: 'Null',
      __Unused60: 'Null',
      __Unused61: 'Null',
      __Unused62: 'Null',
      __Unused63: 'Null',
      __Unused64: 'Null',
      __Unused65: 'Null',
      __Unused66: 'Null',
      __Unused67: 'Null',
      __Unused68: 'Null',
      __Unused69: 'Null',
      __Unused70: 'Null',
      __Unused71: 'Null',
      __Unused72: 'Null',
      __Unused73: 'Null',
      __Unused74: 'Null',
      __Unused75: 'Null',
      __Unused76: 'Null',
      __Unused77: 'Null',
      __Unused78: 'Null',
      __Unused79: 'Null',
      __Unused80: 'Null',
      __Unused81: 'Null',
      __Unused82: 'Null',
      __Unused83: 'Null',
      __Unused84: 'Null',
      __Unused85: 'Null',
      __Unused86: 'Null',
      __Unused87: 'Null',
      __Unused88: 'Null',
      __Unused89: 'Null',
      __Unused90: 'Null',
      __Unused91: 'Null',
      __Unused92: 'Null',
      __Unused93: 'Null',
      __Unused94: 'Null',
      __Unused95: 'Null',
      __Unused96: 'Null',
      __Unused97: 'Null',
      __Unused98: 'Null',
      __Unused99: 'Null',
      AccountId: 'AccountId32'
    }
  },
  /**
   * Lookup143: pallet_tangle_lst::pallet::Event<T>
   **/
  PalletTangleLstEvent: {
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
        newState: 'PalletTangleLstPoolsPoolState',
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
        changeRate: 'PalletTangleLstCommissionCommissionChangeRate',
      },
      PoolCommissionClaimPermissionUpdated: {
        poolId: 'u32',
        permission: 'Option<PalletTangleLstCommissionCommissionClaimPermission>',
      },
      PoolCommissionClaimed: {
        poolId: 'u32',
        commission: 'u128',
      },
      MinBalanceDeficitAdjusted: {
        poolId: 'u32',
        amount: 'u128',
      },
      MinBalanceExcessAdjusted: {
        poolId: 'u32',
        amount: 'u128',
      },
      LastPoolIdUpdated: {
        poolId: 'u32'
      }
    }
  },
  /**
   * Lookup144: pallet_tangle_lst::types::pools::PoolState
   **/
  PalletTangleLstPoolsPoolState: {
    _enum: ['Open', 'Blocked', 'Destroying']
  },
  /**
   * Lookup145: pallet_tangle_lst::types::commission::CommissionChangeRate<BlockNumber>
   **/
  PalletTangleLstCommissionCommissionChangeRate: {
    maxIncrease: 'Perbill',
    minDelay: 'u64'
  },
  /**
   * Lookup147: pallet_tangle_lst::types::commission::CommissionClaimPermission<sp_core::crypto::AccountId32>
   **/
  PalletTangleLstCommissionCommissionClaimPermission: {
    _enum: {
      Permissionless: 'Null',
      Account: 'AccountId32'
    }
  },
  /**
   * Lookup148: pallet_rewards::pallet::Event<T>
   **/
  PalletRewardsEvent: {
    _enum: {
      RewardsClaimed: {
        account: 'AccountId32',
        asset: 'TanglePrimitivesServicesAsset',
        amount: 'u128',
      },
      IncentiveAPYAndCapSet: {
        vaultId: 'u32',
        apy: 'Percent',
        cap: 'u128',
      },
      BlueprintWhitelisted: {
        blueprintId: 'u64',
      },
      AssetUpdatedInVault: {
        vaultId: 'u32',
        assetId: 'TanglePrimitivesServicesAsset',
        action: 'PalletRewardsAssetAction',
      },
      VaultRewardConfigUpdated: {
        vaultId: 'u32',
        newConfig: 'PalletRewardsRewardConfigForAssetVault',
      },
      RewardVaultCreated: {
        vaultId: 'u32',
        newConfig: 'PalletRewardsRewardConfigForAssetVault',
        potAccount: 'AccountId32',
      },
      TotalScoreUpdated: {
        vaultId: 'u32',
        asset: 'TanglePrimitivesServicesAsset',
        totalScore: 'u128',
        lockMultiplier: 'Option<TanglePrimitivesRewardsLockMultiplier>',
      },
      TotalDepositUpdated: {
        vaultId: 'u32',
        asset: 'TanglePrimitivesServicesAsset',
        totalDeposit: 'u128',
      },
      DecayConfigUpdated: {
        startPeriod: 'u64',
        rate: 'Percent',
      },
      ApyBlocksUpdated: {
        blocks: 'u64'
      }
    }
  },
  /**
   * Lookup150: pallet_rewards::types::AssetAction
   **/
  PalletRewardsAssetAction: {
    _enum: ['Add', 'Remove']
  },
  /**
   * Lookup151: pallet_rewards::types::RewardConfigForAssetVault<Balance>
   **/
  PalletRewardsRewardConfigForAssetVault: {
    apy: 'Percent',
    incentiveCap: 'u128',
    depositCap: 'u128',
    boostMultiplier: 'Option<u32>'
  },
  /**
   * Lookup154: tangle_primitives::types::rewards::LockMultiplier
   **/
  TanglePrimitivesRewardsLockMultiplier: {
    _enum: ['__Unused0', 'OneMonth', 'TwoMonths', 'ThreeMonths', '__Unused4', '__Unused5', 'SixMonths']
  },
  /**
   * Lookup155: frame_system::Phase
   **/
  FrameSystemPhase: {
    _enum: {
      ApplyExtrinsic: 'u32',
      Finalization: 'Null',
      Initialization: 'Null'
    }
  },
  /**
   * Lookup157: frame_system::LastRuntimeUpgradeInfo
   **/
  FrameSystemLastRuntimeUpgradeInfo: {
    specVersion: 'Compact<u32>',
    specName: 'Text'
  },
  /**
   * Lookup159: frame_system::CodeUpgradeAuthorization<T>
   **/
  FrameSystemCodeUpgradeAuthorization: {
    codeHash: 'H256',
    checkVersion: 'bool'
  },
  /**
   * Lookup160: frame_system::pallet::Call<T>
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
      __Unused8: 'Null',
      authorize_upgrade: {
        codeHash: 'H256',
      },
      authorize_upgrade_without_checks: {
        codeHash: 'H256',
      },
      apply_authorized_upgrade: {
        code: 'Bytes'
      }
    }
  },
  /**
   * Lookup164: frame_system::limits::BlockWeights
   **/
  FrameSystemLimitsBlockWeights: {
    baseBlock: 'SpWeightsWeightV2Weight',
    maxBlock: 'SpWeightsWeightV2Weight',
    perClass: 'FrameSupportDispatchPerDispatchClassWeightsPerClass'
  },
  /**
   * Lookup165: frame_support::dispatch::PerDispatchClass<frame_system::limits::WeightsPerClass>
   **/
  FrameSupportDispatchPerDispatchClassWeightsPerClass: {
    normal: 'FrameSystemLimitsWeightsPerClass',
    operational: 'FrameSystemLimitsWeightsPerClass',
    mandatory: 'FrameSystemLimitsWeightsPerClass'
  },
  /**
   * Lookup166: frame_system::limits::WeightsPerClass
   **/
  FrameSystemLimitsWeightsPerClass: {
    baseExtrinsic: 'SpWeightsWeightV2Weight',
    maxExtrinsic: 'Option<SpWeightsWeightV2Weight>',
    maxTotal: 'Option<SpWeightsWeightV2Weight>',
    reserved: 'Option<SpWeightsWeightV2Weight>'
  },
  /**
   * Lookup168: frame_system::limits::BlockLength
   **/
  FrameSystemLimitsBlockLength: {
    max: 'FrameSupportDispatchPerDispatchClassU32'
  },
  /**
   * Lookup169: frame_support::dispatch::PerDispatchClass<T>
   **/
  FrameSupportDispatchPerDispatchClassU32: {
    normal: 'u32',
    operational: 'u32',
    mandatory: 'u32'
  },
  /**
   * Lookup170: sp_weights::RuntimeDbWeight
   **/
  SpWeightsRuntimeDbWeight: {
    read: 'u64',
    write: 'u64'
  },
  /**
   * Lookup171: sp_version::RuntimeVersion
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
   * Lookup176: frame_system::pallet::Error<T>
   **/
  FrameSystemError: {
    _enum: ['InvalidSpecName', 'SpecVersionNeedsToIncrease', 'FailedToExtractRuntimeVersion', 'NonDefaultComposite', 'NonZeroRefCount', 'CallFiltered', 'MultiBlockMigrationsOngoing', 'NothingAuthorized', 'Unauthorized']
  },
  /**
   * Lookup177: pallet_timestamp::pallet::Call<T>
   **/
  PalletTimestampCall: {
    _enum: {
      set: {
        now: 'Compact<u64>'
      }
    }
  },
  /**
   * Lookup178: pallet_sudo::pallet::Call<T>
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
      remove_key: 'Null'
    }
  },
  /**
   * Lookup180: pallet_assets::pallet::Call<T, I>
   **/
  PalletAssetsCall: {
    _enum: {
      create: {
        id: 'Compact<u128>',
        admin: 'MultiAddress',
        minBalance: 'u128',
      },
      force_create: {
        id: 'Compact<u128>',
        owner: 'MultiAddress',
        isSufficient: 'bool',
        minBalance: 'Compact<u128>',
      },
      start_destroy: {
        id: 'Compact<u128>',
      },
      destroy_accounts: {
        id: 'Compact<u128>',
      },
      destroy_approvals: {
        id: 'Compact<u128>',
      },
      finish_destroy: {
        id: 'Compact<u128>',
      },
      mint: {
        id: 'Compact<u128>',
        beneficiary: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      burn: {
        id: 'Compact<u128>',
        who: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      transfer: {
        id: 'Compact<u128>',
        target: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      transfer_keep_alive: {
        id: 'Compact<u128>',
        target: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      force_transfer: {
        id: 'Compact<u128>',
        source: 'MultiAddress',
        dest: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      freeze: {
        id: 'Compact<u128>',
        who: 'MultiAddress',
      },
      thaw: {
        id: 'Compact<u128>',
        who: 'MultiAddress',
      },
      freeze_asset: {
        id: 'Compact<u128>',
      },
      thaw_asset: {
        id: 'Compact<u128>',
      },
      transfer_ownership: {
        id: 'Compact<u128>',
        owner: 'MultiAddress',
      },
      set_team: {
        id: 'Compact<u128>',
        issuer: 'MultiAddress',
        admin: 'MultiAddress',
        freezer: 'MultiAddress',
      },
      set_metadata: {
        id: 'Compact<u128>',
        name: 'Bytes',
        symbol: 'Bytes',
        decimals: 'u8',
      },
      clear_metadata: {
        id: 'Compact<u128>',
      },
      force_set_metadata: {
        id: 'Compact<u128>',
        name: 'Bytes',
        symbol: 'Bytes',
        decimals: 'u8',
        isFrozen: 'bool',
      },
      force_clear_metadata: {
        id: 'Compact<u128>',
      },
      force_asset_status: {
        id: 'Compact<u128>',
        owner: 'MultiAddress',
        issuer: 'MultiAddress',
        admin: 'MultiAddress',
        freezer: 'MultiAddress',
        minBalance: 'Compact<u128>',
        isSufficient: 'bool',
        isFrozen: 'bool',
      },
      approve_transfer: {
        id: 'Compact<u128>',
        delegate: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      cancel_approval: {
        id: 'Compact<u128>',
        delegate: 'MultiAddress',
      },
      force_cancel_approval: {
        id: 'Compact<u128>',
        owner: 'MultiAddress',
        delegate: 'MultiAddress',
      },
      transfer_approved: {
        id: 'Compact<u128>',
        owner: 'MultiAddress',
        destination: 'MultiAddress',
        amount: 'Compact<u128>',
      },
      touch: {
        id: 'Compact<u128>',
      },
      refund: {
        id: 'Compact<u128>',
        allowBurn: 'bool',
      },
      set_min_balance: {
        id: 'Compact<u128>',
        minBalance: 'u128',
      },
      touch_other: {
        id: 'Compact<u128>',
        who: 'MultiAddress',
      },
      refund_other: {
        id: 'Compact<u128>',
        who: 'MultiAddress',
      },
      block: {
        id: 'Compact<u128>',
        who: 'MultiAddress'
      }
    }
  },
  /**
   * Lookup182: pallet_balances::pallet::Call<T, I>
   **/
  PalletBalancesCall: {
    _enum: {
      transfer_allow_death: {
        dest: 'MultiAddress',
        value: 'Compact<u128>',
      },
      __Unused1: 'Null',
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
      upgrade_accounts: {
        who: 'Vec<AccountId32>',
      },
      __Unused7: 'Null',
      force_set_balance: {
        who: 'MultiAddress',
        newFree: 'Compact<u128>',
      },
      force_adjust_total_issuance: {
        direction: 'PalletBalancesAdjustmentDirection',
        delta: 'Compact<u128>',
      },
      burn: {
        value: 'Compact<u128>',
        keepAlive: 'bool'
      }
    }
  },
  /**
   * Lookup183: pallet_balances::types::AdjustmentDirection
   **/
  PalletBalancesAdjustmentDirection: {
    _enum: ['Increase', 'Decrease']
  },
  /**
   * Lookup184: pallet_babe::pallet::Call<T>
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
        config: 'SpConsensusBabeDigestsNextConfigDescriptor'
      }
    }
  },
  /**
   * Lookup185: sp_consensus_slots::EquivocationProof<sp_runtime::generic::header::Header<Number, Hash>, sp_consensus_babe::app::Public>
   **/
  SpConsensusSlotsEquivocationProof: {
    offender: 'SpConsensusBabeAppPublic',
    slot: 'u64',
    firstHeader: 'SpRuntimeHeader',
    secondHeader: 'SpRuntimeHeader'
  },
  /**
   * Lookup186: sp_runtime::generic::header::Header<Number, Hash>
   **/
  SpRuntimeHeader: {
    parentHash: 'H256',
    number: 'Compact<u64>',
    stateRoot: 'H256',
    extrinsicsRoot: 'H256',
    digest: 'SpRuntimeDigest'
  },
  /**
   * Lookup187: sp_consensus_babe::app::Public
   **/
  SpConsensusBabeAppPublic: '[u8;32]',
  /**
   * Lookup189: sp_session::MembershipProof
   **/
  SpSessionMembershipProof: {
    session: 'u32',
    trieNodes: 'Vec<Bytes>',
    validatorCount: 'u32'
  },
  /**
   * Lookup190: sp_consensus_babe::digests::NextConfigDescriptor
   **/
  SpConsensusBabeDigestsNextConfigDescriptor: {
    _enum: {
      __Unused0: 'Null',
      V1: {
        c: '(u64,u64)',
        allowedSlots: 'SpConsensusBabeAllowedSlots'
      }
    }
  },
  /**
   * Lookup192: sp_consensus_babe::AllowedSlots
   **/
  SpConsensusBabeAllowedSlots: {
    _enum: ['PrimarySlots', 'PrimaryAndSecondaryPlainSlots', 'PrimaryAndSecondaryVRFSlots']
  },
  /**
   * Lookup193: pallet_grandpa::pallet::Call<T>
   **/
  PalletGrandpaCall: {
    _enum: {
      report_equivocation: {
        equivocationProof: 'SpConsensusGrandpaEquivocationProof',
        keyOwnerProof: 'SpCoreVoid',
      },
      report_equivocation_unsigned: {
        equivocationProof: 'SpConsensusGrandpaEquivocationProof',
        keyOwnerProof: 'SpCoreVoid',
      },
      note_stalled: {
        delay: 'u64',
        bestFinalizedBlockNumber: 'u64'
      }
    }
  },
  /**
   * Lookup194: sp_consensus_grandpa::EquivocationProof<primitive_types::H256, N>
   **/
  SpConsensusGrandpaEquivocationProof: {
    setId: 'u64',
    equivocation: 'SpConsensusGrandpaEquivocation'
  },
  /**
   * Lookup195: sp_consensus_grandpa::Equivocation<primitive_types::H256, N>
   **/
  SpConsensusGrandpaEquivocation: {
    _enum: {
      Prevote: 'FinalityGrandpaEquivocationPrevote',
      Precommit: 'FinalityGrandpaEquivocationPrecommit'
    }
  },
  /**
   * Lookup196: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Prevote<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrevote: {
    roundNumber: 'u64',
    identity: 'SpConsensusGrandpaAppPublic',
    first: '(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)',
    second: '(FinalityGrandpaPrevote,SpConsensusGrandpaAppSignature)'
  },
  /**
   * Lookup197: finality_grandpa::Prevote<primitive_types::H256, N>
   **/
  FinalityGrandpaPrevote: {
    targetHash: 'H256',
    targetNumber: 'u64'
  },
  /**
   * Lookup198: sp_consensus_grandpa::app::Signature
   **/
  SpConsensusGrandpaAppSignature: '[u8;64]',
  /**
   * Lookup201: finality_grandpa::Equivocation<sp_consensus_grandpa::app::Public, finality_grandpa::Precommit<primitive_types::H256, N>, sp_consensus_grandpa::app::Signature>
   **/
  FinalityGrandpaEquivocationPrecommit: {
    roundNumber: 'u64',
    identity: 'SpConsensusGrandpaAppPublic',
    first: '(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)',
    second: '(FinalityGrandpaPrecommit,SpConsensusGrandpaAppSignature)'
  },
  /**
   * Lookup202: finality_grandpa::Precommit<primitive_types::H256, N>
   **/
  FinalityGrandpaPrecommit: {
    targetHash: 'H256',
    targetNumber: 'u64'
  },
  /**
   * Lookup204: sp_core::Void
   **/
  SpCoreVoid: 'Null',
  /**
   * Lookup205: pallet_indices::pallet::Call<T>
   **/
  PalletIndicesCall: {
    _enum: {
      claim: {
        index: 'u32',
      },
      transfer: {
        _alias: {
          new_: 'new',
        },
        new_: 'MultiAddress',
        index: 'u32',
      },
      free: {
        index: 'u32',
      },
      force_transfer: {
        _alias: {
          new_: 'new',
        },
        new_: 'MultiAddress',
        index: 'u32',
        freeze: 'bool',
      },
      freeze: {
        index: 'u32'
      }
    }
  },
  /**
   * Lookup206: pallet_democracy::pallet::Call<T>
   **/
  PalletDemocracyCall: {
    _enum: {
      propose: {
        proposal: 'FrameSupportPreimagesBounded',
        value: 'Compact<u128>',
      },
      second: {
        proposal: 'Compact<u32>',
      },
      vote: {
        refIndex: 'Compact<u32>',
        vote: 'PalletDemocracyVoteAccountVote',
      },
      emergency_cancel: {
        refIndex: 'u32',
      },
      external_propose: {
        proposal: 'FrameSupportPreimagesBounded',
      },
      external_propose_majority: {
        proposal: 'FrameSupportPreimagesBounded',
      },
      external_propose_default: {
        proposal: 'FrameSupportPreimagesBounded',
      },
      fast_track: {
        proposalHash: 'H256',
        votingPeriod: 'u64',
        delay: 'u64',
      },
      veto_external: {
        proposalHash: 'H256',
      },
      cancel_referendum: {
        refIndex: 'Compact<u32>',
      },
      delegate: {
        to: 'MultiAddress',
        conviction: 'PalletDemocracyConviction',
        balance: 'u128',
      },
      undelegate: 'Null',
      clear_public_proposals: 'Null',
      unlock: {
        target: 'MultiAddress',
      },
      remove_vote: {
        index: 'u32',
      },
      remove_other_vote: {
        target: 'MultiAddress',
        index: 'u32',
      },
      blacklist: {
        proposalHash: 'H256',
        maybeRefIndex: 'Option<u32>',
      },
      cancel_proposal: {
        propIndex: 'Compact<u32>',
      },
      set_metadata: {
        owner: 'PalletDemocracyMetadataOwner',
        maybeHash: 'Option<H256>'
      }
    }
  },
  /**
   * Lookup207: frame_support::traits::preimages::Bounded<tangle_testnet_runtime::RuntimeCall, sp_runtime::traits::BlakeTwo256>
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
        len: 'u32'
      }
    }
  },
  /**
   * Lookup208: sp_runtime::traits::BlakeTwo256
   **/
  SpRuntimeBlakeTwo256: 'Null',
  /**
   * Lookup210: pallet_democracy::conviction::Conviction
   **/
  PalletDemocracyConviction: {
    _enum: ['None', 'Locked1x', 'Locked2x', 'Locked3x', 'Locked4x', 'Locked5x', 'Locked6x']
  },
  /**
   * Lookup212: pallet_collective::pallet::Call<T, I>
   **/
  PalletCollectiveCall: {
    _enum: {
      set_members: {
        newMembers: 'Vec<AccountId32>',
        prime: 'Option<AccountId32>',
        oldCount: 'u32',
      },
      execute: {
        proposal: 'Call',
        lengthBound: 'Compact<u32>',
      },
      propose: {
        threshold: 'Compact<u32>',
        proposal: 'Call',
        lengthBound: 'Compact<u32>',
      },
      vote: {
        proposal: 'H256',
        index: 'Compact<u32>',
        approve: 'bool',
      },
      __Unused4: 'Null',
      disapprove_proposal: {
        proposalHash: 'H256',
      },
      close: {
        proposalHash: 'H256',
        index: 'Compact<u32>',
        proposalWeightBound: 'SpWeightsWeightV2Weight',
        lengthBound: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup213: pallet_vesting::pallet::Call<T>
   **/
  PalletVestingCall: {
    _enum: {
      vest: 'Null',
      vest_other: {
        target: 'MultiAddress',
      },
      vested_transfer: {
        target: 'MultiAddress',
        schedule: 'PalletVestingVestingInfo',
      },
      force_vested_transfer: {
        source: 'MultiAddress',
        target: 'MultiAddress',
        schedule: 'PalletVestingVestingInfo',
      },
      merge_schedules: {
        schedule1Index: 'u32',
        schedule2Index: 'u32',
      },
      force_remove_vesting_schedule: {
        target: 'MultiAddress',
        scheduleIndex: 'u32'
      }
    }
  },
  /**
   * Lookup214: pallet_vesting::vesting_info::VestingInfo<Balance, BlockNumber>
   **/
  PalletVestingVestingInfo: {
    locked: 'u128',
    perBlock: 'u128',
    startingBlock: 'u64'
  },
  /**
   * Lookup215: pallet_elections_phragmen::pallet::Call<T>
   **/
  PalletElectionsPhragmenCall: {
    _enum: {
      vote: {
        votes: 'Vec<AccountId32>',
        value: 'Compact<u128>',
      },
      remove_voter: 'Null',
      submit_candidacy: {
        candidateCount: 'Compact<u32>',
      },
      renounce_candidacy: {
        renouncing: 'PalletElectionsPhragmenRenouncing',
      },
      remove_member: {
        who: 'MultiAddress',
        slashBond: 'bool',
        rerunElection: 'bool',
      },
      clean_defunct_voters: {
        numVoters: 'u32',
        numDefunct: 'u32'
      }
    }
  },
  /**
   * Lookup216: pallet_elections_phragmen::Renouncing
   **/
  PalletElectionsPhragmenRenouncing: {
    _enum: {
      Member: 'Null',
      RunnerUp: 'Null',
      Candidate: 'Compact<u32>'
    }
  },
  /**
   * Lookup217: pallet_election_provider_multi_phase::pallet::Call<T>
   **/
  PalletElectionProviderMultiPhaseCall: {
    _enum: {
      submit_unsigned: {
        rawSolution: 'PalletElectionProviderMultiPhaseRawSolution',
        witness: 'PalletElectionProviderMultiPhaseSolutionOrSnapshotSize',
      },
      set_minimum_untrusted_score: {
        maybeNextScore: 'Option<SpNposElectionsElectionScore>',
      },
      set_emergency_election_result: {
        supports: 'Vec<(AccountId32,SpNposElectionsSupport)>',
      },
      submit: {
        rawSolution: 'PalletElectionProviderMultiPhaseRawSolution',
      },
      governance_fallback: {
        maybeMaxVoters: 'Option<u32>',
        maybeMaxTargets: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup218: pallet_election_provider_multi_phase::RawSolution<tangle_testnet_runtime::NposSolution16>
   **/
  PalletElectionProviderMultiPhaseRawSolution: {
    solution: 'TangleTestnetRuntimeNposSolution16',
    score: 'SpNposElectionsElectionScore',
    round: 'u32'
  },
  /**
   * Lookup219: tangle_testnet_runtime::NposSolution16
   **/
  TangleTestnetRuntimeNposSolution16: {
    votes1: 'Vec<(Compact<u32>,Compact<u16>)>',
    votes2: 'Vec<(Compact<u32>,(Compact<u16>,Compact<PerU16>),Compact<u16>)>',
    votes3: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);2],Compact<u16>)>',
    votes4: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);3],Compact<u16>)>',
    votes5: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);4],Compact<u16>)>',
    votes6: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);5],Compact<u16>)>',
    votes7: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);6],Compact<u16>)>',
    votes8: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);7],Compact<u16>)>',
    votes9: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);8],Compact<u16>)>',
    votes10: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);9],Compact<u16>)>',
    votes11: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);10],Compact<u16>)>',
    votes12: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);11],Compact<u16>)>',
    votes13: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);12],Compact<u16>)>',
    votes14: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);13],Compact<u16>)>',
    votes15: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);14],Compact<u16>)>',
    votes16: 'Vec<(Compact<u32>,[(Compact<u16>,Compact<PerU16>);15],Compact<u16>)>'
  },
  /**
   * Lookup270: pallet_election_provider_multi_phase::SolutionOrSnapshotSize
   **/
  PalletElectionProviderMultiPhaseSolutionOrSnapshotSize: {
    voters: 'Compact<u32>',
    targets: 'Compact<u32>'
  },
  /**
   * Lookup274: sp_npos_elections::Support<sp_core::crypto::AccountId32>
   **/
  SpNposElectionsSupport: {
    total: 'u128',
    voters: 'Vec<(AccountId32,u128)>'
  },
  /**
   * Lookup275: pallet_staking::pallet::pallet::Call<T>
   **/
  PalletStakingPalletCall: {
    _enum: {
      bond: {
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
      set_controller: 'Null',
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
        maxStakedRewards: 'PalletStakingPalletConfigOpPercent',
      },
      chill_other: {
        stash: 'AccountId32',
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
      payout_stakers_by_page: {
        validatorStash: 'AccountId32',
        era: 'u32',
        page: 'u32',
      },
      update_payee: {
        controller: 'AccountId32',
      },
      deprecate_controller_batch: {
        controllers: 'Vec<AccountId32>',
      },
      restore_ledger: {
        stash: 'AccountId32',
        maybeController: 'Option<AccountId32>',
        maybeTotal: 'Option<u128>',
        maybeUnlocking: 'Option<Vec<PalletStakingUnlockChunk>>'
      }
    }
  },
  /**
   * Lookup278: pallet_staking::pallet::pallet::ConfigOp<T>
   **/
  PalletStakingPalletConfigOpU128: {
    _enum: {
      Noop: 'Null',
      Set: 'u128',
      Remove: 'Null'
    }
  },
  /**
   * Lookup279: pallet_staking::pallet::pallet::ConfigOp<T>
   **/
  PalletStakingPalletConfigOpU32: {
    _enum: {
      Noop: 'Null',
      Set: 'u32',
      Remove: 'Null'
    }
  },
  /**
   * Lookup280: pallet_staking::pallet::pallet::ConfigOp<sp_arithmetic::per_things::Percent>
   **/
  PalletStakingPalletConfigOpPercent: {
    _enum: {
      Noop: 'Null',
      Set: 'Percent',
      Remove: 'Null'
    }
  },
  /**
   * Lookup281: pallet_staking::pallet::pallet::ConfigOp<sp_arithmetic::per_things::Perbill>
   **/
  PalletStakingPalletConfigOpPerbill: {
    _enum: {
      Noop: 'Null',
      Set: 'Perbill',
      Remove: 'Null'
    }
  },
  /**
   * Lookup286: pallet_staking::UnlockChunk<Balance>
   **/
  PalletStakingUnlockChunk: {
    value: 'Compact<u128>',
    era: 'Compact<u32>'
  },
  /**
   * Lookup288: pallet_session::pallet::Call<T>
   **/
  PalletSessionCall: {
    _enum: {
      set_keys: {
        _alias: {
          keys_: 'keys',
        },
        keys_: 'TangleTestnetRuntimeOpaqueSessionKeys',
        proof: 'Bytes',
      },
      purge_keys: 'Null'
    }
  },
  /**
   * Lookup289: tangle_testnet_runtime::opaque::SessionKeys
   **/
  TangleTestnetRuntimeOpaqueSessionKeys: {
    babe: 'SpConsensusBabeAppPublic',
    grandpa: 'SpConsensusGrandpaAppPublic',
    imOnline: 'PalletImOnlineSr25519AppSr25519Public'
  },
  /**
   * Lookup290: pallet_treasury::pallet::Call<T, I>
   **/
  PalletTreasuryCall: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      __Unused2: 'Null',
      spend_local: {
        amount: 'Compact<u128>',
        beneficiary: 'MultiAddress',
      },
      remove_approval: {
        proposalId: 'Compact<u32>',
      },
      spend: {
        assetKind: 'Null',
        amount: 'Compact<u128>',
        beneficiary: 'AccountId32',
        validFrom: 'Option<u64>',
      },
      payout: {
        index: 'u32',
      },
      check_status: {
        index: 'u32',
      },
      void_spend: {
        index: 'u32'
      }
    }
  },
  /**
   * Lookup292: pallet_bounties::pallet::Call<T, I>
   **/
  PalletBountiesCall: {
    _enum: {
      propose_bounty: {
        value: 'Compact<u128>',
        description: 'Bytes',
      },
      approve_bounty: {
        bountyId: 'Compact<u32>',
      },
      propose_curator: {
        bountyId: 'Compact<u32>',
        curator: 'MultiAddress',
        fee: 'Compact<u128>',
      },
      unassign_curator: {
        bountyId: 'Compact<u32>',
      },
      accept_curator: {
        bountyId: 'Compact<u32>',
      },
      award_bounty: {
        bountyId: 'Compact<u32>',
        beneficiary: 'MultiAddress',
      },
      claim_bounty: {
        bountyId: 'Compact<u32>',
      },
      close_bounty: {
        bountyId: 'Compact<u32>',
      },
      extend_bounty_expiry: {
        bountyId: 'Compact<u32>',
        remark: 'Bytes'
      }
    }
  },
  /**
   * Lookup293: pallet_child_bounties::pallet::Call<T>
   **/
  PalletChildBountiesCall: {
    _enum: {
      add_child_bounty: {
        parentBountyId: 'Compact<u32>',
        value: 'Compact<u128>',
        description: 'Bytes',
      },
      propose_curator: {
        parentBountyId: 'Compact<u32>',
        childBountyId: 'Compact<u32>',
        curator: 'MultiAddress',
        fee: 'Compact<u128>',
      },
      accept_curator: {
        parentBountyId: 'Compact<u32>',
        childBountyId: 'Compact<u32>',
      },
      unassign_curator: {
        parentBountyId: 'Compact<u32>',
        childBountyId: 'Compact<u32>',
      },
      award_child_bounty: {
        parentBountyId: 'Compact<u32>',
        childBountyId: 'Compact<u32>',
        beneficiary: 'MultiAddress',
      },
      claim_child_bounty: {
        parentBountyId: 'Compact<u32>',
        childBountyId: 'Compact<u32>',
      },
      close_child_bounty: {
        parentBountyId: 'Compact<u32>',
        childBountyId: 'Compact<u32>'
      }
    }
  },
  /**
   * Lookup294: pallet_bags_list::pallet::Call<T, I>
   **/
  PalletBagsListCall: {
    _enum: {
      rebag: {
        dislocated: 'MultiAddress',
      },
      put_in_front_of: {
        lighter: 'MultiAddress',
      },
      put_in_front_of_other: {
        heavier: 'MultiAddress',
        lighter: 'MultiAddress'
      }
    }
  },
  /**
   * Lookup295: pallet_nomination_pools::pallet::Call<T>
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
      adjust_pool_deposit: {
        poolId: 'u32',
      },
      set_commission_claim_permission: {
        poolId: 'u32',
        permission: 'Option<PalletNominationPoolsCommissionClaimPermission>',
      },
      apply_slash: {
        memberAccount: 'MultiAddress',
      },
      migrate_delegation: {
        memberAccount: 'MultiAddress',
      },
      migrate_pool_to_delegate_stake: {
        poolId: 'u32'
      }
    }
  },
  /**
   * Lookup296: pallet_nomination_pools::BondExtra<Balance>
   **/
  PalletNominationPoolsBondExtra: {
    _enum: {
      FreeBalance: 'u128',
      Rewards: 'Null'
    }
  },
  /**
   * Lookup297: pallet_nomination_pools::ConfigOp<T>
   **/
  PalletNominationPoolsConfigOpU128: {
    _enum: {
      Noop: 'Null',
      Set: 'u128',
      Remove: 'Null'
    }
  },
  /**
   * Lookup298: pallet_nomination_pools::ConfigOp<T>
   **/
  PalletNominationPoolsConfigOpU32: {
    _enum: {
      Noop: 'Null',
      Set: 'u32',
      Remove: 'Null'
    }
  },
  /**
   * Lookup299: pallet_nomination_pools::ConfigOp<sp_arithmetic::per_things::Perbill>
   **/
  PalletNominationPoolsConfigOpPerbill: {
    _enum: {
      Noop: 'Null',
      Set: 'Perbill',
      Remove: 'Null'
    }
  },
  /**
   * Lookup300: pallet_nomination_pools::ConfigOp<sp_core::crypto::AccountId32>
   **/
  PalletNominationPoolsConfigOpAccountId32: {
    _enum: {
      Noop: 'Null',
      Set: 'AccountId32',
      Remove: 'Null'
    }
  },
  /**
   * Lookup301: pallet_nomination_pools::ClaimPermission
   **/
  PalletNominationPoolsClaimPermission: {
    _enum: ['Permissioned', 'PermissionlessCompound', 'PermissionlessWithdraw', 'PermissionlessAll']
  },
  /**
   * Lookup302: pallet_scheduler::pallet::Call<T>
   **/
  PalletSchedulerCall: {
    _enum: {
      schedule: {
        when: 'u64',
        maybePeriodic: 'Option<(u64,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      cancel: {
        when: 'u64',
        index: 'u32',
      },
      schedule_named: {
        id: '[u8;32]',
        when: 'u64',
        maybePeriodic: 'Option<(u64,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      cancel_named: {
        id: '[u8;32]',
      },
      schedule_after: {
        after: 'u64',
        maybePeriodic: 'Option<(u64,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      schedule_named_after: {
        id: '[u8;32]',
        after: 'u64',
        maybePeriodic: 'Option<(u64,u32)>',
        priority: 'u8',
        call: 'Call',
      },
      set_retry: {
        task: '(u64,u32)',
        retries: 'u8',
        period: 'u64',
      },
      set_retry_named: {
        id: '[u8;32]',
        retries: 'u8',
        period: 'u64',
      },
      cancel_retry: {
        task: '(u64,u32)',
      },
      cancel_retry_named: {
        id: '[u8;32]'
      }
    }
  },
  /**
   * Lookup304: pallet_preimage::pallet::Call<T>
   **/
  PalletPreimageCall: {
    _enum: {
      note_preimage: {
        bytes: 'Bytes',
      },
      unnote_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      request_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      unrequest_preimage: {
        _alias: {
          hash_: 'hash',
        },
        hash_: 'H256',
      },
      ensure_updated: {
        hashes: 'Vec<H256>'
      }
    }
  },
  /**
   * Lookup305: pallet_tx_pause::pallet::Call<T>
   **/
  PalletTxPauseCall: {
    _enum: {
      pause: {
        fullName: '(Bytes,Bytes)',
      },
      unpause: {
        ident: '(Bytes,Bytes)'
      }
    }
  },
  /**
   * Lookup306: pallet_im_online::pallet::Call<T>
   **/
  PalletImOnlineCall: {
    _enum: {
      heartbeat: {
        heartbeat: 'PalletImOnlineHeartbeat',
        signature: 'PalletImOnlineSr25519AppSr25519Signature'
      }
    }
  },
  /**
   * Lookup307: pallet_im_online::Heartbeat<BlockNumber>
   **/
  PalletImOnlineHeartbeat: {
    blockNumber: 'u64',
    sessionIndex: 'u32',
    authorityIndex: 'u32',
    validatorsLen: 'u32'
  },
  /**
   * Lookup308: pallet_im_online::sr25519::app_sr25519::Signature
   **/
  PalletImOnlineSr25519AppSr25519Signature: '[u8;64]',
  /**
   * Lookup309: pallet_identity::pallet::Call<T>
   **/
  PalletIdentityCall: {
    _enum: {
      add_registrar: {
        account: 'MultiAddress',
      },
      set_identity: {
        info: 'PalletIdentityLegacyIdentityInfo',
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
        fields: 'u64',
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
      add_username_authority: {
        authority: 'MultiAddress',
        suffix: 'Bytes',
        allocation: 'u32',
      },
      remove_username_authority: {
        authority: 'MultiAddress',
      },
      set_username_for: {
        who: 'MultiAddress',
        username: 'Bytes',
        signature: 'Option<SpRuntimeMultiSignature>',
      },
      accept_username: {
        username: 'Bytes',
      },
      remove_expired_approval: {
        username: 'Bytes',
      },
      set_primary_username: {
        username: 'Bytes',
      },
      remove_dangling_username: {
        username: 'Bytes'
      }
    }
  },
  /**
   * Lookup310: pallet_identity::legacy::IdentityInfo<FieldLimit>
   **/
  PalletIdentityLegacyIdentityInfo: {
    additional: 'Vec<(Data,Data)>',
    display: 'Data',
    legal: 'Data',
    web: 'Data',
    riot: 'Data',
    email: 'Data',
    pgpFingerprint: 'Option<[u8;20]>',
    image: 'Data',
    twitter: 'Data'
  },
  /**
   * Lookup346: pallet_identity::types::Judgement<Balance>
   **/
  PalletIdentityJudgement: {
    _enum: {
      Unknown: 'Null',
      FeePaid: 'u128',
      Reasonable: 'Null',
      KnownGood: 'Null',
      OutOfDate: 'Null',
      LowQuality: 'Null',
      Erroneous: 'Null'
    }
  },
  /**
   * Lookup348: sp_runtime::MultiSignature
   **/
  SpRuntimeMultiSignature: {
    _enum: {
      Ed25519: '[u8;64]',
      Sr25519: '[u8;64]',
      Ecdsa: '[u8;65]'
    }
  },
  /**
   * Lookup349: pallet_utility::pallet::Call<T>
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
        asOrigin: 'TangleTestnetRuntimeOriginCaller',
        call: 'Call',
      },
      force_batch: {
        calls: 'Vec<Call>',
      },
      with_weight: {
        call: 'Call',
        weight: 'SpWeightsWeightV2Weight'
      }
    }
  },
  /**
   * Lookup351: tangle_testnet_runtime::OriginCaller
   **/
  TangleTestnetRuntimeOriginCaller: {
    _enum: {
      __Unused0: 'Null',
      system: 'FrameSupportDispatchRawOrigin',
      __Unused2: 'Null',
      Void: 'SpCoreVoid',
      __Unused4: 'Null',
      __Unused5: 'Null',
      __Unused6: 'Null',
      __Unused7: 'Null',
      __Unused8: 'Null',
      __Unused9: 'Null',
      __Unused10: 'Null',
      __Unused11: 'Null',
      __Unused12: 'Null',
      Council: 'PalletCollectiveRawOrigin',
      __Unused14: 'Null',
      __Unused15: 'Null',
      __Unused16: 'Null',
      __Unused17: 'Null',
      __Unused18: 'Null',
      __Unused19: 'Null',
      __Unused20: 'Null',
      __Unused21: 'Null',
      __Unused22: 'Null',
      __Unused23: 'Null',
      __Unused24: 'Null',
      __Unused25: 'Null',
      __Unused26: 'Null',
      __Unused27: 'Null',
      __Unused28: 'Null',
      __Unused29: 'Null',
      __Unused30: 'Null',
      __Unused31: 'Null',
      __Unused32: 'Null',
      Ethereum: 'PalletEthereumRawOrigin'
    }
  },
  /**
   * Lookup352: frame_support::dispatch::RawOrigin<sp_core::crypto::AccountId32>
   **/
  FrameSupportDispatchRawOrigin: {
    _enum: {
      Root: 'Null',
      Signed: 'AccountId32',
      None: 'Null'
    }
  },
  /**
   * Lookup353: pallet_collective::RawOrigin<sp_core::crypto::AccountId32, I>
   **/
  PalletCollectiveRawOrigin: {
    _enum: {
      Members: '(u32,u32)',
      Member: 'AccountId32',
      _Phantom: 'Null'
    }
  },
  /**
   * Lookup354: pallet_ethereum::RawOrigin
   **/
  PalletEthereumRawOrigin: {
    _enum: {
      EthereumTransaction: 'H160'
    }
  },
  /**
   * Lookup355: pallet_multisig::pallet::Call<T>
   **/
  PalletMultisigCall: {
    _enum: {
      as_multi_threshold_1: {
        otherSignatories: 'Vec<AccountId32>',
        call: 'Call',
      },
      as_multi: {
        threshold: 'u16',
        otherSignatories: 'Vec<AccountId32>',
        maybeTimepoint: 'Option<PalletMultisigTimepoint>',
        call: 'Call',
        maxWeight: 'SpWeightsWeightV2Weight',
      },
      approve_as_multi: {
        threshold: 'u16',
        otherSignatories: 'Vec<AccountId32>',
        maybeTimepoint: 'Option<PalletMultisigTimepoint>',
        callHash: '[u8;32]',
        maxWeight: 'SpWeightsWeightV2Weight',
      },
      cancel_as_multi: {
        threshold: 'u16',
        otherSignatories: 'Vec<AccountId32>',
        timepoint: 'PalletMultisigTimepoint',
        callHash: '[u8;32]'
      }
    }
  },
  /**
   * Lookup357: pallet_ethereum::pallet::Call<T>
   **/
  PalletEthereumCall: {
    _enum: {
      transact: {
        transaction: 'EthereumTransactionTransactionV2'
      }
    }
  },
  /**
   * Lookup358: ethereum::transaction::TransactionV2
   **/
  EthereumTransactionTransactionV2: {
    _enum: {
      Legacy: 'EthereumTransactionLegacyTransaction',
      EIP2930: 'EthereumTransactionEip2930Transaction',
      EIP1559: 'EthereumTransactionEip1559Transaction'
    }
  },
  /**
   * Lookup359: ethereum::transaction::LegacyTransaction
   **/
  EthereumTransactionLegacyTransaction: {
    nonce: 'U256',
    gasPrice: 'U256',
    gasLimit: 'U256',
    action: 'EthereumTransactionTransactionAction',
    value: 'U256',
    input: 'Bytes',
    signature: 'EthereumTransactionTransactionSignature'
  },
  /**
   * Lookup360: ethereum::transaction::TransactionAction
   **/
  EthereumTransactionTransactionAction: {
    _enum: {
      Call: 'H160',
      Create: 'Null'
    }
  },
  /**
   * Lookup361: ethereum::transaction::TransactionSignature
   **/
  EthereumTransactionTransactionSignature: {
    v: 'u64',
    r: 'H256',
    s: 'H256'
  },
  /**
   * Lookup363: ethereum::transaction::EIP2930Transaction
   **/
  EthereumTransactionEip2930Transaction: {
    chainId: 'u64',
    nonce: 'U256',
    gasPrice: 'U256',
    gasLimit: 'U256',
    action: 'EthereumTransactionTransactionAction',
    value: 'U256',
    input: 'Bytes',
    accessList: 'Vec<EthereumTransactionAccessListItem>',
    oddYParity: 'bool',
    r: 'H256',
    s: 'H256'
  },
  /**
   * Lookup365: ethereum::transaction::AccessListItem
   **/
  EthereumTransactionAccessListItem: {
    address: 'H160',
    storageKeys: 'Vec<H256>'
  },
  /**
   * Lookup366: ethereum::transaction::EIP1559Transaction
   **/
  EthereumTransactionEip1559Transaction: {
    chainId: 'u64',
    nonce: 'U256',
    maxPriorityFeePerGas: 'U256',
    maxFeePerGas: 'U256',
    gasLimit: 'U256',
    action: 'EthereumTransactionTransactionAction',
    value: 'U256',
    input: 'Bytes',
    accessList: 'Vec<EthereumTransactionAccessListItem>',
    oddYParity: 'bool',
    r: 'H256',
    s: 'H256'
  },
  /**
   * Lookup367: pallet_evm::pallet::Call<T>
   **/
  PalletEvmCall: {
    _enum: {
      withdraw: {
        address: 'H160',
        value: 'u128',
      },
      call: {
        source: 'H160',
        target: 'H160',
        input: 'Bytes',
        value: 'U256',
        gasLimit: 'u64',
        maxFeePerGas: 'U256',
        maxPriorityFeePerGas: 'Option<U256>',
        nonce: 'Option<U256>',
        accessList: 'Vec<(H160,Vec<H256>)>',
      },
      create: {
        source: 'H160',
        init: 'Bytes',
        value: 'U256',
        gasLimit: 'u64',
        maxFeePerGas: 'U256',
        maxPriorityFeePerGas: 'Option<U256>',
        nonce: 'Option<U256>',
        accessList: 'Vec<(H160,Vec<H256>)>',
      },
      create2: {
        source: 'H160',
        init: 'Bytes',
        salt: 'H256',
        value: 'U256',
        gasLimit: 'u64',
        maxFeePerGas: 'U256',
        maxPriorityFeePerGas: 'Option<U256>',
        nonce: 'Option<U256>',
        accessList: 'Vec<(H160,Vec<H256>)>'
      }
    }
  },
  /**
   * Lookup371: pallet_dynamic_fee::pallet::Call<T>
   **/
  PalletDynamicFeeCall: {
    _enum: {
      note_min_gas_price_target: {
        target: 'U256'
      }
    }
  },
  /**
   * Lookup372: pallet_base_fee::pallet::Call<T>
   **/
  PalletBaseFeeCall: {
    _enum: {
      set_base_fee_per_gas: {
        fee: 'U256',
      },
      set_elasticity: {
        elasticity: 'Permill'
      }
    }
  },
  /**
   * Lookup373: pallet_hotfix_sufficients::pallet::Call<T>
   **/
  PalletHotfixSufficientsCall: {
    _enum: {
      hotfix_inc_account_sufficients: {
        addresses: 'Vec<H160>'
      }
    }
  },
  /**
   * Lookup375: pallet_airdrop_claims::pallet::Call<T>
   **/
  PalletAirdropClaimsCall: {
    _enum: {
      claim: {
        dest: 'Option<PalletAirdropClaimsUtilsMultiAddress>',
        signer: 'Option<PalletAirdropClaimsUtilsMultiAddress>',
        signature: 'PalletAirdropClaimsUtilsMultiAddressSignature',
      },
      mint_claim: {
        who: 'PalletAirdropClaimsUtilsMultiAddress',
        value: 'u128',
        vestingSchedule: 'Option<Vec<(u128,u128,u64)>>',
        statement: 'Option<PalletAirdropClaimsStatementKind>',
      },
      claim_attest: {
        dest: 'Option<PalletAirdropClaimsUtilsMultiAddress>',
        signer: 'Option<PalletAirdropClaimsUtilsMultiAddress>',
        signature: 'PalletAirdropClaimsUtilsMultiAddressSignature',
        statement: 'Bytes',
      },
      __Unused3: 'Null',
      move_claim: {
        _alias: {
          new_: 'new',
        },
        old: 'PalletAirdropClaimsUtilsMultiAddress',
        new_: 'PalletAirdropClaimsUtilsMultiAddress',
      },
      force_set_expiry_config: {
        expiryBlock: 'u64',
        dest: 'PalletAirdropClaimsUtilsMultiAddress',
      },
      claim_signed: {
        dest: 'Option<PalletAirdropClaimsUtilsMultiAddress>'
      }
    }
  },
  /**
   * Lookup377: pallet_airdrop_claims::utils::MultiAddressSignature
   **/
  PalletAirdropClaimsUtilsMultiAddressSignature: {
    _enum: {
      EVM: 'PalletAirdropClaimsUtilsEthereumAddressEcdsaSignature',
      Native: 'PalletAirdropClaimsUtilsSr25519Signature'
    }
  },
  /**
   * Lookup378: pallet_airdrop_claims::utils::ethereum_address::EcdsaSignature
   **/
  PalletAirdropClaimsUtilsEthereumAddressEcdsaSignature: '[u8;65]',
  /**
   * Lookup379: pallet_airdrop_claims::utils::Sr25519Signature
   **/
  PalletAirdropClaimsUtilsSr25519Signature: '[u8;64]',
  /**
   * Lookup385: pallet_airdrop_claims::StatementKind
   **/
  PalletAirdropClaimsStatementKind: {
    _enum: ['Regular', 'Safe']
  },
  /**
   * Lookup386: pallet_proxy::pallet::Call<T>
   **/
  PalletProxyCall: {
    _enum: {
      proxy: {
        real: 'MultiAddress',
        forceProxyType: 'Option<TangleTestnetRuntimeProxyType>',
        call: 'Call',
      },
      add_proxy: {
        delegate: 'MultiAddress',
        proxyType: 'TangleTestnetRuntimeProxyType',
        delay: 'u64',
      },
      remove_proxy: {
        delegate: 'MultiAddress',
        proxyType: 'TangleTestnetRuntimeProxyType',
        delay: 'u64',
      },
      remove_proxies: 'Null',
      create_pure: {
        proxyType: 'TangleTestnetRuntimeProxyType',
        delay: 'u64',
        index: 'u16',
      },
      kill_pure: {
        spawner: 'MultiAddress',
        proxyType: 'TangleTestnetRuntimeProxyType',
        index: 'u16',
        height: 'Compact<u64>',
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
        forceProxyType: 'Option<TangleTestnetRuntimeProxyType>',
        call: 'Call'
      }
    }
  },
  /**
   * Lookup388: pallet_multi_asset_delegation::pallet::Call<T>
   **/
  PalletMultiAssetDelegationCall: {
    _enum: {
      join_operators: {
        bondAmount: 'u128',
      },
      schedule_leave_operators: 'Null',
      cancel_leave_operators: 'Null',
      execute_leave_operators: 'Null',
      operator_bond_more: {
        additionalBond: 'u128',
      },
      schedule_operator_unstake: {
        unstakeAmount: 'u128',
      },
      execute_operator_unstake: 'Null',
      cancel_operator_unstake: 'Null',
      go_offline: 'Null',
      go_online: 'Null',
      deposit: {
        assetId: 'TanglePrimitivesServicesAsset',
        amount: 'u128',
        evmAddress: 'Option<H160>',
        lockMultiplier: 'Option<TanglePrimitivesRewardsLockMultiplier>',
      },
      schedule_withdraw: {
        assetId: 'TanglePrimitivesServicesAsset',
        amount: 'u128',
      },
      execute_withdraw: {
        evmAddress: 'Option<H160>',
      },
      cancel_withdraw: {
        assetId: 'TanglePrimitivesServicesAsset',
        amount: 'u128',
      },
      delegate: {
        operator: 'AccountId32',
        assetId: 'TanglePrimitivesServicesAsset',
        amount: 'u128',
        blueprintSelection: 'PalletMultiAssetDelegationDelegatorDelegatorBlueprintSelection',
      },
      schedule_delegator_unstake: {
        operator: 'AccountId32',
        assetId: 'TanglePrimitivesServicesAsset',
        amount: 'u128',
      },
      execute_delegator_unstake: 'Null',
      cancel_delegator_unstake: {
        operator: 'AccountId32',
        assetId: 'TanglePrimitivesServicesAsset',
        amount: 'u128',
      },
      __Unused18: 'Null',
      __Unused19: 'Null',
      __Unused20: 'Null',
      __Unused21: 'Null',
      add_blueprint_id: {
        blueprintId: 'u64',
      },
      remove_blueprint_id: {
        blueprintId: 'u64'
      }
    }
  },
  /**
   * Lookup390: pallet_multi_asset_delegation::types::delegator::DelegatorBlueprintSelection<tangle_testnet_runtime::MaxDelegatorBlueprints>
   **/
  PalletMultiAssetDelegationDelegatorDelegatorBlueprintSelection: {
    _enum: {
      Fixed: 'Vec<u64>',
      All: 'Null'
    }
  },
  /**
   * Lookup391: tangle_testnet_runtime::MaxDelegatorBlueprints
   **/
  TangleTestnetRuntimeMaxDelegatorBlueprints: 'Null',
  /**
   * Lookup394: pallet_services::module::Call<T>
   **/
  PalletServicesModuleCall: {
    _enum: {
      create_blueprint: {
        blueprint: 'TanglePrimitivesServicesServiceBlueprint',
      },
      pre_register: {
        blueprintId: 'Compact<u64>',
      },
      register: {
        blueprintId: 'Compact<u64>',
        preferences: 'TanglePrimitivesServicesOperatorPreferences',
        registrationArgs: 'Vec<TanglePrimitivesServicesField>',
        value: 'Compact<u128>',
      },
      unregister: {
        blueprintId: 'Compact<u64>',
      },
      update_price_targets: {
        blueprintId: 'Compact<u64>',
        priceTargets: 'TanglePrimitivesServicesPriceTargets',
      },
      request: {
        evmOrigin: 'Option<H160>',
        blueprintId: 'Compact<u64>',
        permittedCallers: 'Vec<AccountId32>',
        operators: 'Vec<AccountId32>',
        requestArgs: 'Vec<TanglePrimitivesServicesField>',
        assets: 'Vec<u128>',
        ttl: 'Compact<u64>',
        paymentAsset: 'TanglePrimitivesServicesAsset',
        value: 'Compact<u128>',
      },
      approve: {
        requestId: 'Compact<u64>',
        restakingPercent: 'Compact<Percent>',
      },
      reject: {
        requestId: 'Compact<u64>',
      },
      terminate: {
        serviceId: 'Compact<u64>',
      },
      call: {
        serviceId: 'Compact<u64>',
        job: 'Compact<u8>',
        args: 'Vec<TanglePrimitivesServicesField>',
      },
      submit_result: {
        serviceId: 'Compact<u64>',
        callId: 'Compact<u64>',
        result: 'Vec<TanglePrimitivesServicesField>',
      },
      slash: {
        offender: 'AccountId32',
        serviceId: 'Compact<u64>',
        percent: 'Compact<Percent>',
      },
      dispute: {
        era: 'Compact<u32>',
        index: 'Compact<u32>',
      },
      update_master_blueprint_service_manager: {
        address: 'H160'
      }
    }
  },
  /**
   * Lookup395: tangle_primitives::services::ServiceBlueprint<C>
   **/
  TanglePrimitivesServicesServiceBlueprint: {
    metadata: 'TanglePrimitivesServicesServiceMetadata',
    jobs: 'Vec<TanglePrimitivesServicesJobDefinition>',
    registrationParams: 'Vec<TanglePrimitivesServicesFieldFieldType>',
    requestParams: 'Vec<TanglePrimitivesServicesFieldFieldType>',
    manager: 'TanglePrimitivesServicesBlueprintServiceManager',
    masterManagerRevision: 'TanglePrimitivesServicesMasterBlueprintServiceManagerRevision',
    gadget: 'TanglePrimitivesServicesGadget'
  },
  /**
   * Lookup396: tangle_primitives::services::ServiceMetadata<C>
   **/
  TanglePrimitivesServicesServiceMetadata: {
    name: 'Bytes',
    description: 'Option<Bytes>',
    author: 'Option<Bytes>',
    category: 'Option<Bytes>',
    codeRepository: 'Option<Bytes>',
    logo: 'Option<Bytes>',
    website: 'Option<Bytes>',
    license: 'Option<Bytes>'
  },
  /**
   * Lookup401: tangle_primitives::services::JobDefinition<C>
   **/
  TanglePrimitivesServicesJobDefinition: {
    metadata: 'TanglePrimitivesServicesJobMetadata',
    params: 'Vec<TanglePrimitivesServicesFieldFieldType>',
    result: 'Vec<TanglePrimitivesServicesFieldFieldType>'
  },
  /**
   * Lookup402: tangle_primitives::services::JobMetadata<C>
   **/
  TanglePrimitivesServicesJobMetadata: {
    name: 'Bytes',
    description: 'Option<Bytes>'
  },
  /**
   * Lookup404: tangle_primitives::services::field::FieldType
   **/
  TanglePrimitivesServicesFieldFieldType: {
    _enum: {
      Void: 'Null',
      Bool: 'Null',
      Uint8: 'Null',
      Int8: 'Null',
      Uint16: 'Null',
      Int16: 'Null',
      Uint32: 'Null',
      Int32: 'Null',
      Uint64: 'Null',
      Int64: 'Null',
      String: 'Null',
      Bytes: 'Null',
      Optional: 'TanglePrimitivesServicesFieldFieldType',
      Array: '(u64,TanglePrimitivesServicesFieldFieldType)',
      List: 'TanglePrimitivesServicesFieldFieldType',
      Struct: '(TanglePrimitivesServicesFieldFieldType,Vec<(TanglePrimitivesServicesFieldFieldType,TanglePrimitivesServicesFieldFieldType)>)',
      __Unused16: 'Null',
      __Unused17: 'Null',
      __Unused18: 'Null',
      __Unused19: 'Null',
      __Unused20: 'Null',
      __Unused21: 'Null',
      __Unused22: 'Null',
      __Unused23: 'Null',
      __Unused24: 'Null',
      __Unused25: 'Null',
      __Unused26: 'Null',
      __Unused27: 'Null',
      __Unused28: 'Null',
      __Unused29: 'Null',
      __Unused30: 'Null',
      __Unused31: 'Null',
      __Unused32: 'Null',
      __Unused33: 'Null',
      __Unused34: 'Null',
      __Unused35: 'Null',
      __Unused36: 'Null',
      __Unused37: 'Null',
      __Unused38: 'Null',
      __Unused39: 'Null',
      __Unused40: 'Null',
      __Unused41: 'Null',
      __Unused42: 'Null',
      __Unused43: 'Null',
      __Unused44: 'Null',
      __Unused45: 'Null',
      __Unused46: 'Null',
      __Unused47: 'Null',
      __Unused48: 'Null',
      __Unused49: 'Null',
      __Unused50: 'Null',
      __Unused51: 'Null',
      __Unused52: 'Null',
      __Unused53: 'Null',
      __Unused54: 'Null',
      __Unused55: 'Null',
      __Unused56: 'Null',
      __Unused57: 'Null',
      __Unused58: 'Null',
      __Unused59: 'Null',
      __Unused60: 'Null',
      __Unused61: 'Null',
      __Unused62: 'Null',
      __Unused63: 'Null',
      __Unused64: 'Null',
      __Unused65: 'Null',
      __Unused66: 'Null',
      __Unused67: 'Null',
      __Unused68: 'Null',
      __Unused69: 'Null',
      __Unused70: 'Null',
      __Unused71: 'Null',
      __Unused72: 'Null',
      __Unused73: 'Null',
      __Unused74: 'Null',
      __Unused75: 'Null',
      __Unused76: 'Null',
      __Unused77: 'Null',
      __Unused78: 'Null',
      __Unused79: 'Null',
      __Unused80: 'Null',
      __Unused81: 'Null',
      __Unused82: 'Null',
      __Unused83: 'Null',
      __Unused84: 'Null',
      __Unused85: 'Null',
      __Unused86: 'Null',
      __Unused87: 'Null',
      __Unused88: 'Null',
      __Unused89: 'Null',
      __Unused90: 'Null',
      __Unused91: 'Null',
      __Unused92: 'Null',
      __Unused93: 'Null',
      __Unused94: 'Null',
      __Unused95: 'Null',
      __Unused96: 'Null',
      __Unused97: 'Null',
      __Unused98: 'Null',
      __Unused99: 'Null',
      AccountId: 'Null'
    }
  },
  /**
   * Lookup410: tangle_primitives::services::BlueprintServiceManager
   **/
  TanglePrimitivesServicesBlueprintServiceManager: {
    _enum: {
      Evm: 'H160'
    }
  },
  /**
   * Lookup411: tangle_primitives::services::MasterBlueprintServiceManagerRevision
   **/
  TanglePrimitivesServicesMasterBlueprintServiceManagerRevision: {
    _enum: {
      Latest: 'Null',
      Specific: 'u32'
    }
  },
  /**
   * Lookup412: tangle_primitives::services::Gadget<C>
   **/
  TanglePrimitivesServicesGadget: {
    _enum: {
      Wasm: 'TanglePrimitivesServicesWasmGadget',
      Native: 'TanglePrimitivesServicesNativeGadget',
      Container: 'TanglePrimitivesServicesContainerGadget'
    }
  },
  /**
   * Lookup413: tangle_primitives::services::WasmGadget<C>
   **/
  TanglePrimitivesServicesWasmGadget: {
    runtime: 'TanglePrimitivesServicesWasmRuntime',
    sources: 'Vec<TanglePrimitivesServicesGadgetSource>'
  },
  /**
   * Lookup414: tangle_primitives::services::WasmRuntime
   **/
  TanglePrimitivesServicesWasmRuntime: {
    _enum: ['Wasmtime', 'Wasmer']
  },
  /**
   * Lookup416: tangle_primitives::services::GadgetSource<C>
   **/
  TanglePrimitivesServicesGadgetSource: {
    fetcher: 'TanglePrimitivesServicesGadgetSourceFetcher'
  },
  /**
   * Lookup417: tangle_primitives::services::GadgetSourceFetcher<C>
   **/
  TanglePrimitivesServicesGadgetSourceFetcher: {
    _enum: {
      IPFS: 'Bytes',
      Github: 'TanglePrimitivesServicesGithubFetcher',
      ContainerImage: 'TanglePrimitivesServicesImageRegistryFetcher',
      Testing: 'TanglePrimitivesServicesTestFetcher'
    }
  },
  /**
   * Lookup419: tangle_primitives::services::GithubFetcher<C>
   **/
  TanglePrimitivesServicesGithubFetcher: {
    owner: 'Bytes',
    repo: 'Bytes',
    tag: 'Bytes',
    binaries: 'Vec<TanglePrimitivesServicesGadgetBinary>'
  },
  /**
   * Lookup427: tangle_primitives::services::GadgetBinary<C>
   **/
  TanglePrimitivesServicesGadgetBinary: {
    arch: 'TanglePrimitivesServicesArchitecture',
    os: 'TanglePrimitivesServicesOperatingSystem',
    name: 'Bytes',
    sha256: '[u8;32]'
  },
  /**
   * Lookup428: tangle_primitives::services::Architecture
   **/
  TanglePrimitivesServicesArchitecture: {
    _enum: ['Wasm', 'Wasm64', 'Wasi', 'Wasi64', 'Amd', 'Amd64', 'Arm', 'Arm64', 'RiscV', 'RiscV64']
  },
  /**
   * Lookup429: tangle_primitives::services::OperatingSystem
   **/
  TanglePrimitivesServicesOperatingSystem: {
    _enum: ['Unknown', 'Linux', 'Windows', 'MacOS', 'BSD']
  },
  /**
   * Lookup433: tangle_primitives::services::ImageRegistryFetcher<C>
   **/
  TanglePrimitivesServicesImageRegistryFetcher: {
    _alias: {
      registry_: 'registry'
    },
    registry_: 'Bytes',
    image: 'Bytes',
    tag: 'Bytes'
  },
  /**
   * Lookup440: tangle_primitives::services::TestFetcher<C>
   **/
  TanglePrimitivesServicesTestFetcher: {
    cargoPackage: 'Bytes',
    cargoBin: 'Bytes',
    basePath: 'Bytes'
  },
  /**
   * Lookup442: tangle_primitives::services::NativeGadget<C>
   **/
  TanglePrimitivesServicesNativeGadget: {
    sources: 'Vec<TanglePrimitivesServicesGadgetSource>'
  },
  /**
   * Lookup443: tangle_primitives::services::ContainerGadget<C>
   **/
  TanglePrimitivesServicesContainerGadget: {
    sources: 'Vec<TanglePrimitivesServicesGadgetSource>'
  },
  /**
   * Lookup446: pallet_tangle_lst::pallet::Call<T>
   **/
  PalletTangleLstCall: {
    _enum: {
      join: {
        amount: 'Compact<u128>',
        poolId: 'u32',
      },
      bond_extra: {
        poolId: 'u32',
        extra: 'PalletTangleLstBondExtra',
      },
      __Unused2: 'Null',
      unbond: {
        memberAccount: 'MultiAddress',
        poolId: 'u32',
        unbondingPoints: 'Compact<u128>',
      },
      pool_withdraw_unbonded: {
        poolId: 'u32',
        numSlashingSpans: 'u32',
      },
      withdraw_unbonded: {
        memberAccount: 'MultiAddress',
        poolId: 'u32',
        numSlashingSpans: 'u32',
      },
      create: {
        amount: 'Compact<u128>',
        root: 'MultiAddress',
        nominator: 'MultiAddress',
        bouncer: 'MultiAddress',
        name: 'Option<Bytes>',
        icon: 'Option<Bytes>',
      },
      create_with_pool_id: {
        amount: 'Compact<u128>',
        root: 'MultiAddress',
        nominator: 'MultiAddress',
        bouncer: 'MultiAddress',
        poolId: 'u32',
        name: 'Option<Bytes>',
        icon: 'Option<Bytes>',
      },
      nominate: {
        poolId: 'u32',
        validators: 'Vec<AccountId32>',
      },
      set_state: {
        poolId: 'u32',
        state: 'PalletTangleLstPoolsPoolState',
      },
      set_metadata: {
        poolId: 'u32',
        metadata: 'Bytes',
      },
      set_configs: {
        minJoinBond: 'PalletTangleLstConfigOpU128',
        minCreateBond: 'PalletTangleLstConfigOpU128',
        maxPools: 'PalletTangleLstConfigOpU32',
        globalMaxCommission: 'PalletTangleLstConfigOpPerbill',
      },
      update_roles: {
        poolId: 'u32',
        newRoot: 'PalletTangleLstConfigOpAccountId32',
        newNominator: 'PalletTangleLstConfigOpAccountId32',
        newBouncer: 'PalletTangleLstConfigOpAccountId32',
      },
      chill: {
        poolId: 'u32',
      },
      bond_extra_other: {
        member: 'MultiAddress',
        poolId: 'u32',
        extra: 'PalletTangleLstBondExtra',
      },
      __Unused15: 'Null',
      __Unused16: 'Null',
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
        changeRate: 'PalletTangleLstCommissionCommissionChangeRate',
      },
      claim_commission: {
        poolId: 'u32',
      },
      adjust_pool_deposit: {
        poolId: 'u32',
      },
      set_commission_claim_permission: {
        poolId: 'u32',
        permission: 'Option<PalletTangleLstCommissionCommissionClaimPermission>',
      },
      set_last_pool_id: {
        poolId: 'u32'
      }
    }
  },
  /**
   * Lookup447: pallet_tangle_lst::types::BondExtra<Balance>
   **/
  PalletTangleLstBondExtra: {
    _enum: {
      FreeBalance: 'u128'
    }
  },
  /**
   * Lookup452: pallet_tangle_lst::types::ConfigOp<T>
   **/
  PalletTangleLstConfigOpU128: {
    _enum: {
      Noop: 'Null',
      Set: 'u128',
      Remove: 'Null'
    }
  },
  /**
   * Lookup453: pallet_tangle_lst::types::ConfigOp<T>
   **/
  PalletTangleLstConfigOpU32: {
    _enum: {
      Noop: 'Null',
      Set: 'u32',
      Remove: 'Null'
    }
  },
  /**
   * Lookup454: pallet_tangle_lst::types::ConfigOp<sp_arithmetic::per_things::Perbill>
   **/
  PalletTangleLstConfigOpPerbill: {
    _enum: {
      Noop: 'Null',
      Set: 'Perbill',
      Remove: 'Null'
    }
  },
  /**
   * Lookup455: pallet_tangle_lst::types::ConfigOp<sp_core::crypto::AccountId32>
   **/
  PalletTangleLstConfigOpAccountId32: {
    _enum: {
      Noop: 'Null',
      Set: 'AccountId32',
      Remove: 'Null'
    }
  },
  /**
   * Lookup456: pallet_rewards::pallet::Call<T>
   **/
  PalletRewardsCall: {
    _enum: {
      __Unused0: 'Null',
      claim_rewards: {
        asset: 'TanglePrimitivesServicesAsset',
      },
      claim_rewards_other: {
        who: 'AccountId32',
        asset: 'TanglePrimitivesServicesAsset',
      },
      manage_asset_reward_vault: {
        vaultId: 'u32',
        assetId: 'TanglePrimitivesServicesAsset',
        action: 'PalletRewardsAssetAction',
      },
      create_reward_vault: {
        vaultId: 'u32',
        newConfig: 'PalletRewardsRewardConfigForAssetVault',
      },
      update_vault_reward_config: {
        vaultId: 'u32',
        newConfig: 'PalletRewardsRewardConfigForAssetVault',
      },
      update_decay_config: {
        startPeriod: 'u64',
        rate: 'Percent',
      },
      update_apy_blocks: {
        blocks: 'u64'
      }
    }
  },
  /**
   * Lookup457: pallet_sudo::pallet::Error<T>
   **/
  PalletSudoError: {
    _enum: ['RequireSudo']
  },
  /**
   * Lookup459: pallet_assets::types::AssetDetails<Balance, sp_core::crypto::AccountId32, DepositBalance>
   **/
  PalletAssetsAssetDetails: {
    owner: 'AccountId32',
    issuer: 'AccountId32',
    admin: 'AccountId32',
    freezer: 'AccountId32',
    supply: 'u128',
    deposit: 'u128',
    minBalance: 'u128',
    isSufficient: 'bool',
    accounts: 'u32',
    sufficients: 'u32',
    approvals: 'u32',
    status: 'PalletAssetsAssetStatus'
  },
  /**
   * Lookup460: pallet_assets::types::AssetStatus
   **/
  PalletAssetsAssetStatus: {
    _enum: ['Live', 'Frozen', 'Destroying']
  },
  /**
   * Lookup462: pallet_assets::types::AssetAccount<Balance, DepositBalance, Extra, sp_core::crypto::AccountId32>
   **/
  PalletAssetsAssetAccount: {
    balance: 'u128',
    status: 'PalletAssetsAccountStatus',
    reason: 'PalletAssetsExistenceReason',
    extra: 'Null'
  },
  /**
   * Lookup463: pallet_assets::types::AccountStatus
   **/
  PalletAssetsAccountStatus: {
    _enum: ['Liquid', 'Frozen', 'Blocked']
  },
  /**
   * Lookup464: pallet_assets::types::ExistenceReason<Balance, sp_core::crypto::AccountId32>
   **/
  PalletAssetsExistenceReason: {
    _enum: {
      Consumer: 'Null',
      Sufficient: 'Null',
      DepositHeld: 'u128',
      DepositRefunded: 'Null',
      DepositFrom: '(AccountId32,u128)'
    }
  },
  /**
   * Lookup466: pallet_assets::types::Approval<Balance, DepositBalance>
   **/
  PalletAssetsApproval: {
    amount: 'u128',
    deposit: 'u128'
  },
  /**
   * Lookup467: pallet_assets::types::AssetMetadata<DepositBalance, bounded_collections::bounded_vec::BoundedVec<T, S>>
   **/
  PalletAssetsAssetMetadata: {
    deposit: 'u128',
    name: 'Bytes',
    symbol: 'Bytes',
    decimals: 'u8',
    isFrozen: 'bool'
  },
  /**
   * Lookup469: pallet_assets::pallet::Error<T, I>
   **/
  PalletAssetsError: {
    _enum: ['BalanceLow', 'NoAccount', 'NoPermission', 'Unknown', 'Frozen', 'InUse', 'BadWitness', 'MinBalanceZero', 'UnavailableConsumer', 'BadMetadata', 'Unapproved', 'WouldDie', 'AlreadyExists', 'NoDeposit', 'WouldBurn', 'LiveAsset', 'AssetNotLive', 'IncorrectStatus', 'NotFrozen', 'CallbackFailed', 'BadAssetId']
  },
  /**
   * Lookup471: pallet_balances::types::BalanceLock<Balance>
   **/
  PalletBalancesBalanceLock: {
    id: '[u8;8]',
    amount: 'u128',
    reasons: 'PalletBalancesReasons'
  },
  /**
   * Lookup472: pallet_balances::types::Reasons
   **/
  PalletBalancesReasons: {
    _enum: ['Fee', 'Misc', 'All']
  },
  /**
   * Lookup475: pallet_balances::types::ReserveData<ReserveIdentifier, Balance>
   **/
  PalletBalancesReserveData: {
    id: '[u8;8]',
    amount: 'u128'
  },
  /**
   * Lookup478: frame_support::traits::tokens::misc::IdAmount<tangle_testnet_runtime::RuntimeHoldReason, Balance>
   **/
  FrameSupportTokensMiscIdAmountRuntimeHoldReason: {
    id: 'TangleTestnetRuntimeRuntimeHoldReason',
    amount: 'u128'
  },
  /**
   * Lookup479: tangle_testnet_runtime::RuntimeHoldReason
   **/
  TangleTestnetRuntimeRuntimeHoldReason: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      __Unused4: 'Null',
      __Unused5: 'Null',
      __Unused6: 'Null',
      __Unused7: 'Null',
      __Unused8: 'Null',
      __Unused9: 'Null',
      __Unused10: 'Null',
      __Unused11: 'Null',
      __Unused12: 'Null',
      __Unused13: 'Null',
      __Unused14: 'Null',
      __Unused15: 'Null',
      __Unused16: 'Null',
      __Unused17: 'Null',
      __Unused18: 'Null',
      __Unused19: 'Null',
      __Unused20: 'Null',
      __Unused21: 'Null',
      __Unused22: 'Null',
      __Unused23: 'Null',
      __Unused24: 'Null',
      __Unused25: 'Null',
      Preimage: 'PalletPreimageHoldReason'
    }
  },
  /**
   * Lookup480: pallet_preimage::pallet::HoldReason
   **/
  PalletPreimageHoldReason: {
    _enum: ['Preimage']
  },
  /**
   * Lookup483: frame_support::traits::tokens::misc::IdAmount<tangle_testnet_runtime::RuntimeFreezeReason, Balance>
   **/
  FrameSupportTokensMiscIdAmountRuntimeFreezeReason: {
    id: 'TangleTestnetRuntimeRuntimeFreezeReason',
    amount: 'u128'
  },
  /**
   * Lookup484: tangle_testnet_runtime::RuntimeFreezeReason
   **/
  TangleTestnetRuntimeRuntimeFreezeReason: {
    _enum: {
      __Unused0: 'Null',
      __Unused1: 'Null',
      __Unused2: 'Null',
      __Unused3: 'Null',
      __Unused4: 'Null',
      __Unused5: 'Null',
      __Unused6: 'Null',
      __Unused7: 'Null',
      __Unused8: 'Null',
      __Unused9: 'Null',
      __Unused10: 'Null',
      __Unused11: 'Null',
      __Unused12: 'Null',
      __Unused13: 'Null',
      __Unused14: 'Null',
      __Unused15: 'Null',
      __Unused16: 'Null',
      __Unused17: 'Null',
      __Unused18: 'Null',
      __Unused19: 'Null',
      __Unused20: 'Null',
      __Unused21: 'Null',
      __Unused22: 'Null',
      __Unused23: 'Null',
      NominationPools: 'PalletNominationPoolsFreezeReason',
      __Unused25: 'Null',
      __Unused26: 'Null',
      __Unused27: 'Null',
      __Unused28: 'Null',
      __Unused29: 'Null',
      __Unused30: 'Null',
      __Unused31: 'Null',
      __Unused32: 'Null',
      __Unused33: 'Null',
      __Unused34: 'Null',
      __Unused35: 'Null',
      __Unused36: 'Null',
      __Unused37: 'Null',
      __Unused38: 'Null',
      __Unused39: 'Null',
      __Unused40: 'Null',
      __Unused41: 'Null',
      __Unused42: 'Null',
      __Unused43: 'Null',
      __Unused44: 'Null',
      __Unused45: 'Null',
      __Unused46: 'Null',
      __Unused47: 'Null',
      __Unused48: 'Null',
      __Unused49: 'Null',
      __Unused50: 'Null',
      __Unused51: 'Null',
      Lst: 'PalletTangleLstFreezeReason'
    }
  },
  /**
   * Lookup485: pallet_nomination_pools::pallet::FreezeReason
   **/
  PalletNominationPoolsFreezeReason: {
    _enum: ['PoolMinBalance']
  },
  /**
   * Lookup486: pallet_tangle_lst::pallet::FreezeReason
   **/
  PalletTangleLstFreezeReason: {
    _enum: ['PoolMinBalance']
  },
  /**
   * Lookup488: pallet_balances::pallet::Error<T, I>
   **/
  PalletBalancesError: {
    _enum: ['VestingBalance', 'LiquidityRestrictions', 'InsufficientBalance', 'ExistentialDeposit', 'Expendability', 'ExistingVestingSchedule', 'DeadAccount', 'TooManyReserves', 'TooManyHolds', 'TooManyFreezes', 'IssuanceDeactivated', 'DeltaZero']
  },
  /**
   * Lookup490: pallet_transaction_payment::Releases
   **/
  PalletTransactionPaymentReleases: {
    _enum: ['V1Ancient', 'V2']
  },
  /**
   * Lookup497: sp_consensus_babe::digests::PreDigest
   **/
  SpConsensusBabeDigestsPreDigest: {
    _enum: {
      __Unused0: 'Null',
      Primary: 'SpConsensusBabeDigestsPrimaryPreDigest',
      SecondaryPlain: 'SpConsensusBabeDigestsSecondaryPlainPreDigest',
      SecondaryVRF: 'SpConsensusBabeDigestsSecondaryVRFPreDigest'
    }
  },
  /**
   * Lookup498: sp_consensus_babe::digests::PrimaryPreDigest
   **/
  SpConsensusBabeDigestsPrimaryPreDigest: {
    authorityIndex: 'u32',
    slot: 'u64',
    vrfSignature: 'SpCoreSr25519VrfVrfSignature'
  },
  /**
   * Lookup499: sp_core::sr25519::vrf::VrfSignature
   **/
  SpCoreSr25519VrfVrfSignature: {
    preOutput: '[u8;32]',
    proof: '[u8;64]'
  },
  /**
   * Lookup500: sp_consensus_babe::digests::SecondaryPlainPreDigest
   **/
  SpConsensusBabeDigestsSecondaryPlainPreDigest: {
    authorityIndex: 'u32',
    slot: 'u64'
  },
  /**
   * Lookup501: sp_consensus_babe::digests::SecondaryVRFPreDigest
   **/
  SpConsensusBabeDigestsSecondaryVRFPreDigest: {
    authorityIndex: 'u32',
    slot: 'u64',
    vrfSignature: 'SpCoreSr25519VrfVrfSignature'
  },
  /**
   * Lookup502: sp_consensus_babe::BabeEpochConfiguration
   **/
  SpConsensusBabeBabeEpochConfiguration: {
    c: '(u64,u64)',
    allowedSlots: 'SpConsensusBabeAllowedSlots'
  },
  /**
   * Lookup504: pallet_babe::pallet::Error<T>
   **/
  PalletBabeError: {
    _enum: ['InvalidEquivocationProof', 'InvalidKeyOwnershipProof', 'DuplicateOffenceReport', 'InvalidConfiguration']
  },
  /**
   * Lookup505: pallet_grandpa::StoredState<N>
   **/
  PalletGrandpaStoredState: {
    _enum: {
      Live: 'Null',
      PendingPause: {
        scheduledAt: 'u64',
        delay: 'u64',
      },
      Paused: 'Null',
      PendingResume: {
        scheduledAt: 'u64',
        delay: 'u64'
      }
    }
  },
  /**
   * Lookup506: pallet_grandpa::StoredPendingChange<N, Limit>
   **/
  PalletGrandpaStoredPendingChange: {
    scheduledAt: 'u64',
    delay: 'u64',
    nextAuthorities: 'Vec<(SpConsensusGrandpaAppPublic,u64)>',
    forced: 'Option<u64>'
  },
  /**
   * Lookup508: pallet_grandpa::pallet::Error<T>
   **/
  PalletGrandpaError: {
    _enum: ['PauseFailed', 'ResumeFailed', 'ChangePending', 'TooSoon', 'InvalidKeyOwnershipProof', 'InvalidEquivocationProof', 'DuplicateOffenceReport']
  },
  /**
   * Lookup510: pallet_indices::pallet::Error<T>
   **/
  PalletIndicesError: {
    _enum: ['NotAssigned', 'NotOwner', 'InUse', 'NotTransfer', 'Permanent']
  },
  /**
   * Lookup515: pallet_democracy::types::ReferendumInfo<BlockNumber, frame_support::traits::preimages::Bounded<tangle_testnet_runtime::RuntimeCall, sp_runtime::traits::BlakeTwo256>, Balance>
   **/
  PalletDemocracyReferendumInfo: {
    _enum: {
      Ongoing: 'PalletDemocracyReferendumStatus',
      Finished: {
        approved: 'bool',
        end: 'u64'
      }
    }
  },
  /**
   * Lookup516: pallet_democracy::types::ReferendumStatus<BlockNumber, frame_support::traits::preimages::Bounded<tangle_testnet_runtime::RuntimeCall, sp_runtime::traits::BlakeTwo256>, Balance>
   **/
  PalletDemocracyReferendumStatus: {
    end: 'u64',
    proposal: 'FrameSupportPreimagesBounded',
    threshold: 'PalletDemocracyVoteThreshold',
    delay: 'u64',
    tally: 'PalletDemocracyTally'
  },
  /**
   * Lookup517: pallet_democracy::types::Tally<Balance>
   **/
  PalletDemocracyTally: {
    ayes: 'u128',
    nays: 'u128',
    turnout: 'u128'
  },
  /**
   * Lookup518: pallet_democracy::vote::Voting<Balance, sp_core::crypto::AccountId32, BlockNumber, MaxVotes>
   **/
  PalletDemocracyVoteVoting: {
    _enum: {
      Direct: {
        votes: 'Vec<(u32,PalletDemocracyVoteAccountVote)>',
        delegations: 'PalletDemocracyDelegations',
        prior: 'PalletDemocracyVotePriorLock',
      },
      Delegating: {
        balance: 'u128',
        target: 'AccountId32',
        conviction: 'PalletDemocracyConviction',
        delegations: 'PalletDemocracyDelegations',
        prior: 'PalletDemocracyVotePriorLock'
      }
    }
  },
  /**
   * Lookup522: pallet_democracy::types::Delegations<Balance>
   **/
  PalletDemocracyDelegations: {
    votes: 'u128',
    capital: 'u128'
  },
  /**
   * Lookup523: pallet_democracy::vote::PriorLock<BlockNumber, Balance>
   **/
  PalletDemocracyVotePriorLock: '(u64,u128)',
  /**
   * Lookup526: pallet_democracy::pallet::Error<T>
   **/
  PalletDemocracyError: {
    _enum: ['ValueLow', 'ProposalMissing', 'AlreadyCanceled', 'DuplicateProposal', 'ProposalBlacklisted', 'NotSimpleMajority', 'InvalidHash', 'NoProposal', 'AlreadyVetoed', 'ReferendumInvalid', 'NoneWaiting', 'NotVoter', 'NoPermission', 'AlreadyDelegating', 'InsufficientFunds', 'NotDelegating', 'VotesExist', 'InstantNotAllowed', 'Nonsense', 'WrongUpperBound', 'MaxVotesReached', 'TooMany', 'VotingPeriodLow', 'PreimageNotExist']
  },
  /**
   * Lookup528: pallet_collective::Votes<sp_core::crypto::AccountId32, BlockNumber>
   **/
  PalletCollectiveVotes: {
    index: 'u32',
    threshold: 'u32',
    ayes: 'Vec<AccountId32>',
    nays: 'Vec<AccountId32>',
    end: 'u64'
  },
  /**
   * Lookup529: pallet_collective::pallet::Error<T, I>
   **/
  PalletCollectiveError: {
    _enum: ['NotMember', 'DuplicateProposal', 'ProposalMissing', 'WrongIndex', 'DuplicateVote', 'AlreadyInitialized', 'TooEarly', 'TooManyProposals', 'WrongProposalWeight', 'WrongProposalLength', 'PrimeAccountNotMember']
  },
  /**
   * Lookup532: pallet_vesting::Releases
   **/
  PalletVestingReleases: {
    _enum: ['V0', 'V1']
  },
  /**
   * Lookup533: pallet_vesting::pallet::Error<T>
   **/
  PalletVestingError: {
    _enum: ['NotVesting', 'AtMaxVestingSchedules', 'AmountLow', 'ScheduleIndexOutOfBounds', 'InvalidScheduleParams']
  },
  /**
   * Lookup535: pallet_elections_phragmen::SeatHolder<sp_core::crypto::AccountId32, Balance>
   **/
  PalletElectionsPhragmenSeatHolder: {
    who: 'AccountId32',
    stake: 'u128',
    deposit: 'u128'
  },
  /**
   * Lookup536: pallet_elections_phragmen::Voter<sp_core::crypto::AccountId32, Balance>
   **/
  PalletElectionsPhragmenVoter: {
    votes: 'Vec<AccountId32>',
    stake: 'u128',
    deposit: 'u128'
  },
  /**
   * Lookup537: pallet_elections_phragmen::pallet::Error<T>
   **/
  PalletElectionsPhragmenError: {
    _enum: ['UnableToVote', 'NoVotes', 'TooManyVotes', 'MaximumVotesExceeded', 'LowBalance', 'UnableToPayBond', 'MustBeVoter', 'DuplicatedCandidate', 'TooManyCandidates', 'MemberSubmit', 'RunnerUpSubmit', 'InsufficientCandidateFunds', 'NotMember', 'InvalidWitnessData', 'InvalidVoteCount', 'InvalidRenouncing', 'InvalidReplacement']
  },
  /**
   * Lookup538: pallet_election_provider_multi_phase::ReadySolution<AccountId, MaxWinners>
   **/
  PalletElectionProviderMultiPhaseReadySolution: {
    supports: 'Vec<(AccountId32,SpNposElectionsSupport)>',
    score: 'SpNposElectionsElectionScore',
    compute: 'PalletElectionProviderMultiPhaseElectionCompute'
  },
  /**
   * Lookup540: pallet_election_provider_multi_phase::RoundSnapshot<sp_core::crypto::AccountId32, DataProvider>
   **/
  PalletElectionProviderMultiPhaseRoundSnapshot: {
    voters: 'Vec<(AccountId32,u64,Vec<AccountId32>)>',
    targets: 'Vec<AccountId32>'
  },
  /**
   * Lookup547: pallet_election_provider_multi_phase::signed::SignedSubmission<sp_core::crypto::AccountId32, Balance, tangle_testnet_runtime::NposSolution16>
   **/
  PalletElectionProviderMultiPhaseSignedSignedSubmission: {
    who: 'AccountId32',
    deposit: 'u128',
    rawSolution: 'PalletElectionProviderMultiPhaseRawSolution',
    callFee: 'u128'
  },
  /**
   * Lookup548: pallet_election_provider_multi_phase::pallet::Error<T>
   **/
  PalletElectionProviderMultiPhaseError: {
    _enum: ['PreDispatchEarlySubmission', 'PreDispatchWrongWinnerCount', 'PreDispatchWeakSubmission', 'SignedQueueFull', 'SignedCannotPayDeposit', 'SignedInvalidWitness', 'SignedTooMuchWeight', 'OcwCallWrongEra', 'MissingSnapshotMetadata', 'InvalidSubmissionIndex', 'CallNotAllowed', 'FallbackFailed', 'BoundNotMet', 'TooManyWinners', 'PreDispatchDifferentRound']
  },
  /**
   * Lookup549: pallet_staking::StakingLedger<T>
   **/
  PalletStakingStakingLedger: {
    stash: 'AccountId32',
    total: 'Compact<u128>',
    active: 'Compact<u128>',
    unlocking: 'Vec<PalletStakingUnlockChunk>',
    legacyClaimedRewards: 'Vec<u32>'
  },
  /**
   * Lookup551: pallet_staking::Nominations<T>
   **/
  PalletStakingNominations: {
    targets: 'Vec<AccountId32>',
    submittedIn: 'u32',
    suppressed: 'bool'
  },
  /**
   * Lookup552: pallet_staking::ActiveEraInfo
   **/
  PalletStakingActiveEraInfo: {
    index: 'u32',
    start: 'Option<u64>'
  },
  /**
   * Lookup554: sp_staking::PagedExposureMetadata<Balance>
   **/
  SpStakingPagedExposureMetadata: {
    total: 'Compact<u128>',
    own: 'Compact<u128>',
    nominatorCount: 'u32',
    pageCount: 'u32'
  },
  /**
   * Lookup556: sp_staking::ExposurePage<sp_core::crypto::AccountId32, Balance>
   **/
  SpStakingExposurePage: {
    pageTotal: 'Compact<u128>',
    others: 'Vec<SpStakingIndividualExposure>'
  },
  /**
   * Lookup557: pallet_staking::EraRewardPoints<sp_core::crypto::AccountId32>
   **/
  PalletStakingEraRewardPoints: {
    total: 'u32',
    individual: 'BTreeMap<AccountId32, u32>'
  },
  /**
   * Lookup562: pallet_staking::UnappliedSlash<sp_core::crypto::AccountId32, Balance>
   **/
  PalletStakingUnappliedSlash: {
    validator: 'AccountId32',
    own: 'u128',
    others: 'Vec<(AccountId32,u128)>',
    reporters: 'Vec<AccountId32>',
    payout: 'u128'
  },
  /**
   * Lookup566: pallet_staking::slashing::SlashingSpans
   **/
  PalletStakingSlashingSlashingSpans: {
    spanIndex: 'u32',
    lastStart: 'u32',
    lastNonzeroSlash: 'u32',
    prior: 'Vec<u32>'
  },
  /**
   * Lookup567: pallet_staking::slashing::SpanRecord<Balance>
   **/
  PalletStakingSlashingSpanRecord: {
    slashed: 'u128',
    paidOut: 'u128'
  },
  /**
   * Lookup568: pallet_staking::pallet::pallet::Error<T>
   **/
  PalletStakingPalletError: {
    _enum: ['NotController', 'NotStash', 'AlreadyBonded', 'AlreadyPaired', 'EmptyTargets', 'DuplicateIndex', 'InvalidSlashIndex', 'InsufficientBond', 'NoMoreChunks', 'NoUnlockChunk', 'FundedTarget', 'InvalidEraToReward', 'InvalidNumberOfNominations', 'NotSortedAndUnique', 'AlreadyClaimed', 'InvalidPage', 'IncorrectHistoryDepth', 'IncorrectSlashingSpans', 'BadState', 'TooManyTargets', 'BadTarget', 'CannotChillOther', 'TooManyNominators', 'TooManyValidators', 'CommissionTooLow', 'BoundNotMet', 'ControllerDeprecated', 'CannotRestoreLedger', 'RewardDestinationRestricted', 'NotEnoughFunds', 'VirtualStakerNotAllowed']
  },
  /**
   * Lookup572: sp_core::crypto::KeyTypeId
   **/
  SpCoreCryptoKeyTypeId: '[u8;4]',
  /**
   * Lookup573: pallet_session::pallet::Error<T>
   **/
  PalletSessionError: {
    _enum: ['InvalidProof', 'NoAssociatedValidatorId', 'DuplicatedKey', 'NoKeys', 'NoAccount']
  },
  /**
   * Lookup575: pallet_treasury::Proposal<sp_core::crypto::AccountId32, Balance>
   **/
  PalletTreasuryProposal: {
    proposer: 'AccountId32',
    value: 'u128',
    beneficiary: 'AccountId32',
    bond: 'u128'
  },
  /**
   * Lookup577: pallet_treasury::SpendStatus<AssetKind, AssetBalance, sp_core::crypto::AccountId32, BlockNumber, PaymentId>
   **/
  PalletTreasurySpendStatus: {
    assetKind: 'Null',
    amount: 'u128',
    beneficiary: 'AccountId32',
    validFrom: 'u64',
    expireAt: 'u64',
    status: 'PalletTreasuryPaymentState'
  },
  /**
   * Lookup578: pallet_treasury::PaymentState<Id>
   **/
  PalletTreasuryPaymentState: {
    _enum: {
      Pending: 'Null',
      Attempted: {
        id: 'Null',
      },
      Failed: 'Null'
    }
  },
  /**
   * Lookup579: frame_support::PalletId
   **/
  FrameSupportPalletId: '[u8;8]',
  /**
   * Lookup580: pallet_treasury::pallet::Error<T, I>
   **/
  PalletTreasuryError: {
    _enum: ['InvalidIndex', 'TooManyApprovals', 'InsufficientPermission', 'ProposalNotApproved', 'FailedToConvertBalance', 'SpendExpired', 'EarlyPayout', 'AlreadyAttempted', 'PayoutError', 'NotAttempted', 'Inconclusive']
  },
  /**
   * Lookup581: pallet_bounties::Bounty<sp_core::crypto::AccountId32, Balance, BlockNumber>
   **/
  PalletBountiesBounty: {
    proposer: 'AccountId32',
    value: 'u128',
    fee: 'u128',
    curatorDeposit: 'u128',
    bond: 'u128',
    status: 'PalletBountiesBountyStatus'
  },
  /**
   * Lookup582: pallet_bounties::BountyStatus<sp_core::crypto::AccountId32, BlockNumber>
   **/
  PalletBountiesBountyStatus: {
    _enum: {
      Proposed: 'Null',
      Approved: 'Null',
      Funded: 'Null',
      CuratorProposed: {
        curator: 'AccountId32',
      },
      Active: {
        curator: 'AccountId32',
        updateDue: 'u64',
      },
      PendingPayout: {
        curator: 'AccountId32',
        beneficiary: 'AccountId32',
        unlockAt: 'u64'
      }
    }
  },
  /**
   * Lookup584: pallet_bounties::pallet::Error<T, I>
   **/
  PalletBountiesError: {
    _enum: ['InsufficientProposersBalance', 'InvalidIndex', 'ReasonTooBig', 'UnexpectedStatus', 'RequireCurator', 'InvalidValue', 'InvalidFee', 'PendingPayout', 'Premature', 'HasActiveChildBounty', 'TooManyQueued']
  },
  /**
   * Lookup585: pallet_child_bounties::ChildBounty<sp_core::crypto::AccountId32, Balance, BlockNumber>
   **/
  PalletChildBountiesChildBounty: {
    parentBounty: 'u32',
    value: 'u128',
    fee: 'u128',
    curatorDeposit: 'u128',
    status: 'PalletChildBountiesChildBountyStatus'
  },
  /**
   * Lookup586: pallet_child_bounties::ChildBountyStatus<sp_core::crypto::AccountId32, BlockNumber>
   **/
  PalletChildBountiesChildBountyStatus: {
    _enum: {
      Added: 'Null',
      CuratorProposed: {
        curator: 'AccountId32',
      },
      Active: {
        curator: 'AccountId32',
      },
      PendingPayout: {
        curator: 'AccountId32',
        beneficiary: 'AccountId32',
        unlockAt: 'u64'
      }
    }
  },
  /**
   * Lookup587: pallet_child_bounties::pallet::Error<T>
   **/
  PalletChildBountiesError: {
    _enum: ['ParentBountyNotActive', 'InsufficientBountyBalance', 'TooManyChildBounties']
  },
  /**
   * Lookup588: pallet_bags_list::list::Node<T, I>
   **/
  PalletBagsListListNode: {
    id: 'AccountId32',
    prev: 'Option<AccountId32>',
    next: 'Option<AccountId32>',
    bagUpper: 'u64',
    score: 'u64'
  },
  /**
   * Lookup589: pallet_bags_list::list::Bag<T, I>
   **/
  PalletBagsListListBag: {
    head: 'Option<AccountId32>',
    tail: 'Option<AccountId32>'
  },
  /**
   * Lookup590: pallet_bags_list::pallet::Error<T, I>
   **/
  PalletBagsListError: {
    _enum: {
      List: 'PalletBagsListListListError'
    }
  },
  /**
   * Lookup591: pallet_bags_list::list::ListError
   **/
  PalletBagsListListListError: {
    _enum: ['Duplicate', 'NotHeavier', 'NotInSameBag', 'NodeNotFound']
  },
  /**
   * Lookup592: pallet_nomination_pools::PoolMember<T>
   **/
  PalletNominationPoolsPoolMember: {
    poolId: 'u32',
    points: 'u128',
    lastRecordedRewardCounter: 'u128',
    unbondingEras: 'BTreeMap<u32, u128>'
  },
  /**
   * Lookup597: pallet_nomination_pools::BondedPoolInner<T>
   **/
  PalletNominationPoolsBondedPoolInner: {
    commission: 'PalletNominationPoolsCommission',
    memberCounter: 'u32',
    points: 'u128',
    roles: 'PalletNominationPoolsPoolRoles',
    state: 'PalletNominationPoolsPoolState'
  },
  /**
   * Lookup598: pallet_nomination_pools::Commission<T>
   **/
  PalletNominationPoolsCommission: {
    current: 'Option<(Perbill,AccountId32)>',
    max: 'Option<Perbill>',
    changeRate: 'Option<PalletNominationPoolsCommissionChangeRate>',
    throttleFrom: 'Option<u64>',
    claimPermission: 'Option<PalletNominationPoolsCommissionClaimPermission>'
  },
  /**
   * Lookup601: pallet_nomination_pools::PoolRoles<sp_core::crypto::AccountId32>
   **/
  PalletNominationPoolsPoolRoles: {
    depositor: 'AccountId32',
    root: 'Option<AccountId32>',
    nominator: 'Option<AccountId32>',
    bouncer: 'Option<AccountId32>'
  },
  /**
   * Lookup602: pallet_nomination_pools::RewardPool<T>
   **/
  PalletNominationPoolsRewardPool: {
    lastRecordedRewardCounter: 'u128',
    lastRecordedTotalPayouts: 'u128',
    totalRewardsClaimed: 'u128',
    totalCommissionPending: 'u128',
    totalCommissionClaimed: 'u128'
  },
  /**
   * Lookup603: pallet_nomination_pools::SubPools<T>
   **/
  PalletNominationPoolsSubPools: {
    noEra: 'PalletNominationPoolsUnbondPool',
    withEra: 'BTreeMap<u32, PalletNominationPoolsUnbondPool>'
  },
  /**
   * Lookup604: pallet_nomination_pools::UnbondPool<T>
   **/
  PalletNominationPoolsUnbondPool: {
    points: 'u128',
    balance: 'u128'
  },
  /**
   * Lookup609: pallet_nomination_pools::pallet::Error<T>
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
      CommissionExceedsGlobalMaximum: 'Null',
      CommissionChangeThrottled: 'Null',
      CommissionChangeRateNotAllowed: 'Null',
      NoPendingCommission: 'Null',
      NoCommissionCurrentSet: 'Null',
      PoolIdInUse: 'Null',
      InvalidPoolId: 'Null',
      BondExtraRestricted: 'Null',
      NothingToAdjust: 'Null',
      NothingToSlash: 'Null',
      SlashTooLow: 'Null',
      AlreadyMigrated: 'Null',
      NotMigrated: 'Null',
      NotSupported: 'Null'
    }
  },
  /**
   * Lookup610: pallet_nomination_pools::pallet::DefensiveError
   **/
  PalletNominationPoolsDefensiveError: {
    _enum: ['NotEnoughSpaceInUnbondPool', 'PoolNotFound', 'RewardPoolNotFound', 'SubPoolsNotFound', 'BondedStashKilledPrematurely', 'DelegationUnsupported', 'SlashNotApplied']
  },
  /**
   * Lookup613: pallet_scheduler::Scheduled<Name, frame_support::traits::preimages::Bounded<tangle_testnet_runtime::RuntimeCall, sp_runtime::traits::BlakeTwo256>, BlockNumber, tangle_testnet_runtime::OriginCaller, sp_core::crypto::AccountId32>
   **/
  PalletSchedulerScheduled: {
    maybeId: 'Option<[u8;32]>',
    priority: 'u8',
    call: 'FrameSupportPreimagesBounded',
    maybePeriodic: 'Option<(u64,u32)>',
    origin: 'TangleTestnetRuntimeOriginCaller'
  },
  /**
   * Lookup615: pallet_scheduler::RetryConfig<Period>
   **/
  PalletSchedulerRetryConfig: {
    totalRetries: 'u8',
    remaining: 'u8',
    period: 'u64'
  },
  /**
   * Lookup616: pallet_scheduler::pallet::Error<T>
   **/
  PalletSchedulerError: {
    _enum: ['FailedToSchedule', 'NotFound', 'TargetBlockNumberInPast', 'RescheduleNoChange', 'Named']
  },
  /**
   * Lookup617: pallet_preimage::OldRequestStatus<sp_core::crypto::AccountId32, Balance>
   **/
  PalletPreimageOldRequestStatus: {
    _enum: {
      Unrequested: {
        deposit: '(AccountId32,u128)',
        len: 'u32',
      },
      Requested: {
        deposit: 'Option<(AccountId32,u128)>',
        count: 'u32',
        len: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup619: pallet_preimage::RequestStatus<sp_core::crypto::AccountId32, Ticket>
   **/
  PalletPreimageRequestStatus: {
    _enum: {
      Unrequested: {
        ticket: '(AccountId32,Null)',
        len: 'u32',
      },
      Requested: {
        maybeTicket: 'Option<(AccountId32,Null)>',
        count: 'u32',
        maybeLen: 'Option<u32>'
      }
    }
  },
  /**
   * Lookup623: pallet_preimage::pallet::Error<T>
   **/
  PalletPreimageError: {
    _enum: ['TooBig', 'AlreadyNoted', 'NotAuthorized', 'NotNoted', 'Requested', 'NotRequested', 'TooMany', 'TooFew', 'NoCost']
  },
  /**
   * Lookup624: sp_staking::offence::OffenceDetails<sp_core::crypto::AccountId32, Offender>
   **/
  SpStakingOffenceOffenceDetails: {
    offender: '(AccountId32,SpStakingExposure)',
    reporters: 'Vec<AccountId32>'
  },
  /**
   * Lookup626: pallet_tx_pause::pallet::Error<T>
   **/
  PalletTxPauseError: {
    _enum: ['IsPaused', 'IsUnpaused', 'Unpausable', 'NotFound']
  },
  /**
   * Lookup629: pallet_im_online::pallet::Error<T>
   **/
  PalletImOnlineError: {
    _enum: ['InvalidKey', 'DuplicatedHeartbeat']
  },
  /**
   * Lookup631: pallet_identity::types::Registration<Balance, MaxJudgements, pallet_identity::legacy::IdentityInfo<FieldLimit>>
   **/
  PalletIdentityRegistration: {
    judgements: 'Vec<(u32,PalletIdentityJudgement)>',
    deposit: 'u128',
    info: 'PalletIdentityLegacyIdentityInfo'
  },
  /**
   * Lookup640: pallet_identity::types::RegistrarInfo<Balance, sp_core::crypto::AccountId32, IdField>
   **/
  PalletIdentityRegistrarInfo: {
    account: 'AccountId32',
    fee: 'u128',
    fields: 'u64'
  },
  /**
   * Lookup642: pallet_identity::types::AuthorityProperties<bounded_collections::bounded_vec::BoundedVec<T, S>>
   **/
  PalletIdentityAuthorityProperties: {
    suffix: 'Bytes',
    allocation: 'u32'
  },
  /**
   * Lookup645: pallet_identity::pallet::Error<T>
   **/
  PalletIdentityError: {
    _enum: ['TooManySubAccounts', 'NotFound', 'NotNamed', 'EmptyIndex', 'FeeChanged', 'NoIdentity', 'StickyJudgement', 'JudgementGiven', 'InvalidJudgement', 'InvalidIndex', 'InvalidTarget', 'TooManyRegistrars', 'AlreadyClaimed', 'NotSub', 'NotOwned', 'JudgementForDifferentIdentity', 'JudgementPaymentFailed', 'InvalidSuffix', 'NotUsernameAuthority', 'NoAllocation', 'InvalidSignature', 'RequiresSignature', 'InvalidUsername', 'UsernameTaken', 'NoUsername', 'NotExpired']
  },
  /**
   * Lookup646: pallet_utility::pallet::Error<T>
   **/
  PalletUtilityError: {
    _enum: ['TooManyCalls']
  },
  /**
   * Lookup648: pallet_multisig::Multisig<BlockNumber, Balance, sp_core::crypto::AccountId32, MaxApprovals>
   **/
  PalletMultisigMultisig: {
    when: 'PalletMultisigTimepoint',
    deposit: 'u128',
    depositor: 'AccountId32',
    approvals: 'Vec<AccountId32>'
  },
  /**
   * Lookup649: pallet_multisig::pallet::Error<T>
   **/
  PalletMultisigError: {
    _enum: ['MinimumThreshold', 'AlreadyApproved', 'NoApprovalsNeeded', 'TooFewSignatories', 'TooManySignatories', 'SignatoriesOutOfOrder', 'SenderInSignatories', 'NotFound', 'NotOwner', 'NoTimepoint', 'WrongTimepoint', 'UnexpectedTimepoint', 'MaxWeightTooLow', 'AlreadyStored']
  },
  /**
   * Lookup652: fp_rpc::TransactionStatus
   **/
  FpRpcTransactionStatus: {
    transactionHash: 'H256',
    transactionIndex: 'u32',
    from: 'H160',
    to: 'Option<H160>',
    contractAddress: 'Option<H160>',
    logs: 'Vec<EthereumLog>',
    logsBloom: 'EthbloomBloom'
  },
  /**
   * Lookup654: ethbloom::Bloom
   **/
  EthbloomBloom: '[u8;256]',
  /**
   * Lookup656: ethereum::receipt::ReceiptV3
   **/
  EthereumReceiptReceiptV3: {
    _enum: {
      Legacy: 'EthereumReceiptEip658ReceiptData',
      EIP2930: 'EthereumReceiptEip658ReceiptData',
      EIP1559: 'EthereumReceiptEip658ReceiptData'
    }
  },
  /**
   * Lookup657: ethereum::receipt::EIP658ReceiptData
   **/
  EthereumReceiptEip658ReceiptData: {
    statusCode: 'u8',
    usedGas: 'U256',
    logsBloom: 'EthbloomBloom',
    logs: 'Vec<EthereumLog>'
  },
  /**
   * Lookup658: ethereum::block::Block<ethereum::transaction::TransactionV2>
   **/
  EthereumBlock: {
    header: 'EthereumHeader',
    transactions: 'Vec<EthereumTransactionTransactionV2>',
    ommers: 'Vec<EthereumHeader>'
  },
  /**
   * Lookup659: ethereum::header::Header
   **/
  EthereumHeader: {
    parentHash: 'H256',
    ommersHash: 'H256',
    beneficiary: 'H160',
    stateRoot: 'H256',
    transactionsRoot: 'H256',
    receiptsRoot: 'H256',
    logsBloom: 'EthbloomBloom',
    difficulty: 'U256',
    number: 'U256',
    gasLimit: 'U256',
    gasUsed: 'U256',
    timestamp: 'u64',
    extraData: 'Bytes',
    mixHash: 'H256',
    nonce: 'EthereumTypesHashH64'
  },
  /**
   * Lookup660: ethereum_types::hash::H64
   **/
  EthereumTypesHashH64: '[u8;8]',
  /**
   * Lookup665: pallet_ethereum::pallet::Error<T>
   **/
  PalletEthereumError: {
    _enum: ['InvalidSignature', 'PreLogExists']
  },
  /**
   * Lookup666: pallet_evm::CodeMetadata
   **/
  PalletEvmCodeMetadata: {
    _alias: {
      size_: 'size',
      hash_: 'hash'
    },
    size_: 'u64',
    hash_: 'H256'
  },
  /**
   * Lookup668: pallet_evm::pallet::Error<T>
   **/
  PalletEvmError: {
    _enum: ['BalanceLow', 'FeeOverflow', 'PaymentOverflow', 'WithdrawFailed', 'GasPriceTooLow', 'InvalidNonce', 'GasLimitTooLow', 'GasLimitTooHigh', 'InvalidChainId', 'InvalidSignature', 'Reentrancy', 'TransactionMustComeFromEOA', 'Undefined']
  },
  /**
   * Lookup669: pallet_hotfix_sufficients::pallet::Error<T>
   **/
  PalletHotfixSufficientsError: {
    _enum: ['MaxAddressCountExceeded']
  },
  /**
   * Lookup671: pallet_airdrop_claims::pallet::Error<T>
   **/
  PalletAirdropClaimsError: {
    _enum: ['InvalidEthereumSignature', 'InvalidNativeSignature', 'InvalidNativeAccount', 'SignerHasNoClaim', 'SenderHasNoClaim', 'PotUnderflow', 'InvalidStatement', 'VestedBalanceExists']
  },
  /**
   * Lookup674: pallet_proxy::ProxyDefinition<sp_core::crypto::AccountId32, tangle_testnet_runtime::ProxyType, BlockNumber>
   **/
  PalletProxyProxyDefinition: {
    delegate: 'AccountId32',
    proxyType: 'TangleTestnetRuntimeProxyType',
    delay: 'u64'
  },
  /**
   * Lookup678: pallet_proxy::Announcement<sp_core::crypto::AccountId32, primitive_types::H256, BlockNumber>
   **/
  PalletProxyAnnouncement: {
    real: 'AccountId32',
    callHash: 'H256',
    height: 'u64'
  },
  /**
   * Lookup680: pallet_proxy::pallet::Error<T>
   **/
  PalletProxyError: {
    _enum: ['TooMany', 'NotFound', 'NotProxy', 'Unproxyable', 'Duplicate', 'NoPermission', 'Unannounced', 'NoSelfProxy']
  },
  /**
   * Lookup681: pallet_multi_asset_delegation::types::operator::OperatorMetadata<sp_core::crypto::AccountId32, Balance, AssetId, tangle_testnet_runtime::MaxDelegations, tangle_testnet_runtime::MaxOperatorBlueprints>
   **/
  PalletMultiAssetDelegationOperatorOperatorMetadata: {
    stake: 'u128',
    delegationCount: 'u32',
    request: 'Option<PalletMultiAssetDelegationOperatorOperatorBondLessRequest>',
    delegations: 'Vec<PalletMultiAssetDelegationOperatorDelegatorBond>',
    status: 'PalletMultiAssetDelegationOperatorOperatorStatus',
    blueprintIds: 'Vec<u32>'
  },
  /**
   * Lookup682: tangle_testnet_runtime::MaxDelegations
   **/
  TangleTestnetRuntimeMaxDelegations: 'Null',
  /**
   * Lookup683: tangle_testnet_runtime::MaxOperatorBlueprints
   **/
  TangleTestnetRuntimeMaxOperatorBlueprints: 'Null',
  /**
   * Lookup685: pallet_multi_asset_delegation::types::operator::OperatorBondLessRequest<Balance>
   **/
  PalletMultiAssetDelegationOperatorOperatorBondLessRequest: {
    amount: 'u128',
    requestTime: 'u32'
  },
  /**
   * Lookup687: pallet_multi_asset_delegation::types::operator::DelegatorBond<sp_core::crypto::AccountId32, Balance, AssetId>
   **/
  PalletMultiAssetDelegationOperatorDelegatorBond: {
    delegator: 'AccountId32',
    amount: 'u128',
    assetId: 'TanglePrimitivesServicesAsset'
  },
  /**
   * Lookup689: pallet_multi_asset_delegation::types::operator::OperatorStatus
   **/
  PalletMultiAssetDelegationOperatorOperatorStatus: {
    _enum: {
      Active: 'Null',
      Inactive: 'Null',
      Leaving: 'u32'
    }
  },
  /**
   * Lookup691: pallet_multi_asset_delegation::types::operator::OperatorSnapshot<sp_core::crypto::AccountId32, Balance, AssetId, tangle_testnet_runtime::MaxDelegations>
   **/
  PalletMultiAssetDelegationOperatorOperatorSnapshot: {
    stake: 'u128',
    delegations: 'Vec<PalletMultiAssetDelegationOperatorDelegatorBond>'
  },
  /**
   * Lookup692: pallet_multi_asset_delegation::types::delegator::DelegatorMetadata<sp_core::crypto::AccountId32, Balance, AssetId, tangle_testnet_runtime::MaxWithdrawRequests, tangle_testnet_runtime::MaxDelegations, tangle_testnet_runtime::MaxUnstakeRequests, tangle_testnet_runtime::MaxDelegatorBlueprints, BlockNumber, tangle_testnet_runtime::MaxDelegations>
   **/
  PalletMultiAssetDelegationDelegatorDelegatorMetadata: {
    deposits: 'BTreeMap<TanglePrimitivesServicesAsset, PalletMultiAssetDelegationDelegatorDeposit>',
    withdrawRequests: 'Vec<PalletMultiAssetDelegationDelegatorWithdrawRequest>',
    delegations: 'Vec<PalletMultiAssetDelegationDelegatorBondInfoDelegator>',
    delegatorUnstakeRequests: 'Vec<PalletMultiAssetDelegationDelegatorBondLessRequest>',
    status: 'PalletMultiAssetDelegationDelegatorDelegatorStatus'
  },
  /**
   * Lookup693: tangle_testnet_runtime::MaxWithdrawRequests
   **/
  TangleTestnetRuntimeMaxWithdrawRequests: 'Null',
  /**
   * Lookup694: tangle_testnet_runtime::MaxUnstakeRequests
   **/
  TangleTestnetRuntimeMaxUnstakeRequests: 'Null',
  /**
   * Lookup696: pallet_multi_asset_delegation::types::delegator::Deposit<Balance, BlockNumber, tangle_testnet_runtime::MaxDelegations>
   **/
  PalletMultiAssetDelegationDelegatorDeposit: {
    amount: 'u128',
    delegatedAmount: 'u128',
    locks: 'Option<Vec<TanglePrimitivesRewardsLockInfo>>'
  },
  /**
   * Lookup699: tangle_primitives::types::rewards::LockInfo<Balance, BlockNumber>
   **/
  TanglePrimitivesRewardsLockInfo: {
    amount: 'u128',
    lockMultiplier: 'TanglePrimitivesRewardsLockMultiplier',
    expiryBlock: 'u64'
  },
  /**
   * Lookup704: pallet_multi_asset_delegation::types::delegator::WithdrawRequest<AssetId, Balance>
   **/
  PalletMultiAssetDelegationDelegatorWithdrawRequest: {
    assetId: 'TanglePrimitivesServicesAsset',
    amount: 'u128',
    requestedRound: 'u32'
  },
  /**
   * Lookup707: pallet_multi_asset_delegation::types::delegator::BondInfoDelegator<sp_core::crypto::AccountId32, Balance, AssetId, tangle_testnet_runtime::MaxDelegatorBlueprints>
   **/
  PalletMultiAssetDelegationDelegatorBondInfoDelegator: {
    operator: 'AccountId32',
    amount: 'u128',
    assetId: 'TanglePrimitivesServicesAsset',
    blueprintSelection: 'PalletMultiAssetDelegationDelegatorDelegatorBlueprintSelection'
  },
  /**
   * Lookup710: pallet_multi_asset_delegation::types::delegator::BondLessRequest<sp_core::crypto::AccountId32, AssetId, Balance, tangle_testnet_runtime::MaxDelegatorBlueprints>
   **/
  PalletMultiAssetDelegationDelegatorBondLessRequest: {
    operator: 'AccountId32',
    assetId: 'TanglePrimitivesServicesAsset',
    amount: 'u128',
    requestedRound: 'u32',
    blueprintSelection: 'PalletMultiAssetDelegationDelegatorDelegatorBlueprintSelection'
  },
  /**
   * Lookup712: pallet_multi_asset_delegation::types::delegator::DelegatorStatus
   **/
  PalletMultiAssetDelegationDelegatorDelegatorStatus: {
    _enum: {
      Active: 'Null',
      LeavingScheduled: 'u32'
    }
  },
  /**
   * Lookup713: pallet_multi_asset_delegation::pallet::Error<T>
   **/
  PalletMultiAssetDelegationError: {
    _enum: ['AlreadyOperator', 'BondTooLow', 'NotAnOperator', 'CannotExit', 'AlreadyLeaving', 'NotLeavingOperator', 'LeavingRoundNotReached', 'NoScheduledBondLess', 'BondLessRequestNotSatisfied', 'NotActiveOperator', 'NotOfflineOperator', 'AlreadyDelegator', 'NotDelegator', 'WithdrawRequestAlreadyExists', 'InsufficientBalance', 'NoWithdrawRequest', 'NoBondLessRequest', 'BondLessNotReady', 'BondLessRequestAlreadyExists', 'ActiveServicesUsingAsset', 'NoActiveDelegation', 'AssetNotWhitelisted', 'NotAuthorized', 'MaxBlueprintsExceeded', 'AssetNotFound', 'BlueprintAlreadyWhitelisted', 'NowithdrawRequests', 'NoMatchingwithdrawRequest', 'AssetAlreadyInVault', 'AssetNotInVault', 'VaultNotFound', 'DuplicateBlueprintId', 'BlueprintIdNotFound', 'NotInFixedMode', 'MaxDelegationsExceeded', 'MaxUnstakeRequestsExceeded', 'MaxWithdrawRequestsExceeded', 'DepositOverflow', 'UnstakeAmountTooLarge', 'StakeOverflow', 'InsufficientStakeRemaining', 'APYExceedsMaximum', 'CapCannotBeZero', 'CapExceedsTotalSupply', 'PendingUnstakeRequestExists', 'BlueprintNotSelected', 'ERC20TransferFailed', 'EVMAbiEncode', 'EVMAbiDecode', 'LockViolation', 'DepositExceedsCapForAsset', 'OverflowRisk']
  },
  /**
   * Lookup716: tangle_primitives::services::ServiceRequest<C, sp_core::crypto::AccountId32, BlockNumber, AssetId>
   **/
  TanglePrimitivesServicesServiceRequest: {
    blueprint: 'u64',
    owner: 'AccountId32',
    permittedCallers: 'Vec<AccountId32>',
    assets: 'Vec<u128>',
    ttl: 'u64',
    args: 'Vec<TanglePrimitivesServicesField>',
    operatorsWithApprovalState: 'Vec<(AccountId32,TanglePrimitivesServicesApprovalState)>'
  },
  /**
   * Lookup722: tangle_primitives::services::ApprovalState
   **/
  TanglePrimitivesServicesApprovalState: {
    _enum: {
      Pending: 'Null',
      Approved: {
        restakingPercent: 'Percent',
      },
      Rejected: 'Null'
    }
  },
  /**
   * Lookup724: tangle_primitives::services::Service<C, sp_core::crypto::AccountId32, BlockNumber, AssetId>
   **/
  TanglePrimitivesServicesService: {
    id: 'u64',
    blueprint: 'u64',
    owner: 'AccountId32',
    permittedCallers: 'Vec<AccountId32>',
    operators: 'Vec<(AccountId32,Percent)>',
    assets: 'Vec<u128>',
    ttl: 'u64'
  },
  /**
   * Lookup730: tangle_primitives::services::JobCall<C, sp_core::crypto::AccountId32>
   **/
  TanglePrimitivesServicesJobCall: {
    serviceId: 'u64',
    job: 'u8',
    args: 'Vec<TanglePrimitivesServicesField>'
  },
  /**
   * Lookup731: tangle_primitives::services::JobCallResult<C, sp_core::crypto::AccountId32>
   **/
  TanglePrimitivesServicesJobCallResult: {
    serviceId: 'u64',
    callId: 'u64',
    result: 'Vec<TanglePrimitivesServicesField>'
  },
  /**
   * Lookup732: pallet_services::types::UnappliedSlash<sp_core::crypto::AccountId32, Balance>
   **/
  PalletServicesUnappliedSlash: {
    serviceId: 'u64',
    operator: 'AccountId32',
    own: 'u128',
    others: 'Vec<(AccountId32,u128)>',
    reporters: 'Vec<AccountId32>',
    payout: 'u128'
  },
  /**
   * Lookup734: tangle_primitives::services::OperatorProfile<C>
   **/
  TanglePrimitivesServicesOperatorProfile: {
    services: 'BTreeSet<u64>',
    blueprints: 'BTreeSet<u64>'
  },
  /**
   * Lookup737: tangle_primitives::services::StagingServicePayment<sp_core::crypto::AccountId32, AssetId, Balance>
   **/
  TanglePrimitivesServicesStagingServicePayment: {
    requestId: 'u64',
    refundTo: 'TanglePrimitivesAccount',
    asset: 'TanglePrimitivesServicesAsset',
    amount: 'u128'
  },
  /**
   * Lookup738: tangle_primitives::types::Account<sp_core::crypto::AccountId32>
   **/
  TanglePrimitivesAccount: {
    _enum: {
      Id: 'AccountId32',
      Address: 'H160'
    }
  },
  /**
   * Lookup739: pallet_services::module::Error<T>
   **/
  PalletServicesModuleError: {
    _enum: {
      BlueprintNotFound: 'Null',
      BlueprintCreationInterrupted: 'Null',
      AlreadyRegistered: 'Null',
      InvalidRegistrationInput: 'Null',
      NotAllowedToUnregister: 'Null',
      NotAllowedToUpdatePriceTargets: 'Null',
      InvalidRequestInput: 'Null',
      InvalidJobCallInput: 'Null',
      InvalidJobResult: 'Null',
      NotRegistered: 'Null',
      ApprovalInterrupted: 'Null',
      RejectionInterrupted: 'Null',
      ServiceRequestNotFound: 'Null',
      ServiceInitializationInterrupted: 'Null',
      ServiceNotFound: 'Null',
      TerminationInterrupted: 'Null',
      TypeCheck: 'TanglePrimitivesServicesTypeCheckError',
      MaxPermittedCallersExceeded: 'Null',
      MaxServiceProvidersExceeded: 'Null',
      MaxServicesPerUserExceeded: 'Null',
      MaxFieldsExceeded: 'Null',
      ApprovalNotRequested: 'Null',
      JobDefinitionNotFound: 'Null',
      ServiceOrJobCallNotFound: 'Null',
      JobCallResultNotFound: 'Null',
      EVMAbiEncode: 'Null',
      EVMAbiDecode: 'Null',
      OperatorProfileNotFound: 'Null',
      MaxServicesPerProviderExceeded: 'Null',
      OperatorNotActive: 'Null',
      NoAssetsProvided: 'Null',
      MaxAssetsPerServiceExceeded: 'Null',
      OffenderNotOperator: 'Null',
      OffenderNotActiveOperator: 'Null',
      NoSlashingOrigin: 'Null',
      NoDisputeOrigin: 'Null',
      UnappliedSlashNotFound: 'Null',
      MasterBlueprintServiceManagerRevisionNotFound: 'Null',
      MaxMasterBlueprintServiceManagerVersionsExceeded: 'Null',
      ERC20TransferFailed: 'Null',
      MissingEVMOrigin: 'Null',
      ExpectedEVMAddress: 'Null',
      ExpectedAccountId: 'Null'
    }
  },
  /**
   * Lookup740: tangle_primitives::services::TypeCheckError
   **/
  TanglePrimitivesServicesTypeCheckError: {
    _enum: {
      ArgumentTypeMismatch: {
        index: 'u8',
        expected: 'TanglePrimitivesServicesFieldFieldType',
        actual: 'TanglePrimitivesServicesFieldFieldType',
      },
      NotEnoughArguments: {
        expected: 'u8',
        actual: 'u8',
      },
      ResultTypeMismatch: {
        index: 'u8',
        expected: 'TanglePrimitivesServicesFieldFieldType',
        actual: 'TanglePrimitivesServicesFieldFieldType'
      }
    }
  },
  /**
   * Lookup741: pallet_tangle_lst::types::bonded_pool::BondedPoolInner<T>
   **/
  PalletTangleLstBondedPoolBondedPoolInner: {
    commission: 'PalletTangleLstCommission',
    roles: 'PalletTangleLstPoolsPoolRoles',
    state: 'PalletTangleLstPoolsPoolState',
    metadata: 'PalletTangleLstBondedPoolPoolMetadata'
  },
  /**
   * Lookup742: pallet_tangle_lst::types::commission::Commission<T>
   **/
  PalletTangleLstCommission: {
    current: 'Option<(Perbill,AccountId32)>',
    max: 'Option<Perbill>',
    changeRate: 'Option<PalletTangleLstCommissionCommissionChangeRate>',
    throttleFrom: 'Option<u64>',
    claimPermission: 'Option<PalletTangleLstCommissionCommissionClaimPermission>'
  },
  /**
   * Lookup744: pallet_tangle_lst::types::pools::PoolRoles<sp_core::crypto::AccountId32>
   **/
  PalletTangleLstPoolsPoolRoles: {
    depositor: 'AccountId32',
    root: 'Option<AccountId32>',
    nominator: 'Option<AccountId32>',
    bouncer: 'Option<AccountId32>'
  },
  /**
   * Lookup745: pallet_tangle_lst::types::bonded_pool::PoolMetadata<T>
   **/
  PalletTangleLstBondedPoolPoolMetadata: {
    name: 'Option<Bytes>',
    icon: 'Option<Bytes>'
  },
  /**
   * Lookup746: pallet_tangle_lst::types::sub_pools::RewardPool<T>
   **/
  PalletTangleLstSubPoolsRewardPool: {
    lastRecordedRewardCounter: 'u128',
    lastRecordedTotalPayouts: 'u128',
    totalRewardsClaimed: 'u128',
    totalCommissionPending: 'u128',
    totalCommissionClaimed: 'u128'
  },
  /**
   * Lookup747: pallet_tangle_lst::types::sub_pools::SubPools<T>
   **/
  PalletTangleLstSubPools: {
    noEra: 'PalletTangleLstSubPoolsUnbondPool',
    withEra: 'BTreeMap<u32, PalletTangleLstSubPoolsUnbondPool>'
  },
  /**
   * Lookup748: pallet_tangle_lst::types::sub_pools::UnbondPool<T>
   **/
  PalletTangleLstSubPoolsUnbondPool: {
    points: 'u128',
    balance: 'u128'
  },
  /**
   * Lookup754: pallet_tangle_lst::types::pools::PoolMember<T>
   **/
  PalletTangleLstPoolsPoolMember: {
    unbondingEras: 'BTreeMap<u32, (u32,u128)>'
  },
  /**
   * Lookup759: pallet_tangle_lst::types::ClaimPermission
   **/
  PalletTangleLstClaimPermission: {
    _enum: ['Permissioned', 'PermissionlessCompound', 'PermissionlessWithdraw', 'PermissionlessAll']
  },
  /**
   * Lookup760: pallet_tangle_lst::pallet::Error<T>
   **/
  PalletTangleLstError: {
    _enum: {
      PoolNotFound: 'Null',
      PoolMemberNotFound: 'Null',
      RewardPoolNotFound: 'Null',
      SubPoolsNotFound: 'Null',
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
      Defensive: 'PalletTangleLstDefensiveError',
      PartialUnbondNotAllowedPermissionlessly: 'Null',
      MaxCommissionRestricted: 'Null',
      CommissionExceedsMaximum: 'Null',
      CommissionExceedsGlobalMaximum: 'Null',
      CommissionChangeThrottled: 'Null',
      CommissionChangeRateNotAllowed: 'Null',
      NoPendingCommission: 'Null',
      NoCommissionCurrentSet: 'Null',
      PoolIdInUse: 'Null',
      InvalidPoolId: 'Null',
      BondExtraRestricted: 'Null',
      NothingToAdjust: 'Null',
      PoolTokenCreationFailed: 'Null',
      NoBalanceToUnbond: 'Null'
    }
  },
  /**
   * Lookup761: pallet_tangle_lst::pallet::DefensiveError
   **/
  PalletTangleLstDefensiveError: {
    _enum: ['NotEnoughSpaceInUnbondPool', 'PoolNotFound', 'RewardPoolNotFound', 'SubPoolsNotFound', 'BondedStashKilledPrematurely']
  },
  /**
   * Lookup765: pallet_rewards::pallet::Error<T>
   **/
  PalletRewardsError: {
    _enum: ['NoRewardsAvailable', 'InsufficientRewardsBalance', 'AssetNotWhitelisted', 'AssetAlreadyWhitelisted', 'InvalidAPY', 'AssetAlreadyInVault', 'AssetNotInVault', 'VaultNotFound', 'DuplicateBlueprintId', 'BlueprintIdNotFound', 'RewardConfigNotFound', 'CannotCalculatePropotionalApy', 'CannotCalculateRewardPerBlock', 'IncentiveCapGreaterThanDepositCap', 'BoostMultiplierMustBeOne', 'VaultAlreadyExists', 'TotalDepositLessThanIncentiveCap', 'PotAlreadyExists', 'PotAccountNotFound', 'InvalidDecayRate']
  },
  /**
   * Lookup768: frame_system::extensions::check_non_zero_sender::CheckNonZeroSender<T>
   **/
  FrameSystemExtensionsCheckNonZeroSender: 'Null',
  /**
   * Lookup769: frame_system::extensions::check_spec_version::CheckSpecVersion<T>
   **/
  FrameSystemExtensionsCheckSpecVersion: 'Null',
  /**
   * Lookup770: frame_system::extensions::check_tx_version::CheckTxVersion<T>
   **/
  FrameSystemExtensionsCheckTxVersion: 'Null',
  /**
   * Lookup771: frame_system::extensions::check_genesis::CheckGenesis<T>
   **/
  FrameSystemExtensionsCheckGenesis: 'Null',
  /**
   * Lookup774: frame_system::extensions::check_nonce::CheckNonce<T>
   **/
  FrameSystemExtensionsCheckNonce: 'Compact<u32>',
  /**
   * Lookup775: frame_system::extensions::check_weight::CheckWeight<T>
   **/
  FrameSystemExtensionsCheckWeight: 'Null',
  /**
   * Lookup776: pallet_transaction_payment::ChargeTransactionPayment<T>
   **/
  PalletTransactionPaymentChargeTransactionPayment: 'Compact<u128>',
  /**
   * Lookup777: frame_metadata_hash_extension::CheckMetadataHash<T>
   **/
  FrameMetadataHashExtensionCheckMetadataHash: {
    mode: 'FrameMetadataHashExtensionMode'
  },
  /**
   * Lookup778: frame_metadata_hash_extension::Mode
   **/
  FrameMetadataHashExtensionMode: {
    _enum: ['Disabled', 'Enabled']
  },
  /**
   * Lookup780: tangle_testnet_runtime::Runtime
   **/
  TangleTestnetRuntimeRuntime: 'Null'
};
