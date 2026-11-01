// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

/**
 * @title Governance
 * @notice Governança BFT e registro de deliberações de agentes (OpenCLAW)
 */
contract Governance {
    struct Proposal {
        bytes32 proposalHash;
        uint256 approvals;
        bool executed;
    }

    mapping(bytes32 => Proposal) public proposals;
    mapping(bytes32 => mapping(address => bool)) public hasVoted;

    event ProposalVoted(bytes32 indexed proposalId, address indexed voter, uint256 currentApprovals);
    event ProposalFinalized(bytes32 indexed proposalId);

    function vote(bytes32 proposalId) external {
        require(!hasVoted[proposalId][msg.sender], "Already voted");
        hasVoted[proposalId][msg.sender] = true;

        Proposal storage p = proposals[proposalId];
        p.proposalHash = proposalId;
        p.approvals += 1;
        emit ProposalVoted(proposalId, msg.sender, p.approvals);

        // Quórum de 14 em 20 nós (OpenCLAW BFT > 2/3)
        if (p.approvals >= 14 && !p.executed) {
            p.executed = true;
            emit ProposalFinalized(proposalId);
        }
    }
}
