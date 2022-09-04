# ETH <> ALEPH ZERO ZKP bridge

This repo contains PoC for a trustless bridge between Ethereum and Aleph zero chains. The "trustless" part is achieved by validating that w

## Architecture

Our bridge enables passing arbitrary messages between Ethereum and Aleph Zero in a trustless manner. Trustlessness is achieved by the design of the bridge for both ETH -> AZERO briding and AZERO -> ETH bridging. For now, the PoC only covers briding from Ethereum, but it can be extended for passing arbitrary messages from AZERO to Ethereum. 

### Passing messages from Ethereum to Aleph Zero 

The core principle of verifying that a certain event (or, more generally, transaction receipt) was included in a block, is by providing the preimage to the block hash. You can read more about the structure of the block hash in the [yellow paper](https://ethereum.github.io/yellowpaper/paper.pdf).

The hardest part here is maintaining the correct Ethereum's block hashes in an efficient way. We maintain all the known Ethereum block hashes as well as the latest one based on the consensus algorithm:

- In the case of PoW, we choose the latest block using "difficulty".
- In the case of PoS, we choose the correct block by validating the rules for the PoS consensus.

In this PoC, we have not yet implemented the validation logic for the consensus algorithm as well as support for re-orgs.

The inspiration for the design was taken from the [rainbow bridge](https://github.com/aurora-is-near/rainbow-bridge).

### Passing messages from Aleph Zero to Ethereum 

Since Ethereum is a rather expensive chain, we can not afford to validate Aleph Zero's block headers for each block, so we will do the optimistic approach.

### Using zero-knowledge proofs to lower the costs

Most of the validation logic can be made constant with respect to the number of blocks using zero-knowledge proofs.

## Repo structure

- The basic implementation of the bridge that allows receiving arbitrary messages from Ethereum can be found [here](./contracts/aleph/eth-bridge).
- The implementation of the bridged Ethereum can that uses the contract above to receive events from Ethereum about bridged funds [here](./contracts/aleph/aleph-weth/).
- The L1 bridge of Ethereum is [here](./contracts/eth/AlephConnector).
- The backend deployment used for tracking the events of the deposit used for better UX can be found [here](./backend).
- The front-end for Ethereum bridge can be found [here](./eth-frontend).

## What is done?

- [+] Basic implementation of the Ethereum -> Aleph Zero message passing.
- [+] Backend deployment used for tracking deposit events.
- [+] Front-end for the Ethereum part of the bridge.

## What is to be done?

- [ ] Consensus validation logic.
- [ ] Full event inclusion validation.
- [ ] Integrate zero-knowledge proofs. 
- [ ] Front-end for the Aleph Zero part of the bridge.

