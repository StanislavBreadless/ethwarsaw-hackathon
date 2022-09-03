import { ethers } from "hardhat";

const NUM_OF_TESTS = 1;
const OUTPUT_FILE = "tests.json";

async function main() {
  const connectorFactory = await ethers.getContractFactory("AlephConnector");
  const connector: ethers.Contract = await connectorFactory.deploy();

  await connector.deployed();

  console.log(`Contract deployed at ${connector.address}`);

  for(let i = 0; i < NUM_OF_TESTS; i++) {
    const [owner] = await ethers.getSigners();

    const receiver = '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d';

    const receipt = await (
      await connector.lock(receiver, {
        value: ethers.utils.parseEther('1.0')
      })
    ).wait();

    console.log(JSON.stringify(receipt, null, 2));

    const contract = receipt.logs[0].address;
    const topic0 = receipt.logs[0].topics[0];
    const topic1 = receipt.logs[0].topics[1];
    const data = receipt.logs[0].data;

    const fullData = ethers.utils.hexConcat([contract, topic0, topic1, data]);
    console.log(fullData);
  }
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
