// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract RelayerConsensus {
    // State variables
    uint256 public minStake = 1 ether;
    uint256 public slashingPenalty = 0.5 ether;
    uint256 public rewardAmount = 0.1 ether;

    struct Relayer {
        uint256 stake;
        bool isRegistered;
    }

    mapping(address => Relayer) public relayers;
    mapping(bytes32 => bool) public messageHashes;  // Track relayed message hashes

    // Events
    event RelayerRegistered(address indexed relayer, uint256 stake);
    event RelayerSlashed(address indexed relayer, uint256 penalty);
    event MessageRelayed(address indexed relayer, bytes32 indexed messageHash, uint256 reward);

    // Register a relayer with staking
    function registerRelayer() public payable {
        require(msg.value >= minStake, "Insufficient stake");
        require(!relayers[msg.sender].isRegistered, "Already registered");

        relayers[msg.sender] = Relayer(msg.value, true);
        emit RelayerRegistered(msg.sender, msg.value);
    }

    // Submit proof of relaying a message
    function submitProof(bytes32 messageHash) public {
        require(relayers[msg.sender].isRegistered, "Not a registered relayer");
        require(!messageHashes[messageHash], "Message already relayed");

        // Mark the message as relayed
        messageHashes[messageHash] = true;

        // Reward the relayer
        payable(msg.sender).transfer(rewardAmount);
        emit MessageRelayed(msg.sender, messageHash, rewardAmount);
    }

    // Slash a relayer for malicious behavior
    function slashRelayer(address relayer) public {
        require(relayers[relayer].isRegistered, "Not a registered relayer");

        // Penalize the relayer by slashing a portion of their stake
        relayers[relayer].stake -= slashingPenalty;
        payable(msg.sender).transfer(slashingPenalty);
        emit RelayerSlashed(relayer, slashingPenalty);

        // If stake falls below minimum, deregister the relayer
        if (relayers[relayer].stake < minStake) {
            relayers[relayer].isRegistered = false;
        }
    }
}
