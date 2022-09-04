import { MongoClient, ServerApiVersion } from 'mongodb'
import * as dotenv from 'dotenv'

import {ethBlocksSync} from "./sync.js"
import {ethEventsSave} from "./events-saver.js"
import {getNewBlocks} from "./poll-blocks.js"
import { sleep } from './utils.js';

dotenv.config()
export const client = new MongoClient(process.env.URI, { useNewUrlParser: true, useUnifiedTopology: true, serverApi: ServerApiVersion.v1 });


client.connect(err => {
    const collection = client.db("bridge").collection("test");
    client.close();
});

const POLL_INTERVAL = process.env.POLL_INTERVAL;

async function main() {

    while(true) {
        await sleep(POLL_INTERVAL);

        try {
            console.log('Fetching blocks...');
            const blocks = await getNewBlocks();

            if (blocks.length == 0) {
                continue;
            }
            console.log(`Found ${blocks.length} new blocks`);

            console.log(`Sending new blocks to alef`)
            await ethBlocksSync(blocks)
            console.log(`Updating saved events for users`)
            await ethEventsSave(blocks)
        } catch (e) {
            console.log(`Failed to update blocks`)
        }

    }
}
