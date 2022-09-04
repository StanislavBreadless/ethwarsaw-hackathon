// Pushes Ethereum Ropsten network blocks to the Aleph Zero test network
import { providers } from 'ethers'
import { client } from './index.js'

const eth = new providers.JsonRpcProvider('https://ropsten.infura.io/v3/[infura_project_id]')

const blockNumberDocumentId = '6313f8d8f561e4628058ca91'

export const getNewBlocks = async () => {
    const collection = client.db("bridge").collection("block_id");
    const lastProcessedBlock = collection.findOne({_id: blockNumberDocumentId}).lastBlock;

    let currentBlock = eth.blockNumber;

    collection.updateOne({_id: blockNumberDocumentId}, { $set: {
        lastBlock: currentBlock
    }});

    let blocks = []
    for (let number = lastProcessedBlock; number <= currentBlock; number++) {
        let block = await eth.getBlock(i);
        let receipts = [];
        for (let txHash in block.transactions) {
            receipts.push(eth.getTransactionReceipt(txHash));
        }
        block.receipts = receipts;
    }

    return blocks;
}