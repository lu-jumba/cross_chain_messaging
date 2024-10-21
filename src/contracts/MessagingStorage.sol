 //SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract MessagingStorage {

    // Event to emit when a new message hash is stored
    event MessageStored(address indexed sender, string ipfsHash, uint256 timestamp);

    // Struct to hold message details
    struct Message {
        string ipfsHash;
        uint256 timestamp;
    }

    // Mapping of sender address to list of messages
    mapping(address => Message[]) private messages;

    // Function to store a message hash
    function storeMessageHash(string memory ipfsHash) public {
        // Create a new message and push it to the sender's message list
        messages[msg.sender].push(Message(ipfsHash, block.timestamp));

        // Emit the MessageStored event
        emit MessageStored(msg.sender, ipfsHash, block.timestamp);
    }

    // Function to retrieve all message hashes for a given address
    function getMessages(address user) public view returns (Message[] memory) {
        return messages[user];
    }

    // Function to retrieve the number of messages for a given address
    function getMessageCount(address user) public view returns (uint256) {
        return messages[user].length;
    }

    // Event for cross-chain messaging
    event CrossChainMessage(address indexed sender, string ipfsHash, string targetChain, address targetRecipient, uint256 timestamp);

    struct Message {
        string ipfsHash;
        uint256 timestamp;
    }

    mapping(address => Message[]) private messages;

    // Store the message on the EVM chain and emit an event for cross-chain relaying
    function storeMessageHash(string memory ipfsHash, string memory targetChain, address targetRecipient) public {
        messages[msg.sender].push(Message(ipfsHash, block.timestamp));

        // Emit cross-chain message event
        emit CrossChainMessage(msg.sender, ipfsHash, targetChain, targetRecipient, block.timestamp);
    }

    // Function to retrieve stored messages for a user (for verification)
    function getMessages(address user) public view returns (Message[] memory) {
        return messages[user];
    }
}
