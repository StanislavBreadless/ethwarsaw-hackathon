// Pushes Ethereum Ropsten network blocks to the Aleph Zero test network
import * as Eth from 'web3-eth'
const eth = new Eth(new Eth.Eth.providers.HttpProvider('https://ropsten.infura.io/v3/[infura_project_id]'))
import {client} from './index.js'

export const ethEventsSave = () => {

}