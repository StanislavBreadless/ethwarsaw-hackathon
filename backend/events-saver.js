// Pushes Ethereum Ropsten network blocks to the Aleph Zero test network
import { providers } from 'ethers'
const eth = new providers.JsonRpcProvider('https://ropsten.infura.io/v3/[infura_project_id]')
import {client} from './index.js'

const ETH_BRIDGE_ADDRESS = process.env.ETH_BRIDGE_ADDRESS;

const ETH_BRIDGE = new utils.Interface(require('./eth-bridge-abi.json'));

export const ethEventsSave = async (blocks) => {
    for(const block of blocks) {
        for(const receipt of block.receipts) {
            for(const log of receipt.logs) {
                if (log.address.toLowerCase() == ETH_BRIDGE_ADDRESS.toLowerCase()) {
                    const event = ETH_BRIDGE.parseLog(log);
                    if (event.name != 'AlephMintETH') {
                        continue;
                    }
                    console.log(`Found event ${event}`);
                    
                    const contract = log.address;
                    const topic0 = log.topics[0];
                    const topic1 = log.topics[1];
                    const data = log.data;
                                
                    const fullData = ethers.utils.hexConcat([contract, topic0, topic1, data]);
                    
                    // TODO: Need to save "fullData" to DB
                }
            }
        }
    }
}