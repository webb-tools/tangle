// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "../src/SigningRules.sol";
import { Proposal, ProposalStatus } from "../src/SigningRules.sol";


contract VotableSigningRules is SigningRules {
    function _isVotableProposal(uint64 phase1JobId, bytes memory phase1JobDetails, bytes memory phase2JobDetails) override pure internal returns (bool) {
        require(phase1JobDetails.length != 0, "Job details must be non-empty");
        require(phase2JobDetails.length != 0, "Job details must be non-empty");
        return true;
    }

    function _refreshVoters(bytes32 proposalId) override internal {
        // Do nothing
    }

    function _submitToDemocracyPallet(uint64 phase1JobId, bytes memory phase1JobDetails, bytes memory phase2JobDetails) override internal {
        // Do nothing
    }
}

contract SigningRulesTest is Test {
    VotableSigningRules public rules;

    function setUp() public {
        rules = new VotableSigningRules();
    }

    function test_setup() public {
        uint64 phase1JobId = 1;
        bytes memory phase1JobDetails = "test";
        uint8 threshold = 1;
        bool useDemocracy = false;
        address[] memory voters = new address[](0);
        uint64 expiry = 1000;
        uint64 ttl = 1000;
        bytes32 proposalId = rules.calculatePhase1ProposalId(phase1JobId, phase1JobDetails);
        rules.initialize(phase1JobId, phase1JobDetails, threshold, useDemocracy, voters, expiry, ttl);
        assertTrue(rules.initialized());
        assertTrue(rules.threshold(proposalId) == threshold);
        assertTrue(rules.useDemocracy(proposalId) == useDemocracy);
        assertTrue(rules.expiry(proposalId) == expiry);
        assertTrue(rules.useValidators(proposalId) == true);
        assertTrue(rules.admins(proposalId) == address(this));
    }

    function test_submitAndVoteOnProposal() public {
        uint64 phase1JobId = 1;
        bytes memory phase1JobDetails = "test";
        bytes memory phase2JobDetails = "test";
        uint8 threshold = 2;
        bool useDemocracy = false;
        address[] memory voters = new address[](2);
        voters[0] = vm.addr(1);
        voters[1] = vm.addr(2);
        uint40 expiry = 1000;
        uint64 ttl = 1000;
        bytes32 phase1ProposalId = rules.calculatePhase1ProposalId(phase1JobId, phase1JobDetails);
        bytes32 phase2JobHash = rules.calculatePhase2JobHash(phase1ProposalId, phase2JobDetails);

        rules.initialize(phase1JobId, phase1JobDetails, threshold, useDemocracy, voters, expiry, ttl);
        vm.prank(vm.addr(1));
        rules.voteProposal(phase1JobId, phase1JobDetails, phase2JobDetails);
        assertTrue(rules.getProposalState(phase2JobHash) == ProposalStatus.Active);

        vm.expectRevert("relayer already voted");
        vm.prank(vm.addr(1));
        rules.voteProposal(phase1JobId, phase1JobDetails, phase2JobDetails);

        vm.prank(vm.addr(2));
        rules.voteProposal(phase1JobId, phase1JobDetails, phase2JobDetails);
        assertTrue(rules.getProposalState(phase2JobHash) == ProposalStatus.Executed);
    }

    function test_submitAndVote255Participants() public {
        uint64 phase1JobId = 1;
        bytes memory phase1JobDetails = "test";
        bytes memory phase2JobDetails = "test";
        uint8 threshold = 255;
        bool useDemocracy = false;
        address[] memory voters = new address[](255);
        for (uint8 i = 0; i < 255; i++) {
            voters[i] = vm.addr(i + 1);
        }
        uint40 expiry = 1000;
        uint64 ttl = 1000;
        bytes32 phase1ProposalId = rules.calculatePhase1ProposalId(phase1JobId, phase1JobDetails);
        bytes32 phase2JobHash = rules.calculatePhase2JobHash(phase1ProposalId, phase2JobDetails);
        rules.initialize(phase1JobId, phase1JobDetails, threshold, useDemocracy, voters, expiry, ttl);
        for (uint8 i = 0; i < 255; i++) {
            vm.prank(vm.addr(i + 1));
            rules.voteProposal(phase1JobId, phase1JobDetails, phase2JobDetails);

            if (i < 254) {
                assertTrue(rules.getProposalState(phase2JobHash) == ProposalStatus.Active);
            }
        }
        assertTrue(rules.getProposalState(phase2JobHash) == ProposalStatus.Executed);
    }

    function test_submitVoteAndExpireProposal() public {
        uint64 phase1JobId = 1;
        bytes memory phase1JobDetails = "test";
        bytes memory phase2JobDetails = "test";
        uint8 threshold = 2;
        bool useDemocracy = false;
        address[] memory voters = new address[](2);
        voters[0] = vm.addr(1);
        voters[1] = vm.addr(2);
        uint40 expiry = 10;
        uint64 ttl = 10;
        uint nowBlockNumber = block.number;
        bytes32 phase1ProposalId = rules.calculatePhase1ProposalId(phase1JobId, phase1JobDetails);
        bytes32 phase2JobHash = rules.calculatePhase2JobHash(phase1ProposalId, phase2JobDetails);

        rules.initialize(phase1JobId, phase1JobDetails, threshold, useDemocracy, voters, expiry, ttl);
        vm.prank(vm.addr(1));
        rules.voteProposal(phase1JobId, phase1JobDetails, phase2JobDetails);
        assertTrue(rules.getProposalState(phase2JobHash) == ProposalStatus.Active);

        vm.roll(nowBlockNumber + expiry + 1);
        vm.prank(vm.addr(2));
        rules.voteProposal(phase1JobId, phase1JobDetails, phase2JobDetails);
        assertTrue(rules.getProposalState(phase2JobHash) == ProposalStatus.Cancelled);
        vm.expectRevert("proposal already executed/cancelled");
        vm.prank(vm.addr(2));
        rules.voteProposal(phase1JobId, phase1JobDetails, phase2JobDetails);
    }

    function test_adminFunctions() public {
        uint64 phase1JobId = 1;
        bytes memory phase1JobDetails = "test";
        uint8 threshold = 2;
        bool useDemocracy = false;
        address[] memory voters = new address[](2);
        voters[0] = vm.addr(1);
        voters[1] = vm.addr(2);
        uint40 expiry = 1000;
        uint64 ttl = 1000;
        bytes32 phase1ProposalId = rules.calculatePhase1ProposalId(phase1JobId, phase1JobDetails);
        rules.initialize(phase1JobId, phase1JobDetails, threshold, useDemocracy, voters, expiry, ttl);
        
        rules.adminSetForwarder(phase1ProposalId, vm.addr(100), true);
        assertTrue(rules.isValidForwarder(phase1ProposalId, vm.addr(100)));
        assertFalse(rules.isValidForwarder(phase1ProposalId, vm.addr(101)));

        vm.expectRevert("Only admin can call this function");
        vm.prank(vm.addr(1));
        rules.adminSetForwarder(phase1ProposalId, vm.addr(100), false);
    }
}
