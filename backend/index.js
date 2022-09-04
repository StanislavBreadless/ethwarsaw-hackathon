import { MongoClient, ServerApiVersion } from 'mongodb'
import { providers } from 'ethers'
import * as dotenv from 'dotenv'
import express from 'express'

import {sendBlocksToAlef} from "./sync.js"
import {ethEventsSave} from "./events-saver.js"
import {getNewBlocks} from "./poll-blocks.js"
import { sleep } from './utils.js';

dotenv.config()
export const eth = new providers.getDefaultProvider('ropsten');
export const client = new MongoClient(process.env.URI, { useNewUrlParser: true, useUnifiedTopology: true, serverApi: ServerApiVersion.v1 });

const POLL_INTERVAL = 10000;

async function main() {
    while(true) {
        await sleep(POLL_INTERVAL);

        try {
            console.log('Fetching blocks...');
            const [blocks, logs] = await getNewBlocks();
            if (blocks.length === 0) {
                continue;
            }
            console.log(`Found ${blocks.length} new blocks`);

            console.log(`Sending new blocks to alef`)
            await sendBlocksToAlef(blocks)

            console.log(`Updating saved events for users`)
            await ethEventsSave(logs)
        } catch (e) {
            console.log(`Failed to update blocks: ` + e)
        }

    }
}

const app = express()

app.get('/getLogData', async (req, res) => {
    const receiver = req.query.receiver;
    const collection = client.db("bridge").collection("logs");
    const log = await collection.findOne({receiver: receiver})
    let data = ''
    let success = true
    if (log == null) {
        success = false
    } else {
        data = log.full_data
    }
    res.send({success: success, data: data})
})

main()
app.listen(process.env.PORT, () => {
    console.log(`Example app listening on port ${process.env.PORT}`)
})
