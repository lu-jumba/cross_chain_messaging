require("@nomiclabs/hardhat-ethers");

module.exports = {
  networks: {
    mainnet: {
      url: `https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID`, // Ethereum Mainnet
      accounts: [`0x${YOUR_PRIVATE_KEY}`],
    },
    bscMainnet: {
      url: `https://bsc-dataseed.binance.org/`, // Binance Smart Chain Mainnet
      accounts: [`0x${YOUR_PRIVATE_KEY}`],
    },
  },
  solidity: "0.8.0",
};
