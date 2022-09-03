import { MongoClient, ServerApiVersion } from 'mongodb'
import * as dotenv from 'dotenv'

dotenv.config()

const client = new MongoClient(process.env.URI, { useNewUrlParser: true, useUnifiedTopology: true, serverApi: ServerApiVersion.v1 });

client.connect(err => {
    const collection = client.db("bridge").collection("test");
    client.close();
});
