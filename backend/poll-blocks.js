// Pushes Ethereum Ropsten network blocks to the Aleph Zero test network
import { providers } from 'ethers'
const eth = new providers.JsonRpcProvider('https://ropsten.infura.io/v3/[infura_project_id]')
import {client} from './index.js'

export const getNewBlocks = async () => {
    
}