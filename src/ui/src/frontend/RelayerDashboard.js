import React, { useState, useEffect } from 'react';
import Web3 from 'web3';

function RelayerDashboard() {
    const [relayers, setRelayers] = useState([]);
    const [rewards, setRewards] = useState(0);

    useEffect(() => {
        // Fetch relayer data from smart contract
        async function fetchRelayerData() {
            const web3 = new Web3(Web3.givenProvider || 'http://localhost:8545');
            const contract = new web3.eth.Contract(RelayerGovernanceABI, contractAddress);
            const relayersData = await contract.methods.getRelayers().call();
            setRelayers(relayersData);
        }
        fetchRelayerData();
    }, []);

    return (
        <div>
            <h1>Relayer Dashboard</h1>
            <h2>Total Rewards: {rewards} ETH</h2>
            <h3>Active Relayers:</h3>
            <ul>
                {relayers.map((relayer, index) => (
                    <li key={index}>Relayer: {relayer.address} - Stake: {relayer.stake} ETH</li>
                ))}
            </ul>
        </div>
    );
}

export default RelayerDashboard;
