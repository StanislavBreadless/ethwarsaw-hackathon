// Pushes Ethereum Ropsten network blocks to the Aleph Zero test network
import * as Eth from 'web3-eth'
const eth = new Eth(new Eth.Eth.providers.HttpProvider('https://ropsten.infura.io/v3/[infura_project_id]'))

export const ethBlocksSync = () => {
    eth.subscribe('newBlockHeaders', function (error, result) {
        if (!error) {
            console.log(result);

            return;
        }

        console.error(error);
    })
        .on("connected", function (subscriptionId) {
            console.log(subscriptionId);
        })
        .on("data", function (blockHeader) {
            console.log('new ETH block' + blockHeader);
        })
        .on("error", console.error);
}