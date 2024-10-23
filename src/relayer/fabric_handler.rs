package main

import (
	"fmt"
	"github.com/hyperledger/fabric-sdk-go/pkg/client/channel"
	"github.com/hyperledger/fabric-sdk-go/pkg/fabsdk"
	"github.com/hyperledger/fabric-sdk-go/pkg/gateway"
)

type Relayer struct {
	client *channel.Client
}

// NewRelayer initializes a new relayer instance connected to the Fabric network
func NewRelayer(configPath string, channelID string) (*Relayer, error) {
	sdk, err := fabsdk.New(configPath)
	if err != nil {
		return nil, fmt.Errorf("Failed to create Fabric SDK: %s", err.Error())
	}

	clientContext := sdk.ChannelContext(channelID)
	client, err := channel.New(clientContext)
	if err != nil {
		return nil, fmt.Errorf("Failed to create channel client: %s", err.Error())
	}

	return &Relayer{client: client}, nil
}

// SubmitMessage submits a cross-chain message to Fabric chaincode
func (r *Relayer) SubmitMessage(id string, content string, recipient string) error {
	messageArgs := [][]byte{[]byte("SubmitMessage"), []byte(id), []byte(content), []byte(recipient)}
	response, err := r.client.Execute(channel.Request{
		ChaincodeID: "message_chaincode", // Chaincode name
		Fcn:         "SubmitMessage",
		Args:        messageArgs,
	})
	if err != nil {
		return fmt.Errorf("Failed to submit message: %s", err.Error())
	}

	fmt.Printf("Message submitted successfully: %s\n", response.TransactionID)
	return nil
}

// CreateCheckpoint creates a message hash checkpoint in Fabric
func (r *Relayer) CreateCheckpoint(id string) error {
	args := [][]byte{[]byte("CreateCheckpoint"), []byte(id)}
	response, err := r.client.Execute(channel.Request{
		ChaincodeID: "message_chaincode",
		Fcn:         "CreateCheckpoint",
		Args:        args,
	})
	if err != nil {
		return fmt.Errorf("Failed to create checkpoint: %s", err.Error())
	}

	fmt.Printf("Checkpoint created with hash: %s\n", string(response.Payload))
	return nil
}


func (r *Relayer) SubmitToFabric(messageID string, content string, recipient string) error {
    client := getFabricClient() // Function to initialize Fabric client using SDK
    args := [][]byte{[]byte("SubmitMessage"), []byte(messageID), []byte(content), []byte(recipient)}
    response, err := client.Execute(channel.Request{
        ChaincodeID: "message_chaincode",
        Fcn:         "SubmitMessage",
        Args:        args,
    })
    if err != nil {
        return fmt.Errorf("Failed to submit message: %s", err.Error())
    }

    fmt.Printf("Message submitted successfully with ID: %s\n", messageID)
    return nil
}


func (r *Relayer) CreateCheckpointOnEthereum(messageHash string) error {
    tx := r.ethClient.CallContractMethod("MessageAuditor", "createCheckpoint", messageHash)
    if err := tx.WaitForConfirmation(); err != nil {
        return fmt.Errorf("Failed to create checkpoint on Ethereum: %s", err.Error())
    }

    fmt.Printf("Checkpoint created on Ethereum for message hash: %s\n", messageHash)
    return nil
}

// Relay a batch of messages from Fabric to Ethereum
func (r *Relayer) RelayMessageBatch(messages []Message) error {
    for _, message := range messages {
        err := r.SubmitToFabric(message.ID, message.Content, message.Recipient)
        if err != nil {
            return fmt.Errorf("Failed to submit message %s to Fabric: %s", message.ID, err.Error())
        }
        messageHash := generateMessageHash(message.Content)
        err = r.CreateCheckpointOnEthereum(messageHash)
        if err != nil {
            return fmt.Errorf("Failed to create checkpoint on Ethereum: %s", err.Error())
        }
    }
    return nil
}


func RelayMessageQueue(messages []Message) {
    sort.SliceStable(messages, func(i, j int) bool {
        return messages[i].Priority > messages[j].Priority // Higher priority first
    })

    for _, message := range messages {
        processMessage(message)
    }
}

func VerifyMerkleProof(leafHash []byte, proof [][]byte, rootHash []byte) bool {
    hash := leafHash
    for _, siblingHash := range proof {
        hash = appendHashes(hash, siblingHash)
    }
    return bytes.Equal(hash, rootHash)
}