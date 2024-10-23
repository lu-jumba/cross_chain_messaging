// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract RelayerGovernance {
    struct Proposal {
        uint256 id;
        string description;
        uint256 newStakeAmount;
        uint256 newRewardAmount;
        uint256 votesFor;
        uint256 votesAgainst;
        uint256 endTime;
        uint256 executionTime; // Time when the proposal can be executed
        bool executed;
    }

    struct Relayer {
        uint256 stake;
        uint256 uptime;
        uint256 messagesRelayed;
        bool isRegistered;
    }

    mapping(uint256 => Proposal) public proposals;
    uint256 public nextProposalId;
    mapping(address => Relayer) public relayers;
    mapping(address => uint256) public stakes;

    uint256 public stakeAmount = 1 ether;
    uint256 public rewardAmount = 0.1 ether;
    uint256 public executionDelay = 2 days; // Time lock for proposal execution

    event ProposalCreated(uint256 proposalId, string description);
    event Voted(uint256 proposalId, address voter, bool vote);
    event ProposalExecuted(uint256 proposalId);
    event RelayerRewarded(address relayer, uint256 reward);
    
    // Create a new proposal
    function createProposal(string memory description, uint256 newStakeAmount, uint256 newRewardAmount) public {
        proposals[nextProposalId] = Proposal({
            id: nextProposalId,
            description: description,
            newStakeAmount: newStakeAmount,
            newRewardAmount: newRewardAmount,
            votesFor: 0,
            votesAgainst: 0,
            endTime: block.timestamp + 7 days,  // Voting period of 7 days
            executionTime: block.timestamp + 7 days + executionDelay,  // Time lock after voting ends
            executed: false
        });
        emit ProposalCreated(nextProposalId, description);
        nextProposalId++;
    }

    // Vote for or against a proposal
    function vote(uint256 proposalId, bool support) public {
        require(proposals[proposalId].endTime > block.timestamp, "Voting period has ended");

        if (support) {
            proposals[proposalId].votesFor += stakes[msg.sender];
        } else {
            proposals[proposalId].votesAgainst += stakes[msg.sender];
        }
        emit Voted(proposalId, msg.sender, support);
    }

    // Execute the proposal after the time lock
    function executeProposal(uint256 proposalId) public {
        Proposal storage proposal = proposals[proposalId];
        require(block.timestamp > proposal.executionTime, "Time lock not expired");
        require(!proposal.executed, "Proposal already executed");

        if (proposal.votesFor > proposal.votesAgainst) {
            stakeAmount = proposal.newStakeAmount;
            rewardAmount = proposal.newRewardAmount;
        }

        proposal.executed = true;
        emit ProposalExecuted(proposalId);
    }

    // Register relayer with staking
    function stakeTokens() public payable {
        require(msg.value >= 0.1 ether, "Minimum stake of 0.1 ETH required");
        stakes[msg.sender] += msg.value;
        relayers[msg.sender] = Relayer({
            stake: msg.value,
            uptime: 0,
            messagesRelayed: 0,
            isRegistered: true
        });
    }

    // Calculate and distribute rewards based on performance
    function distributeRewards() public {
        for (uint256 i = 0; i < nextProposalId; i++) {
            address relayer = address(uint160(i));
            uint256 reward = calculateReward(relayer);
            payable(relayer).transfer(reward);
            emit RelayerRewarded(relayer, reward);
        }
    }

    // Calculate relayer rewards based on uptime and messages relayed
    function calculateReward(address relayer) internal view returns (uint256) {
        Relayer memory r = relayers[relayer];
        uint256 uptimeBonus = (r.uptime / 100) * 0.05 ether;
        uint256 performanceBonus = (r.messagesRelayed / 100) * 0.05 ether;
        return rewardAmount + uptimeBonus + performanceBonus;
    }
}
