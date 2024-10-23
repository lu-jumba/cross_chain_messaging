func (m *MessageChaincode) CreateMessageCheckpoint(ctx contractapi.TransactionContextInterface, messageID string) error {
    // Check if the message exists
    message, err := m.GetMessage(ctx, messageID)
    if err != nil {
        return err
    }

    // Create a hash or checkpoint of the message
    messageHash := fmt.Sprintf("%x", sha256.Sum256([]byte(message.Content)))

    // Store the message hash as a checkpoint
    return ctx.GetStub().PutState(messageID+"_checkpoint", []byte(messageHash))
}
