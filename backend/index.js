import { MongoClient, ServerApiVersion } from 'mongodb'
import * as dotenv from 'dotenv'

import {ethBlocksSync} from "./sync.js"
import {ethEventsSave} from "./events_saver.js"

dotenv.config()
export const client = new MongoClient(process.env.URI, { useNewUrlParser: true, useUnifiedTopology: true, serverApi: ServerApiVersion.v1 });

ethBlocksSync()
ethEventsSave()

client.connect(err => {
    const collection = client.db("bridge").collection("test");
    client.close();
});
