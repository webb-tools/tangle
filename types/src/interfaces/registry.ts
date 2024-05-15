// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

// import type lookup before we augment - in some environments
// this is required to allow for ambient/previous definitions
import "@polkadot/types/types/registry";

import type {
  EthbloomBloom,
  EthereumBlock,
  EthereumHeader,
  EthereumLog,
  EthereumReceiptEip658ReceiptData,
  EthereumReceiptReceiptV3,
  EthereumTransactionAccessListItem,
  EthereumTransactionEip1559Transaction,
  EthereumTransactionEip2930Transaction,
  EthereumTransactionLegacyTransaction,
  EthereumTransactionTransactionAction,
  EthereumTransactionTransactionSignature,
  EthereumTransactionTransactionV2,
  EthereumTypesHashH64,
  EvmCoreErrorExitError,
  EvmCoreErrorExitFatal,
  EvmCoreErrorExitReason,
  EvmCoreErrorExitRevert,
  EvmCoreErrorExitSucceed,
  FinalityGrandpaEquivocationPrecommit,
  FinalityGrandpaEquivocationPrevote,
  FinalityGrandpaPrecommit,
  FinalityGrandpaPrevote,
  FpRpcTransactionStatus,
  FrameSupportDispatchDispatchClass,
  FrameSupportDispatchDispatchInfo,
  FrameSupportDispatchPays,
  FrameSupportDispatchPerDispatchClassU32,
  FrameSupportDispatchPerDispatchClassWeight,
  FrameSupportDispatchPerDispatchClassWeightsPerClass,
  FrameSupportDispatchRawOrigin,
  FrameSupportPalletId,
  FrameSupportPreimagesBounded,
  FrameSupportTokensMiscBalanceStatus,
  FrameSystemAccountInfo,
  FrameSystemCall,
  FrameSystemCodeUpgradeAuthorization,
  FrameSystemError,
  FrameSystemEvent,
  FrameSystemEventRecord,
  FrameSystemExtensionsCheckGenesis,
  FrameSystemExtensionsCheckNonZeroSender,
  FrameSystemExtensionsCheckNonce,
  FrameSystemExtensionsCheckSpecVersion,
  FrameSystemExtensionsCheckTxVersion,
  FrameSystemExtensionsCheckWeight,
  FrameSystemLastRuntimeUpgradeInfo,
  FrameSystemLimitsBlockLength,
  FrameSystemLimitsBlockWeights,
  FrameSystemLimitsWeightsPerClass,
  FrameSystemPhase,
  PalletAirdropClaimsCall,
  PalletAirdropClaimsError,
  PalletAirdropClaimsEvent,
  PalletAirdropClaimsStatementKind,
  PalletAirdropClaimsUtilsEthereumAddress,
  PalletAirdropClaimsUtilsEthereumAddressEcdsaSignature,
  PalletAirdropClaimsUtilsMultiAddress,
  PalletAirdropClaimsUtilsMultiAddressSignature,
  PalletAirdropClaimsUtilsSr25519Signature,
  PalletBabeCall,
  PalletBabeError,
  PalletBagsListCall,
  PalletBagsListError,
  PalletBagsListEvent,
  PalletBagsListListBag,
  PalletBagsListListListError,
  PalletBagsListListNode,
  PalletBalancesAccountData,
  PalletBalancesAdjustmentDirection,
  PalletBalancesBalanceLock,
  PalletBalancesCall,
  PalletBalancesError,
  PalletBalancesEvent,
  PalletBalancesIdAmountRuntimeFreezeReason,
  PalletBalancesIdAmountRuntimeHoldReason,
  PalletBalancesReasons,
  PalletBalancesReserveData,
  PalletBaseFeeCall,
  PalletBaseFeeEvent,
  PalletBountiesBounty,
  PalletBountiesBountyStatus,
  PalletBountiesCall,
  PalletBountiesError,
  PalletBountiesEvent,
  PalletChildBountiesCall,
  PalletChildBountiesChildBounty,
  PalletChildBountiesChildBountyStatus,
  PalletChildBountiesError,
  PalletChildBountiesEvent,
  PalletCollectiveCall,
  PalletCollectiveError,
  PalletCollectiveEvent,
  PalletCollectiveRawOrigin,
  PalletCollectiveVotes,
  PalletDemocracyCall,
  PalletDemocracyConviction,
  PalletDemocracyDelegations,
  PalletDemocracyError,
  PalletDemocracyEvent,
  PalletDemocracyMetadataOwner,
  PalletDemocracyReferendumInfo,
  PalletDemocracyReferendumStatus,
  PalletDemocracyTally,
  PalletDemocracyVoteAccountVote,
  PalletDemocracyVotePriorLock,
  PalletDemocracyVoteThreshold,
  PalletDemocracyVoteVoting,
  PalletDkgCall,
  PalletDkgError,
  PalletDkgEvent,
  PalletDkgFeeInfo,
  PalletDynamicFeeCall,
  PalletElectionProviderMultiPhaseCall,
  PalletElectionProviderMultiPhaseElectionCompute,
  PalletElectionProviderMultiPhaseError,
  PalletElectionProviderMultiPhaseEvent,
  PalletElectionProviderMultiPhasePhase,
  PalletElectionProviderMultiPhaseRawSolution,
  PalletElectionProviderMultiPhaseReadySolution,
  PalletElectionProviderMultiPhaseRoundSnapshot,
  PalletElectionProviderMultiPhaseSignedSignedSubmission,
  PalletElectionProviderMultiPhaseSolutionOrSnapshotSize,
  PalletElectionsPhragmenCall,
  PalletElectionsPhragmenError,
  PalletElectionsPhragmenEvent,
  PalletElectionsPhragmenRenouncing,
  PalletElectionsPhragmenSeatHolder,
  PalletElectionsPhragmenVoter,
  PalletEthereumCall,
  PalletEthereumError,
  PalletEthereumEvent,
  PalletEthereumRawOrigin,
  PalletEvmCall,
  PalletEvmCodeMetadata,
  PalletEvmError,
  PalletEvmEvent,
  PalletGrandpaCall,
  PalletGrandpaError,
  PalletGrandpaEvent,
  PalletGrandpaStoredPendingChange,
  PalletGrandpaStoredState,
  PalletHotfixSufficientsCall,
  PalletHotfixSufficientsError,
  PalletIdentityAuthorityProperties,
  PalletIdentityCall,
  PalletIdentityError,
  PalletIdentityEvent,
  PalletIdentityJudgement,
  PalletIdentityLegacyIdentityInfo,
  PalletIdentityRegistrarInfo,
  PalletIdentityRegistration,
  PalletImOnlineCall,
  PalletImOnlineError,
  PalletImOnlineEvent,
  PalletImOnlineHeartbeat,
  PalletImOnlineSr25519AppSr25519Public,
  PalletImOnlineSr25519AppSr25519Signature,
  PalletIndicesCall,
  PalletIndicesError,
  PalletIndicesEvent,
  PalletJobsModuleCall,
  PalletJobsModuleError,
  PalletJobsModuleEvent,
  PalletMultisigCall,
  PalletMultisigError,
  PalletMultisigEvent,
  PalletMultisigMultisig,
  PalletMultisigTimepoint,
  PalletNominationPoolsBondExtra,
  PalletNominationPoolsBondedPoolInner,
  PalletNominationPoolsCall,
  PalletNominationPoolsClaimPermission,
  PalletNominationPoolsCommission,
  PalletNominationPoolsCommissionChangeRate,
  PalletNominationPoolsCommissionClaimPermission,
  PalletNominationPoolsConfigOpAccountId32,
  PalletNominationPoolsConfigOpPerbill,
  PalletNominationPoolsConfigOpU128,
  PalletNominationPoolsConfigOpU32,
  PalletNominationPoolsDefensiveError,
  PalletNominationPoolsError,
  PalletNominationPoolsEvent,
  PalletNominationPoolsFreezeReason,
  PalletNominationPoolsPoolMember,
  PalletNominationPoolsPoolRoles,
  PalletNominationPoolsPoolState,
  PalletNominationPoolsRewardPool,
  PalletNominationPoolsSubPools,
  PalletNominationPoolsUnbondPool,
  PalletOffencesEvent,
  PalletPreimageCall,
  PalletPreimageError,
  PalletPreimageEvent,
  PalletPreimageHoldReason,
  PalletPreimageOldRequestStatus,
  PalletPreimageRequestStatus,
  PalletProxyAnnouncement,
  PalletProxyCall,
  PalletProxyError,
  PalletProxyEvent,
  PalletProxyProxyDefinition,
  PalletRolesCall,
  PalletRolesError,
  PalletRolesEvent,
  PalletRolesProfile,
  PalletRolesProfileIndependentRestakeProfile,
  PalletRolesProfileRecord,
  PalletRolesProfileSharedRestakeProfile,
  PalletRolesRestakingLedger,
  PalletRolesUnlockChunk,
  PalletSchedulerCall,
  PalletSchedulerError,
  PalletSchedulerEvent,
  PalletSchedulerScheduled,
  PalletServicesModuleCall,
  PalletServicesModuleError,
  PalletServicesModuleEvent,
  PalletSessionCall,
  PalletSessionError,
  PalletSessionEvent,
  PalletStakingActiveEraInfo,
  PalletStakingEraRewardPoints,
  PalletStakingForcing,
  PalletStakingNominations,
  PalletStakingPalletCall,
  PalletStakingPalletConfigOpPerbill,
  PalletStakingPalletConfigOpPercent,
  PalletStakingPalletConfigOpU128,
  PalletStakingPalletConfigOpU32,
  PalletStakingPalletError,
  PalletStakingPalletEvent,
  PalletStakingRewardDestination,
  PalletStakingSlashingSlashingSpans,
  PalletStakingSlashingSpanRecord,
  PalletStakingStakingLedger,
  PalletStakingUnappliedSlash,
  PalletStakingUnlockChunk,
  PalletStakingValidatorPrefs,
  PalletSudoCall,
  PalletSudoError,
  PalletSudoEvent,
  PalletTimestampCall,
  PalletTransactionPaymentChargeTransactionPayment,
  PalletTransactionPaymentEvent,
  PalletTransactionPaymentReleases,
  PalletTreasuryCall,
  PalletTreasuryError,
  PalletTreasuryEvent,
  PalletTreasuryPaymentState,
  PalletTreasuryProposal,
  PalletTreasurySpendStatus,
  PalletTxPauseCall,
  PalletTxPauseError,
  PalletTxPauseEvent,
  PalletUtilityCall,
  PalletUtilityError,
  PalletUtilityEvent,
  PalletVestingCall,
  PalletVestingError,
  PalletVestingEvent,
  PalletVestingReleases,
  PalletVestingVestingInfo,
  PalletZksaasCall,
  PalletZksaasError,
  PalletZksaasEvent,
  PalletZksaasFeeInfo,
  SpArithmeticArithmeticError,
  SpConsensusBabeAllowedSlots,
  SpConsensusBabeAppPublic,
  SpConsensusBabeBabeEpochConfiguration,
  SpConsensusBabeDigestsNextConfigDescriptor,
  SpConsensusBabeDigestsPreDigest,
  SpConsensusBabeDigestsPrimaryPreDigest,
  SpConsensusBabeDigestsSecondaryPlainPreDigest,
  SpConsensusBabeDigestsSecondaryVRFPreDigest,
  SpConsensusGrandpaAppPublic,
  SpConsensusGrandpaAppSignature,
  SpConsensusGrandpaEquivocation,
  SpConsensusGrandpaEquivocationProof,
  SpConsensusSlotsEquivocationProof,
  SpCoreCryptoKeyTypeId,
  SpCoreEcdsaPublic,
  SpCoreEcdsaSignature,
  SpCoreEd25519Public,
  SpCoreEd25519Signature,
  SpCoreSr25519Public,
  SpCoreSr25519Signature,
  SpCoreSr25519VrfVrfSignature,
  SpCoreVoid,
  SpNposElectionsElectionScore,
  SpNposElectionsSupport,
  SpRuntimeBlakeTwo256,
  SpRuntimeDigest,
  SpRuntimeDigestDigestItem,
  SpRuntimeDispatchError,
  SpRuntimeHeader,
  SpRuntimeModuleError,
  SpRuntimeMultiSignature,
  SpRuntimeTokenError,
  SpRuntimeTransactionalError,
  SpSessionMembershipProof,
  SpStakingExposure,
  SpStakingExposurePage,
  SpStakingIndividualExposure,
  SpStakingOffenceOffenceDetails,
  SpStakingPagedExposureMetadata,
  SpVersionRuntimeVersion,
  SpWeightsRuntimeDbWeight,
  SpWeightsWeightV2Weight,
  TangleCryptoPrimitivesCryptoPublic,
  TanglePrimitivesJobsFallbackOptions,
  TanglePrimitivesJobsJobInfo,
  TanglePrimitivesJobsJobResult,
  TanglePrimitivesJobsJobSubmission,
  TanglePrimitivesJobsJobType,
  TanglePrimitivesJobsPhaseResult,
  TanglePrimitivesJobsTssDigitalSignatureScheme,
  TanglePrimitivesJobsTssDkgtssKeyRefreshResult,
  TanglePrimitivesJobsTssDkgtssKeyRotationResult,
  TanglePrimitivesJobsTssDkgtssKeySubmissionResult,
  TanglePrimitivesJobsTssDkgtssPhaseFourJobType,
  TanglePrimitivesJobsTssDkgtssPhaseOneJobType,
  TanglePrimitivesJobsTssDkgtssPhaseThreeJobType,
  TanglePrimitivesJobsTssDkgtssPhaseTwoJobType,
  TanglePrimitivesJobsTssDkgtssSignatureResult,
  TanglePrimitivesJobsV2ApprovalPrefrence,
  TanglePrimitivesJobsV2ApprovalState,
  TanglePrimitivesJobsV2Architecture,
  TanglePrimitivesJobsV2ContainerGadget,
  TanglePrimitivesJobsV2Field,
  TanglePrimitivesJobsV2FieldFieldType,
  TanglePrimitivesJobsV2Gadget,
  TanglePrimitivesJobsV2GadgetBinary,
  TanglePrimitivesJobsV2GithubFetcher,
  TanglePrimitivesJobsV2ImageRegistryFetcher,
  TanglePrimitivesJobsV2JobCall,
  TanglePrimitivesJobsV2JobCallResult,
  TanglePrimitivesJobsV2JobDefinition,
  TanglePrimitivesJobsV2JobMetadata,
  TanglePrimitivesJobsV2JobResultVerifier,
  TanglePrimitivesJobsV2NativeGadget,
  TanglePrimitivesJobsV2OperatingSystem,
  TanglePrimitivesJobsV2OperatorPreferences,
  TanglePrimitivesJobsV2OperatorProfile,
  TanglePrimitivesJobsV2Service,
  TanglePrimitivesJobsV2ServiceBlueprint,
  TanglePrimitivesJobsV2ServiceMetadata,
  TanglePrimitivesJobsV2ServiceRegistrationHook,
  TanglePrimitivesJobsV2ServiceRequest,
  TanglePrimitivesJobsV2ServiceRequestHook,
  TanglePrimitivesJobsV2TypeCheckError,
  TanglePrimitivesJobsV2WasmGadget,
  TanglePrimitivesJobsValidatorOffenceType,
  TanglePrimitivesJobsZksaasArkworksProofResult,
  TanglePrimitivesJobsZksaasCircomProofResult,
  TanglePrimitivesJobsZksaasGroth16ProveRequest,
  TanglePrimitivesJobsZksaasGroth16System,
  TanglePrimitivesJobsZksaasHyperData,
  TanglePrimitivesJobsZksaasQapShare,
  TanglePrimitivesJobsZksaasZkSaaSCircuitResult,
  TanglePrimitivesJobsZksaasZkSaaSPhaseOneJobType,
  TanglePrimitivesJobsZksaasZkSaaSPhaseTwoJobType,
  TanglePrimitivesJobsZksaasZkSaaSPhaseTwoRequest,
  TanglePrimitivesJobsZksaasZkSaaSProofResult,
  TanglePrimitivesJobsZksaasZkSaaSSystem,
  TanglePrimitivesMisbehaviorDfnsCggmp21DfnsCGGMP21Justification,
  TanglePrimitivesMisbehaviorDfnsCggmp21InvalidProofReason,
  TanglePrimitivesMisbehaviorDfnsCggmp21KeyRefreshAborted,
  TanglePrimitivesMisbehaviorDfnsCggmp21KeygenAborted,
  TanglePrimitivesMisbehaviorDfnsCggmp21SigningAborted,
  TanglePrimitivesMisbehaviorDkgtssJustification,
  TanglePrimitivesMisbehaviorMisbehaviorJustification,
  TanglePrimitivesMisbehaviorMisbehaviorSubmission,
  TanglePrimitivesMisbehaviorSignedRoundMessage,
  TanglePrimitivesMisbehaviorZcashFrostKeygenAborted,
  TanglePrimitivesMisbehaviorZcashFrostSigningAborted,
  TanglePrimitivesMisbehaviorZcashFrostZCashFrostJustification,
  TanglePrimitivesMisbehaviorZkSaaSJustification,
  TanglePrimitivesRolesRoleType,
  TanglePrimitivesRolesTssThresholdSignatureRoleType,
  TanglePrimitivesRolesZksaasZeroKnowledgeRoleType,
  TangleTestnetRuntimeMaxAdditionalParamsLen,
  TangleTestnetRuntimeMaxDataLen,
  TangleTestnetRuntimeMaxKeyLen,
  TangleTestnetRuntimeMaxParticipants,
  TangleTestnetRuntimeMaxProofLen,
  TangleTestnetRuntimeMaxSignatureLen,
  TangleTestnetRuntimeMaxSubmissionLen,
  TangleTestnetRuntimeNposSolution16,
  TangleTestnetRuntimeOpaqueSessionKeys,
  TangleTestnetRuntimeOriginCaller,
  TangleTestnetRuntimeProxyType,
  TangleTestnetRuntimeRuntime,
  TangleTestnetRuntimeRuntimeFreezeReason,
  TangleTestnetRuntimeRuntimeHoldReason,
} from "@polkadot/types/lookup";

declare module "@polkadot/types/types/registry" {
  interface InterfaceTypes {
    EthbloomBloom: EthbloomBloom;
    EthereumBlock: EthereumBlock;
    EthereumHeader: EthereumHeader;
    EthereumLog: EthereumLog;
    EthereumReceiptEip658ReceiptData: EthereumReceiptEip658ReceiptData;
    EthereumReceiptReceiptV3: EthereumReceiptReceiptV3;
    EthereumTransactionAccessListItem: EthereumTransactionAccessListItem;
    EthereumTransactionEip1559Transaction: EthereumTransactionEip1559Transaction;
    EthereumTransactionEip2930Transaction: EthereumTransactionEip2930Transaction;
    EthereumTransactionLegacyTransaction: EthereumTransactionLegacyTransaction;
    EthereumTransactionTransactionAction: EthereumTransactionTransactionAction;
    EthereumTransactionTransactionSignature: EthereumTransactionTransactionSignature;
    EthereumTransactionTransactionV2: EthereumTransactionTransactionV2;
    EthereumTypesHashH64: EthereumTypesHashH64;
    EvmCoreErrorExitError: EvmCoreErrorExitError;
    EvmCoreErrorExitFatal: EvmCoreErrorExitFatal;
    EvmCoreErrorExitReason: EvmCoreErrorExitReason;
    EvmCoreErrorExitRevert: EvmCoreErrorExitRevert;
    EvmCoreErrorExitSucceed: EvmCoreErrorExitSucceed;
    FinalityGrandpaEquivocationPrecommit: FinalityGrandpaEquivocationPrecommit;
    FinalityGrandpaEquivocationPrevote: FinalityGrandpaEquivocationPrevote;
    FinalityGrandpaPrecommit: FinalityGrandpaPrecommit;
    FinalityGrandpaPrevote: FinalityGrandpaPrevote;
    FpRpcTransactionStatus: FpRpcTransactionStatus;
    FrameSupportDispatchDispatchClass: FrameSupportDispatchDispatchClass;
    FrameSupportDispatchDispatchInfo: FrameSupportDispatchDispatchInfo;
    FrameSupportDispatchPays: FrameSupportDispatchPays;
    FrameSupportDispatchPerDispatchClassU32: FrameSupportDispatchPerDispatchClassU32;
    FrameSupportDispatchPerDispatchClassWeight: FrameSupportDispatchPerDispatchClassWeight;
    FrameSupportDispatchPerDispatchClassWeightsPerClass: FrameSupportDispatchPerDispatchClassWeightsPerClass;
    FrameSupportDispatchRawOrigin: FrameSupportDispatchRawOrigin;
    FrameSupportPalletId: FrameSupportPalletId;
    FrameSupportPreimagesBounded: FrameSupportPreimagesBounded;
    FrameSupportTokensMiscBalanceStatus: FrameSupportTokensMiscBalanceStatus;
    FrameSystemAccountInfo: FrameSystemAccountInfo;
    FrameSystemCall: FrameSystemCall;
    FrameSystemCodeUpgradeAuthorization: FrameSystemCodeUpgradeAuthorization;
    FrameSystemError: FrameSystemError;
    FrameSystemEvent: FrameSystemEvent;
    FrameSystemEventRecord: FrameSystemEventRecord;
    FrameSystemExtensionsCheckGenesis: FrameSystemExtensionsCheckGenesis;
    FrameSystemExtensionsCheckNonZeroSender: FrameSystemExtensionsCheckNonZeroSender;
    FrameSystemExtensionsCheckNonce: FrameSystemExtensionsCheckNonce;
    FrameSystemExtensionsCheckSpecVersion: FrameSystemExtensionsCheckSpecVersion;
    FrameSystemExtensionsCheckTxVersion: FrameSystemExtensionsCheckTxVersion;
    FrameSystemExtensionsCheckWeight: FrameSystemExtensionsCheckWeight;
    FrameSystemLastRuntimeUpgradeInfo: FrameSystemLastRuntimeUpgradeInfo;
    FrameSystemLimitsBlockLength: FrameSystemLimitsBlockLength;
    FrameSystemLimitsBlockWeights: FrameSystemLimitsBlockWeights;
    FrameSystemLimitsWeightsPerClass: FrameSystemLimitsWeightsPerClass;
    FrameSystemPhase: FrameSystemPhase;
    PalletAirdropClaimsCall: PalletAirdropClaimsCall;
    PalletAirdropClaimsError: PalletAirdropClaimsError;
    PalletAirdropClaimsEvent: PalletAirdropClaimsEvent;
    PalletAirdropClaimsStatementKind: PalletAirdropClaimsStatementKind;
    PalletAirdropClaimsUtilsEthereumAddress: PalletAirdropClaimsUtilsEthereumAddress;
    PalletAirdropClaimsUtilsEthereumAddressEcdsaSignature: PalletAirdropClaimsUtilsEthereumAddressEcdsaSignature;
    PalletAirdropClaimsUtilsMultiAddress: PalletAirdropClaimsUtilsMultiAddress;
    PalletAirdropClaimsUtilsMultiAddressSignature: PalletAirdropClaimsUtilsMultiAddressSignature;
    PalletAirdropClaimsUtilsSr25519Signature: PalletAirdropClaimsUtilsSr25519Signature;
    PalletAssetsAccountStatus: PalletAssetsAccountStatus;
    PalletAssetsApproval: PalletAssetsApproval;
    PalletAssetsAssetAccount: PalletAssetsAssetAccount;
    PalletAssetsAssetDetails: PalletAssetsAssetDetails;
    PalletAssetsAssetMetadata: PalletAssetsAssetMetadata;
    PalletAssetsAssetStatus: PalletAssetsAssetStatus;
    PalletAssetsCall: PalletAssetsCall;
    PalletAssetsError: PalletAssetsError;
    PalletAssetsEvent: PalletAssetsEvent;
    PalletAssetsExistenceReason: PalletAssetsExistenceReason;
    PalletBabeCall: PalletBabeCall;
    PalletBabeError: PalletBabeError;
    PalletBagsListCall: PalletBagsListCall;
    PalletBagsListError: PalletBagsListError;
    PalletBagsListEvent: PalletBagsListEvent;
    PalletBagsListListBag: PalletBagsListListBag;
    PalletBagsListListListError: PalletBagsListListListError;
    PalletBagsListListNode: PalletBagsListListNode;
    PalletBalancesAccountData: PalletBalancesAccountData;
    PalletBalancesAdjustmentDirection: PalletBalancesAdjustmentDirection;
    PalletBalancesBalanceLock: PalletBalancesBalanceLock;
    PalletBalancesCall: PalletBalancesCall;
    PalletBalancesError: PalletBalancesError;
    PalletBalancesEvent: PalletBalancesEvent;
    PalletBalancesIdAmountRuntimeFreezeReason: PalletBalancesIdAmountRuntimeFreezeReason;
    PalletBalancesIdAmountRuntimeHoldReason: PalletBalancesIdAmountRuntimeHoldReason;
    PalletBalancesReasons: PalletBalancesReasons;
    PalletBalancesReserveData: PalletBalancesReserveData;
    PalletBaseFeeCall: PalletBaseFeeCall;
    PalletBaseFeeEvent: PalletBaseFeeEvent;
    PalletBountiesBounty: PalletBountiesBounty;
    PalletBountiesBountyStatus: PalletBountiesBountyStatus;
    PalletBountiesCall: PalletBountiesCall;
    PalletBountiesError: PalletBountiesError;
    PalletBountiesEvent: PalletBountiesEvent;
    PalletChildBountiesCall: PalletChildBountiesCall;
    PalletChildBountiesChildBounty: PalletChildBountiesChildBounty;
    PalletChildBountiesChildBountyStatus: PalletChildBountiesChildBountyStatus;
    PalletChildBountiesError: PalletChildBountiesError;
    PalletChildBountiesEvent: PalletChildBountiesEvent;
    PalletCollectiveCall: PalletCollectiveCall;
    PalletCollectiveError: PalletCollectiveError;
    PalletCollectiveEvent: PalletCollectiveEvent;
    PalletCollectiveRawOrigin: PalletCollectiveRawOrigin;
    PalletCollectiveVotes: PalletCollectiveVotes;
    PalletDemocracyCall: PalletDemocracyCall;
    PalletDemocracyConviction: PalletDemocracyConviction;
    PalletDemocracyDelegations: PalletDemocracyDelegations;
    PalletDemocracyError: PalletDemocracyError;
    PalletDemocracyEvent: PalletDemocracyEvent;
    PalletDemocracyMetadataOwner: PalletDemocracyMetadataOwner;
    PalletDemocracyReferendumInfo: PalletDemocracyReferendumInfo;
    PalletDemocracyReferendumStatus: PalletDemocracyReferendumStatus;
    PalletDemocracyTally: PalletDemocracyTally;
    PalletDemocracyVoteAccountVote: PalletDemocracyVoteAccountVote;
    PalletDemocracyVotePriorLock: PalletDemocracyVotePriorLock;
    PalletDemocracyVoteThreshold: PalletDemocracyVoteThreshold;
    PalletDemocracyVoteVoting: PalletDemocracyVoteVoting;
    PalletDkgCall: PalletDkgCall;
    PalletDkgError: PalletDkgError;
    PalletDkgEvent: PalletDkgEvent;
    PalletDkgFeeInfo: PalletDkgFeeInfo;
    PalletDynamicFeeCall: PalletDynamicFeeCall;
    PalletElectionProviderMultiPhaseCall: PalletElectionProviderMultiPhaseCall;
    PalletElectionProviderMultiPhaseElectionCompute: PalletElectionProviderMultiPhaseElectionCompute;
    PalletElectionProviderMultiPhaseError: PalletElectionProviderMultiPhaseError;
    PalletElectionProviderMultiPhaseEvent: PalletElectionProviderMultiPhaseEvent;
    PalletElectionProviderMultiPhasePhase: PalletElectionProviderMultiPhasePhase;
    PalletElectionProviderMultiPhaseRawSolution: PalletElectionProviderMultiPhaseRawSolution;
    PalletElectionProviderMultiPhaseReadySolution: PalletElectionProviderMultiPhaseReadySolution;
    PalletElectionProviderMultiPhaseRoundSnapshot: PalletElectionProviderMultiPhaseRoundSnapshot;
    PalletElectionProviderMultiPhaseSignedSignedSubmission: PalletElectionProviderMultiPhaseSignedSignedSubmission;
    PalletElectionProviderMultiPhaseSolutionOrSnapshotSize: PalletElectionProviderMultiPhaseSolutionOrSnapshotSize;
    PalletElectionsPhragmenCall: PalletElectionsPhragmenCall;
    PalletElectionsPhragmenError: PalletElectionsPhragmenError;
    PalletElectionsPhragmenEvent: PalletElectionsPhragmenEvent;
    PalletElectionsPhragmenRenouncing: PalletElectionsPhragmenRenouncing;
    PalletElectionsPhragmenSeatHolder: PalletElectionsPhragmenSeatHolder;
    PalletElectionsPhragmenVoter: PalletElectionsPhragmenVoter;
    PalletEthereumCall: PalletEthereumCall;
    PalletEthereumError: PalletEthereumError;
    PalletEthereumEvent: PalletEthereumEvent;
    PalletEthereumRawOrigin: PalletEthereumRawOrigin;
    PalletEvmCall: PalletEvmCall;
    PalletEvmCodeMetadata: PalletEvmCodeMetadata;
    PalletEvmError: PalletEvmError;
    PalletEvmEvent: PalletEvmEvent;
    PalletGrandpaCall: PalletGrandpaCall;
    PalletGrandpaError: PalletGrandpaError;
    PalletGrandpaEvent: PalletGrandpaEvent;
    PalletGrandpaStoredPendingChange: PalletGrandpaStoredPendingChange;
    PalletGrandpaStoredState: PalletGrandpaStoredState;
    PalletHotfixSufficientsCall: PalletHotfixSufficientsCall;
    PalletHotfixSufficientsError: PalletHotfixSufficientsError;
    PalletIdentityAuthorityProperties: PalletIdentityAuthorityProperties;
    PalletIdentityCall: PalletIdentityCall;
    PalletIdentityError: PalletIdentityError;
    PalletIdentityEvent: PalletIdentityEvent;
    PalletIdentityJudgement: PalletIdentityJudgement;
    PalletIdentityLegacyIdentityInfo: PalletIdentityLegacyIdentityInfo;
    PalletIdentityRegistrarInfo: PalletIdentityRegistrarInfo;
    PalletIdentityRegistration: PalletIdentityRegistration;
    PalletImOnlineCall: PalletImOnlineCall;
    PalletImOnlineError: PalletImOnlineError;
    PalletImOnlineEvent: PalletImOnlineEvent;
    PalletImOnlineHeartbeat: PalletImOnlineHeartbeat;
    PalletImOnlineSr25519AppSr25519Public: PalletImOnlineSr25519AppSr25519Public;
    PalletImOnlineSr25519AppSr25519Signature: PalletImOnlineSr25519AppSr25519Signature;
    PalletIndicesCall: PalletIndicesCall;
    PalletIndicesError: PalletIndicesError;
    PalletIndicesEvent: PalletIndicesEvent;
    PalletJobsModuleCall: PalletJobsModuleCall;
    PalletJobsModuleError: PalletJobsModuleError;
    PalletJobsModuleEvent: PalletJobsModuleEvent;
    PalletMultiAssetDelegationCall: PalletMultiAssetDelegationCall;
    PalletMultiAssetDelegationDelegatorBondInfoDelegator: PalletMultiAssetDelegationDelegatorBondInfoDelegator;
    PalletMultiAssetDelegationDelegatorBondLessRequest: PalletMultiAssetDelegationDelegatorBondLessRequest;
    PalletMultiAssetDelegationDelegatorDelegatorMetadata: PalletMultiAssetDelegationDelegatorDelegatorMetadata;
    PalletMultiAssetDelegationDelegatorDelegatorStatus: PalletMultiAssetDelegationDelegatorDelegatorStatus;
    PalletMultiAssetDelegationDelegatorUnstakeRequest: PalletMultiAssetDelegationDelegatorUnstakeRequest;
    PalletMultiAssetDelegationError: PalletMultiAssetDelegationError;
    PalletMultiAssetDelegationEvent: PalletMultiAssetDelegationEvent;
    PalletMultiAssetDelegationOperatorDelegatorBond: PalletMultiAssetDelegationOperatorDelegatorBond;
    PalletMultiAssetDelegationOperatorOperatorBondLessRequest: PalletMultiAssetDelegationOperatorOperatorBondLessRequest;
    PalletMultiAssetDelegationOperatorOperatorMetadata: PalletMultiAssetDelegationOperatorOperatorMetadata;
    PalletMultiAssetDelegationOperatorOperatorSnapshot: PalletMultiAssetDelegationOperatorOperatorSnapshot;
    PalletMultiAssetDelegationOperatorOperatorStatus: PalletMultiAssetDelegationOperatorOperatorStatus;
    PalletMultiAssetDelegationRewardsRewardConfig: PalletMultiAssetDelegationRewardsRewardConfig;
    PalletMultiAssetDelegationRewardsRewardConfigForAsset: PalletMultiAssetDelegationRewardsRewardConfigForAsset;
    PalletMultisigCall: PalletMultisigCall;
    PalletMultisigError: PalletMultisigError;
    PalletMultisigEvent: PalletMultisigEvent;
    PalletMultisigMultisig: PalletMultisigMultisig;
    PalletMultisigTimepoint: PalletMultisigTimepoint;
    PalletNominationPoolsBondExtra: PalletNominationPoolsBondExtra;
    PalletNominationPoolsBondedPoolInner: PalletNominationPoolsBondedPoolInner;
    PalletNominationPoolsCall: PalletNominationPoolsCall;
    PalletNominationPoolsClaimPermission: PalletNominationPoolsClaimPermission;
    PalletNominationPoolsCommission: PalletNominationPoolsCommission;
    PalletNominationPoolsCommissionChangeRate: PalletNominationPoolsCommissionChangeRate;
    PalletNominationPoolsCommissionClaimPermission: PalletNominationPoolsCommissionClaimPermission;
    PalletNominationPoolsConfigOpAccountId32: PalletNominationPoolsConfigOpAccountId32;
    PalletNominationPoolsConfigOpPerbill: PalletNominationPoolsConfigOpPerbill;
    PalletNominationPoolsConfigOpU128: PalletNominationPoolsConfigOpU128;
    PalletNominationPoolsConfigOpU32: PalletNominationPoolsConfigOpU32;
    PalletNominationPoolsDefensiveError: PalletNominationPoolsDefensiveError;
    PalletNominationPoolsError: PalletNominationPoolsError;
    PalletNominationPoolsEvent: PalletNominationPoolsEvent;
    PalletNominationPoolsFreezeReason: PalletNominationPoolsFreezeReason;
    PalletNominationPoolsPoolMember: PalletNominationPoolsPoolMember;
    PalletNominationPoolsPoolRoles: PalletNominationPoolsPoolRoles;
    PalletNominationPoolsPoolState: PalletNominationPoolsPoolState;
    PalletNominationPoolsRewardPool: PalletNominationPoolsRewardPool;
    PalletNominationPoolsSubPools: PalletNominationPoolsSubPools;
    PalletNominationPoolsUnbondPool: PalletNominationPoolsUnbondPool;
    PalletOffencesEvent: PalletOffencesEvent;
    PalletPreimageCall: PalletPreimageCall;
    PalletPreimageError: PalletPreimageError;
    PalletPreimageEvent: PalletPreimageEvent;
    PalletPreimageHoldReason: PalletPreimageHoldReason;
    PalletPreimageOldRequestStatus: PalletPreimageOldRequestStatus;
    PalletPreimageRequestStatus: PalletPreimageRequestStatus;
    PalletProxyAnnouncement: PalletProxyAnnouncement;
    PalletProxyCall: PalletProxyCall;
    PalletProxyError: PalletProxyError;
    PalletProxyEvent: PalletProxyEvent;
    PalletProxyProxyDefinition: PalletProxyProxyDefinition;
    PalletRolesCall: PalletRolesCall;
    PalletRolesError: PalletRolesError;
    PalletRolesEvent: PalletRolesEvent;
    PalletRolesProfile: PalletRolesProfile;
    PalletRolesProfileIndependentRestakeProfile: PalletRolesProfileIndependentRestakeProfile;
    PalletRolesProfileRecord: PalletRolesProfileRecord;
    PalletRolesProfileSharedRestakeProfile: PalletRolesProfileSharedRestakeProfile;
    PalletRolesRestakingLedger: PalletRolesRestakingLedger;
    PalletRolesUnlockChunk: PalletRolesUnlockChunk;
    PalletSchedulerCall: PalletSchedulerCall;
    PalletSchedulerError: PalletSchedulerError;
    PalletSchedulerEvent: PalletSchedulerEvent;
    PalletSchedulerScheduled: PalletSchedulerScheduled;
    PalletServicesModuleCall: PalletServicesModuleCall;
    PalletServicesModuleError: PalletServicesModuleError;
    PalletServicesModuleEvent: PalletServicesModuleEvent;
    PalletSessionCall: PalletSessionCall;
    PalletSessionError: PalletSessionError;
    PalletSessionEvent: PalletSessionEvent;
    PalletStakingActiveEraInfo: PalletStakingActiveEraInfo;
    PalletStakingEraRewardPoints: PalletStakingEraRewardPoints;
    PalletStakingForcing: PalletStakingForcing;
    PalletStakingNominations: PalletStakingNominations;
    PalletStakingPalletCall: PalletStakingPalletCall;
    PalletStakingPalletConfigOpPerbill: PalletStakingPalletConfigOpPerbill;
    PalletStakingPalletConfigOpPercent: PalletStakingPalletConfigOpPercent;
    PalletStakingPalletConfigOpU128: PalletStakingPalletConfigOpU128;
    PalletStakingPalletConfigOpU32: PalletStakingPalletConfigOpU32;
    PalletStakingPalletError: PalletStakingPalletError;
    PalletStakingPalletEvent: PalletStakingPalletEvent;
    PalletStakingRewardDestination: PalletStakingRewardDestination;
    PalletStakingSlashingSlashingSpans: PalletStakingSlashingSlashingSpans;
    PalletStakingSlashingSpanRecord: PalletStakingSlashingSpanRecord;
    PalletStakingStakingLedger: PalletStakingStakingLedger;
    PalletStakingUnappliedSlash: PalletStakingUnappliedSlash;
    PalletStakingUnlockChunk: PalletStakingUnlockChunk;
    PalletStakingValidatorPrefs: PalletStakingValidatorPrefs;
    PalletSudoCall: PalletSudoCall;
    PalletSudoError: PalletSudoError;
    PalletSudoEvent: PalletSudoEvent;
    PalletTimestampCall: PalletTimestampCall;
    PalletTransactionPaymentChargeTransactionPayment: PalletTransactionPaymentChargeTransactionPayment;
    PalletTransactionPaymentEvent: PalletTransactionPaymentEvent;
    PalletTransactionPaymentReleases: PalletTransactionPaymentReleases;
    PalletTreasuryCall: PalletTreasuryCall;
    PalletTreasuryError: PalletTreasuryError;
    PalletTreasuryEvent: PalletTreasuryEvent;
    PalletTreasuryPaymentState: PalletTreasuryPaymentState;
    PalletTreasuryProposal: PalletTreasuryProposal;
    PalletTreasurySpendStatus: PalletTreasurySpendStatus;
    PalletTxPauseCall: PalletTxPauseCall;
    PalletTxPauseError: PalletTxPauseError;
    PalletTxPauseEvent: PalletTxPauseEvent;
    PalletUtilityCall: PalletUtilityCall;
    PalletUtilityError: PalletUtilityError;
    PalletUtilityEvent: PalletUtilityEvent;
    PalletVestingCall: PalletVestingCall;
    PalletVestingError: PalletVestingError;
    PalletVestingEvent: PalletVestingEvent;
    PalletVestingReleases: PalletVestingReleases;
    PalletVestingVestingInfo: PalletVestingVestingInfo;
    PalletZksaasCall: PalletZksaasCall;
    PalletZksaasError: PalletZksaasError;
    PalletZksaasEvent: PalletZksaasEvent;
    PalletZksaasFeeInfo: PalletZksaasFeeInfo;
    SpArithmeticArithmeticError: SpArithmeticArithmeticError;
    SpConsensusBabeAllowedSlots: SpConsensusBabeAllowedSlots;
    SpConsensusBabeAppPublic: SpConsensusBabeAppPublic;
    SpConsensusBabeBabeEpochConfiguration: SpConsensusBabeBabeEpochConfiguration;
    SpConsensusBabeDigestsNextConfigDescriptor: SpConsensusBabeDigestsNextConfigDescriptor;
    SpConsensusBabeDigestsPreDigest: SpConsensusBabeDigestsPreDigest;
    SpConsensusBabeDigestsPrimaryPreDigest: SpConsensusBabeDigestsPrimaryPreDigest;
    SpConsensusBabeDigestsSecondaryPlainPreDigest: SpConsensusBabeDigestsSecondaryPlainPreDigest;
    SpConsensusBabeDigestsSecondaryVRFPreDigest: SpConsensusBabeDigestsSecondaryVRFPreDigest;
    SpConsensusGrandpaAppPublic: SpConsensusGrandpaAppPublic;
    SpConsensusGrandpaAppSignature: SpConsensusGrandpaAppSignature;
    SpConsensusGrandpaEquivocation: SpConsensusGrandpaEquivocation;
    SpConsensusGrandpaEquivocationProof: SpConsensusGrandpaEquivocationProof;
    SpConsensusSlotsEquivocationProof: SpConsensusSlotsEquivocationProof;
    SpCoreCryptoKeyTypeId: SpCoreCryptoKeyTypeId;
    SpCoreEcdsaPublic: SpCoreEcdsaPublic;
    SpCoreEcdsaSignature: SpCoreEcdsaSignature;
    SpCoreEd25519Public: SpCoreEd25519Public;
    SpCoreEd25519Signature: SpCoreEd25519Signature;
    SpCoreSr25519Public: SpCoreSr25519Public;
    SpCoreSr25519Signature: SpCoreSr25519Signature;
    SpCoreSr25519VrfVrfSignature: SpCoreSr25519VrfVrfSignature;
    SpCoreVoid: SpCoreVoid;
    SpNposElectionsElectionScore: SpNposElectionsElectionScore;
    SpNposElectionsSupport: SpNposElectionsSupport;
    SpRuntimeBlakeTwo256: SpRuntimeBlakeTwo256;
    SpRuntimeDigest: SpRuntimeDigest;
    SpRuntimeDigestDigestItem: SpRuntimeDigestDigestItem;
    SpRuntimeDispatchError: SpRuntimeDispatchError;
    SpRuntimeHeader: SpRuntimeHeader;
    SpRuntimeModuleError: SpRuntimeModuleError;
    SpRuntimeMultiSignature: SpRuntimeMultiSignature;
    SpRuntimeTokenError: SpRuntimeTokenError;
    SpRuntimeTransactionalError: SpRuntimeTransactionalError;
    SpSessionMembershipProof: SpSessionMembershipProof;
    SpStakingExposure: SpStakingExposure;
    SpStakingExposurePage: SpStakingExposurePage;
    SpStakingIndividualExposure: SpStakingIndividualExposure;
    SpStakingOffenceOffenceDetails: SpStakingOffenceOffenceDetails;
    SpStakingPagedExposureMetadata: SpStakingPagedExposureMetadata;
    SpVersionRuntimeVersion: SpVersionRuntimeVersion;
    SpWeightsRuntimeDbWeight: SpWeightsRuntimeDbWeight;
    SpWeightsWeightV2Weight: SpWeightsWeightV2Weight;
    StagingXcmV4Asset: StagingXcmV4Asset;
    StagingXcmV4AssetAssetId: StagingXcmV4AssetAssetId;
    StagingXcmV4AssetAssetInstance: StagingXcmV4AssetAssetInstance;
    StagingXcmV4AssetFungibility: StagingXcmV4AssetFungibility;
    StagingXcmV4Junction: StagingXcmV4Junction;
    StagingXcmV4JunctionNetworkId: StagingXcmV4JunctionNetworkId;
    StagingXcmV4Junctions: StagingXcmV4Junctions;
    StagingXcmV4Location: StagingXcmV4Location;
    SygmaAccessSegregatorCall: SygmaAccessSegregatorCall;
    SygmaAccessSegregatorError: SygmaAccessSegregatorError;
    SygmaAccessSegregatorEvent: SygmaAccessSegregatorEvent;
    SygmaBasicFeehandlerCall: SygmaBasicFeehandlerCall;
    SygmaBasicFeehandlerError: SygmaBasicFeehandlerError;
    SygmaBasicFeehandlerEvent: SygmaBasicFeehandlerEvent;
    SygmaBridgeCall: SygmaBridgeCall;
    SygmaBridgeError: SygmaBridgeError;
    SygmaBridgeEvent: SygmaBridgeEvent;
    SygmaBridgeProposal: SygmaBridgeProposal;
    SygmaFeeHandlerRouterCall: SygmaFeeHandlerRouterCall;
    SygmaFeeHandlerRouterError: SygmaFeeHandlerRouterError;
    SygmaFeeHandlerRouterEvent: SygmaFeeHandlerRouterEvent;
    SygmaFeeHandlerRouterFeeHandlerType: SygmaFeeHandlerRouterFeeHandlerType;
    SygmaPercentageFeehandlerCall: SygmaPercentageFeehandlerCall;
    SygmaPercentageFeehandlerError: SygmaPercentageFeehandlerError;
    SygmaPercentageFeehandlerEvent: SygmaPercentageFeehandlerEvent;
    SygmaTraitsMpcAddress: SygmaTraitsMpcAddress;
    SygmaTraitsTransferType: SygmaTraitsTransferType;
    TangleCryptoPrimitivesCryptoPublic: TangleCryptoPrimitivesCryptoPublic;
    TanglePrimitivesJobsFallbackOptions: TanglePrimitivesJobsFallbackOptions;
    TanglePrimitivesJobsJobInfo: TanglePrimitivesJobsJobInfo;
    TanglePrimitivesJobsJobResult: TanglePrimitivesJobsJobResult;
    TanglePrimitivesJobsJobSubmission: TanglePrimitivesJobsJobSubmission;
    TanglePrimitivesJobsJobType: TanglePrimitivesJobsJobType;
    TanglePrimitivesJobsPhaseResult: TanglePrimitivesJobsPhaseResult;
    TanglePrimitivesJobsTssDigitalSignatureScheme: TanglePrimitivesJobsTssDigitalSignatureScheme;
    TanglePrimitivesJobsTssDkgtssKeyRefreshResult: TanglePrimitivesJobsTssDkgtssKeyRefreshResult;
    TanglePrimitivesJobsTssDkgtssKeyRotationResult: TanglePrimitivesJobsTssDkgtssKeyRotationResult;
    TanglePrimitivesJobsTssDkgtssKeySubmissionResult: TanglePrimitivesJobsTssDkgtssKeySubmissionResult;
    TanglePrimitivesJobsTssDkgtssPhaseFourJobType: TanglePrimitivesJobsTssDkgtssPhaseFourJobType;
    TanglePrimitivesJobsTssDkgtssPhaseOneJobType: TanglePrimitivesJobsTssDkgtssPhaseOneJobType;
    TanglePrimitivesJobsTssDkgtssPhaseThreeJobType: TanglePrimitivesJobsTssDkgtssPhaseThreeJobType;
    TanglePrimitivesJobsTssDkgtssPhaseTwoJobType: TanglePrimitivesJobsTssDkgtssPhaseTwoJobType;
    TanglePrimitivesJobsTssDkgtssSignatureResult: TanglePrimitivesJobsTssDkgtssSignatureResult;
    TanglePrimitivesJobsV2ApprovalPrefrence: TanglePrimitivesJobsV2ApprovalPrefrence;
    TanglePrimitivesJobsV2ApprovalState: TanglePrimitivesJobsV2ApprovalState;
    TanglePrimitivesJobsV2Architecture: TanglePrimitivesJobsV2Architecture;
    TanglePrimitivesJobsV2ContainerGadget: TanglePrimitivesJobsV2ContainerGadget;
    TanglePrimitivesJobsV2Field: TanglePrimitivesJobsV2Field;
    TanglePrimitivesJobsV2FieldFieldType: TanglePrimitivesJobsV2FieldFieldType;
    TanglePrimitivesJobsV2Gadget: TanglePrimitivesJobsV2Gadget;
    TanglePrimitivesJobsV2GadgetBinary: TanglePrimitivesJobsV2GadgetBinary;
    TanglePrimitivesJobsV2GithubFetcher: TanglePrimitivesJobsV2GithubFetcher;
    TanglePrimitivesJobsV2ImageRegistryFetcher: TanglePrimitivesJobsV2ImageRegistryFetcher;
    TanglePrimitivesJobsV2JobCall: TanglePrimitivesJobsV2JobCall;
    TanglePrimitivesJobsV2JobCallResult: TanglePrimitivesJobsV2JobCallResult;
    TanglePrimitivesJobsV2JobDefinition: TanglePrimitivesJobsV2JobDefinition;
    TanglePrimitivesJobsV2JobMetadata: TanglePrimitivesJobsV2JobMetadata;
    TanglePrimitivesJobsV2JobResultVerifier: TanglePrimitivesJobsV2JobResultVerifier;
    TanglePrimitivesJobsV2NativeGadget: TanglePrimitivesJobsV2NativeGadget;
    TanglePrimitivesJobsV2OperatingSystem: TanglePrimitivesJobsV2OperatingSystem;
    TanglePrimitivesJobsV2OperatorPreferences: TanglePrimitivesJobsV2OperatorPreferences;
    TanglePrimitivesJobsV2OperatorProfile: TanglePrimitivesJobsV2OperatorProfile;
    TanglePrimitivesJobsV2Service: TanglePrimitivesJobsV2Service;
    TanglePrimitivesJobsV2ServiceBlueprint: TanglePrimitivesJobsV2ServiceBlueprint;
    TanglePrimitivesJobsV2ServiceMetadata: TanglePrimitivesJobsV2ServiceMetadata;
    TanglePrimitivesJobsV2ServiceRegistrationHook: TanglePrimitivesJobsV2ServiceRegistrationHook;
    TanglePrimitivesJobsV2ServiceRequest: TanglePrimitivesJobsV2ServiceRequest;
    TanglePrimitivesJobsV2ServiceRequestHook: TanglePrimitivesJobsV2ServiceRequestHook;
    TanglePrimitivesJobsV2TypeCheckError: TanglePrimitivesJobsV2TypeCheckError;
    TanglePrimitivesJobsV2WasmGadget: TanglePrimitivesJobsV2WasmGadget;
    TanglePrimitivesJobsValidatorOffenceType: TanglePrimitivesJobsValidatorOffenceType;
    TanglePrimitivesJobsZksaasArkworksProofResult: TanglePrimitivesJobsZksaasArkworksProofResult;
    TanglePrimitivesJobsZksaasCircomProofResult: TanglePrimitivesJobsZksaasCircomProofResult;
    TanglePrimitivesJobsZksaasGroth16ProveRequest: TanglePrimitivesJobsZksaasGroth16ProveRequest;
    TanglePrimitivesJobsZksaasGroth16System: TanglePrimitivesJobsZksaasGroth16System;
    TanglePrimitivesJobsZksaasHyperData: TanglePrimitivesJobsZksaasHyperData;
    TanglePrimitivesJobsZksaasQapShare: TanglePrimitivesJobsZksaasQapShare;
    TanglePrimitivesJobsZksaasZkSaaSCircuitResult: TanglePrimitivesJobsZksaasZkSaaSCircuitResult;
    TanglePrimitivesJobsZksaasZkSaaSPhaseOneJobType: TanglePrimitivesJobsZksaasZkSaaSPhaseOneJobType;
    TanglePrimitivesJobsZksaasZkSaaSPhaseTwoJobType: TanglePrimitivesJobsZksaasZkSaaSPhaseTwoJobType;
    TanglePrimitivesJobsZksaasZkSaaSPhaseTwoRequest: TanglePrimitivesJobsZksaasZkSaaSPhaseTwoRequest;
    TanglePrimitivesJobsZksaasZkSaaSProofResult: TanglePrimitivesJobsZksaasZkSaaSProofResult;
    TanglePrimitivesJobsZksaasZkSaaSSystem: TanglePrimitivesJobsZksaasZkSaaSSystem;
    TanglePrimitivesMisbehaviorDfnsCggmp21DfnsCGGMP21Justification: TanglePrimitivesMisbehaviorDfnsCggmp21DfnsCGGMP21Justification;
    TanglePrimitivesMisbehaviorDfnsCggmp21InvalidProofReason: TanglePrimitivesMisbehaviorDfnsCggmp21InvalidProofReason;
    TanglePrimitivesMisbehaviorDfnsCggmp21KeyRefreshAborted: TanglePrimitivesMisbehaviorDfnsCggmp21KeyRefreshAborted;
    TanglePrimitivesMisbehaviorDfnsCggmp21KeygenAborted: TanglePrimitivesMisbehaviorDfnsCggmp21KeygenAborted;
    TanglePrimitivesMisbehaviorDfnsCggmp21SigningAborted: TanglePrimitivesMisbehaviorDfnsCggmp21SigningAborted;
    TanglePrimitivesMisbehaviorDkgtssJustification: TanglePrimitivesMisbehaviorDkgtssJustification;
    TanglePrimitivesMisbehaviorMisbehaviorJustification: TanglePrimitivesMisbehaviorMisbehaviorJustification;
    TanglePrimitivesMisbehaviorMisbehaviorSubmission: TanglePrimitivesMisbehaviorMisbehaviorSubmission;
    TanglePrimitivesMisbehaviorSignedRoundMessage: TanglePrimitivesMisbehaviorSignedRoundMessage;
    TanglePrimitivesMisbehaviorZcashFrostKeygenAborted: TanglePrimitivesMisbehaviorZcashFrostKeygenAborted;
    TanglePrimitivesMisbehaviorZcashFrostSigningAborted: TanglePrimitivesMisbehaviorZcashFrostSigningAborted;
    TanglePrimitivesMisbehaviorZcashFrostZCashFrostJustification: TanglePrimitivesMisbehaviorZcashFrostZCashFrostJustification;
    TanglePrimitivesMisbehaviorZkSaaSJustification: TanglePrimitivesMisbehaviorZkSaaSJustification;
    TanglePrimitivesRolesRoleType: TanglePrimitivesRolesRoleType;
    TanglePrimitivesRolesTssThresholdSignatureRoleType: TanglePrimitivesRolesTssThresholdSignatureRoleType;
    TanglePrimitivesRolesZksaasZeroKnowledgeRoleType: TanglePrimitivesRolesZksaasZeroKnowledgeRoleType;
    TangleTestnetRuntimeMaxAdditionalParamsLen: TangleTestnetRuntimeMaxAdditionalParamsLen;
    TangleTestnetRuntimeMaxDataLen: TangleTestnetRuntimeMaxDataLen;
    TangleTestnetRuntimeMaxKeyLen: TangleTestnetRuntimeMaxKeyLen;
    TangleTestnetRuntimeMaxParticipants: TangleTestnetRuntimeMaxParticipants;
    TangleTestnetRuntimeMaxProofLen: TangleTestnetRuntimeMaxProofLen;
    TangleTestnetRuntimeMaxSignatureLen: TangleTestnetRuntimeMaxSignatureLen;
    TangleTestnetRuntimeMaxSubmissionLen: TangleTestnetRuntimeMaxSubmissionLen;
    TangleTestnetRuntimeNposSolution16: TangleTestnetRuntimeNposSolution16;
    TangleTestnetRuntimeOpaqueSessionKeys: TangleTestnetRuntimeOpaqueSessionKeys;
    TangleTestnetRuntimeOriginCaller: TangleTestnetRuntimeOriginCaller;
    TangleTestnetRuntimeProxyType: TangleTestnetRuntimeProxyType;
    TangleTestnetRuntimeRuntime: TangleTestnetRuntimeRuntime;
    TangleTestnetRuntimeRuntimeFreezeReason: TangleTestnetRuntimeRuntimeFreezeReason;
    TangleTestnetRuntimeRuntimeHoldReason: TangleTestnetRuntimeRuntimeHoldReason;
    XcmV3JunctionBodyId: XcmV3JunctionBodyId;
    XcmV3JunctionBodyPart: XcmV3JunctionBodyPart;
  } // InterfaceTypes
} // declare module
