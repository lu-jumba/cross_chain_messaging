async function main() {
    const Governance = await ethers.getContractFactory("RelayerGovernance");
    const governance = await Governance.deploy();
    await governance.deployed();
    console.log("RelayerGovernance deployed to:", governance.address);
  }
  
  main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });
  