const Web3 = require('web3');
const contractABI = require('./MessageAuditorABI.json');
const contractAddress = '0x...'; // Deployed contract address

class RelayerSDK {
    constructor(ethereumUrl) {
        this.web3 = new Web3(new Web3.providers.HttpProvider(ethereumUrl));
        this.contract = new this.web3.eth.Contract(contractABI, contractAddress);
    }

    async createCheckpoint(messageHash, fromAddress, privateKey) {
        const tx = this.contract.methods.createCheckpoint(messageHash);
        const signedTx = await this.web3.eth.accounts.signTransaction(tx, privateKey);
        return this.web3.eth.sendSignedTransaction(signedTx.rawTransaction);
    }

    async validateCheckpoint(messageHash) {
        return this.contract.methods.validateCheckpoint(messageHash).call();
    }
}

module.exports = RelayerSDK;
