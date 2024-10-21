# Blockchain-Based Messaging Platform in Rust

#Project Structure

## Overview

This project is a **Rust-based blockchain messaging platform** designed for secure, cross-chain communication using blockchain technologies like **Ethereum (EVM)**, **Substrate-based chains (Polkadot)**, and **Hyperledger Fabric**. The platform features:

- **End-to-end encrypted messaging** using wallet-based signatures.

- **Multi-chain relayer service** to relay signed messages across different blockchain networks.

- **Web-based UI (Yew)** to allow users to compose and relay messages across chains.

---

## Key Features

1. **Cross-Chain Messaging**: Relay messages across **Ethereum**, **Substrate**, and **Hyperledger Fabric** blockchains.

2. **Wallet-Based Messaging**: Messages are signed using **local wallets** or **MetaMask** before submission to the relayer.

3. **Decentralized Storage**: Encrypted messages are stored in decentralized storage (IPFS, Arweave).

4. **Yew UI**: A web-based user interface built using **Yew** to interact with the platform.
 
5. **Multi-Signature Governance**: Multi-sig governance for critical actions (if integrated).

---

## Project Structure

The project is structured as follows:

- **src/**: Core Rust source code for relayers, blockchain interactions, wallet management, and Yew UI.

- **relayer/**: Contains relayer logic for each blockchain (Ethereum, Substrate, and Hyperledger Fabric).

- **wallet/**: Handles wallet generation, signing, and verification of messages.

- **ui/**: Yew-based web UI to compose, sign, and relay messages.

- **config/**: Configuration files for connecting to Ethereum, Substrate, and Fabric.

- **tests/**: Unit and integration tests for various components (relayers, wallets, etc.).

- **scripts/**: Deployment and testing scripts for blockchain networks and relayers.

---

## Prerequisites

Ensure you have the following installed:

- **Rust**: To build and run the project.

- **Substrate Node**: For Substrate-based chain relaying.

- **Hyperledger Fabric**: To deploy the Fabric chaincode and interact with the ledger.

- **Ethereum Node (Infura or Local)**: To relay messages on Ethereum.

- **Docker**: For managing Fabric and Ethereum services if running locally.

- **Trunk**: For serving the Yew frontend.

---

## Installation and Setup

1. **Clone the Repository**:

    ```bash
    
    git clone https://github.com/your-repo/blockchain-messaging
    
    cd blockchain-messaging
    ```

2. **Install Dependencies**:

    ```bash
    
    cargo build
    
    ```

3. **Configure Blockchains**:

    Update the `config/fabric_connection.yaml`, `config/ethereum_config.json`, and `config/substrate_config.json` files with your connection details.

4. **Deploy Hyperledger Fabric Chaincode**:

    ```bash
    
    ./scripts/deploy_chaincode.sh
    
    ```

5. **Deploy Ethereum Smart Contracts** (if needed):

    ```bash
    
    ./scripts/deploy_ethereum_contract.sh
    
    ```

5. **Start the Backend**:

    ```bash
    
    cargo run
    
    ```

6. **Start the Yew UI**:

    ```bash
    
    trunk serve
    
    ```

---

## Usage

1. **Compose and Sign Messages**:

    - Open the browser at **`localhost:8080`**.
   
    - Use the **Yew UI** to compose messages.
   
    - Select a blockchain (Ethereum, Substrate, Fabric) to relay your signed message.

2. **Relaying Messages**:

    - The message will be signed using your selected wallet (MetaMask or local).
   
    - The signed message is then submitted to the **backend relayer service**, which relays the message across the selected blockchain.

---

## Relayer Services

The **relayer** is responsible for submitting signed messages across multiple blockchain networks. Supported blockchains:

- **Ethereum**: Using **ethers-rs** to submit the message as a transaction.

- **Substrate**: Using **substrate-api-client** to submit an extrinsic to a Substrate chain.

- **Hyperledger Fabric**: Using a **REST API** or **gRPC** to invoke the chaincode and store the message on the ledger.

---

## Running Tests

The project includes unit and integration tests for the relayers, wallet logic, and UI.

1. **Run All Tests**:

    ```bash
    
    ./scripts/run_tests.sh
    
    ```

2. **Test Results**:

    Results will be displayed on the terminal. Ensure the relayers work as expected for all supported blockchains.

---

TO DO:

1. Testing and Validation: Ensure that all the relayers and the UI work as expected across the blockchains (Ethereum, Substrate, and Fabric).

2. Documentation Review: Finalize the documentation for SDKs, APIs, and deployment instructions for different environments.

## License

This project is licensed under the MIT License. See `LICENSE` for more details.
