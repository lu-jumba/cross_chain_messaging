package main

import (
	"encoding/json"
	"fmt"
	"crypto/sha256"
	"github.com/hyperledger/fabric-contract-api-go/contractapi"
)

// Message structure for storing cross-chain messages
type Message struct {
	ID        string `json:"id"`
	Content   string `json:"content"`
	Sender    string `json:"sender"`
	Recipient string `json:"recipient"`
	Timestamp string `json:"timestamp"`
}

// MessageChaincode defines the smart contract for handling messages
type MessageChaincode struct {
	contractapi.Contract
}

// SubmitMessage submits a new message for cross-chain relay
func (m *MessageChaincode) SubmitMessage(ctx contractapi.TransactionContextInterface, id string, content string, recipient string) error {
    // Avoid redundant I/O operations by caching results or using in-memory processing
    messageKey := fmt.Sprintf("msg_%s", id)
    
    existingMessage, err := ctx.GetStub().GetState(messageKey)
    if existingMessage != nil {
        return fmt.Errorf("Message with ID %s already exists", id)
    }

    message := Message{
        ID:        id,
        Content:   content,
        Sender:    "Fabric",
        Recipient: recipient,
        Timestamp: ctx.GetStub().GetTxTimestamp().String(),
    }

    messageJSON, err := json.Marshal(message)
    if err != nil {
        return fmt.Errorf("Failed to marshal message: %s", err.Error())
    }

    return ctx.GetStub().PutState(messageKey, messageJSON)
}

// RetrieveMessage retrieves a message by its ID
func (m *MessageChaincode) RetrieveMessage(ctx contractapi.TransactionContextInterface, id string) (*Message, error) {
	messageJSON, err := ctx.GetStub().GetState(id)
	if err != nil {
		return nil, fmt.Errorf("Failed to get message: %s", err.Error())
	}
	if messageJSON == nil {
		return nil, fmt.Errorf("Message not found")
	}

	var message Message
	err = json.Unmarshal(messageJSON, &message)
	if err != nil {
		return nil, fmt.Errorf("Failed to unmarshal message: %s", err.Error())
	}

	return &message, nil
}

// CreateCheckpoint creates a message hash checkpoint for cross-chain auditing
func (m *MessageChaincode) CreateCheckpoint(ctx contractapi.TransactionContextInterface, id string) (string, error) {
	message, err := m.RetrieveMessage(ctx, id)
	if err != nil {
		return "", err
	}

	messageHash := sha256.Sum256([]byte(message.Content))
	hashString := fmt.Sprintf("%x", messageHash)

	err = ctx.GetStub().PutState(id+"_checkpoint", []byte(hashString))
	if err != nil {
		return "", fmt.Errorf("Failed to store checkpoint: %s", err.Error())
	}

	return hashString, nil
}

func main() {
	chaincode, err := contractapi.NewChaincode(&MessageChaincode{})
	if err != nil {
		fmt.Printf("Error creating MessageChaincode: %s", err.Error())
	}

	if err := chaincode.Start(); err != nil {
		fmt.Printf("Error starting MessageChaincode: %s", err.Error())
	}
}



