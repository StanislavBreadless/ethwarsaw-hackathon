// Pushes Ethereum Ropsten network blocks to the Aleph Zero test network
import { client, eth } from './index.js'
import {ObjectID} from "mongodb";

const blockNumberDocumentId = new ObjectID('631404f5bd44be1c8003e2c6');
const eventSelector = '0x8ab8616cbf81546fc53fe1a6c6566dc7e7f3670dda4afb2da6c03c8d88f4342c'; // keccak256("AlephMintETH(bytes32,uint256)")

export const getNewBlocks = async () => {
    const collection = client.db("bridge").collection("block_id");
    const lastProcessedBlock = (await collection.findOne({_id: blockNumberDocumentId})).lastBlock;

    let currentBlock = await eth.getBlockNumber();

    if (currentBlock === lastProcessedBlock) {
        return [[], []];
    }

    console.log('Last processed block: ' + lastProcessedBlock + ', current block: ' + currentBlock);

    collection.updateOne({_id: blockNumberDocumentId}, { $set: {
        lastBlock: currentBlock
    }});

    let logs = eth.getLogs({
        address: process.env.ETH_BRIDGE_ADDRESS.toLowerCase(),
        topics: [eventSelector],
        fromBlock: lastProcessedBlock + 1,
        toBlock: currentBlock
    });

    let blocks = []

    for (let number = lastProcessedBlock + 1; number <= currentBlock; number++) {
        blocks.push(eth.getBlock(lastProcessedBlock));
    }

    return Promise.all([Promise.all(blocks), logs]);
}