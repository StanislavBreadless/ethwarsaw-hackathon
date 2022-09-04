// Saves logs for ETH transfers

import {client} from './index.js'
import { utils } from 'ethers'

export const ethEventsSave = async (logs) => {
    for(const log of logs) {
        console.log(`Saving log ${log}`);

        const contract = log.address;
        const topic0 = log.topics[0];
        const topic1 = log.topics[1];
        const data = log.data;

        const fullData = utils.hexConcat([contract, topic0, topic1, data]);

        const collection = client.db("bridge").collection("logs");
        collection.insertOne({receiver: topic1, full_data: fullData})
    }
}