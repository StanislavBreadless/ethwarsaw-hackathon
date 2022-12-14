export const abi = [
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "bytes32",
                "name": "receiver",
                "type": "bytes32"
            },
            {
                "indexed": false,
                "internalType": "uint256",
                "name": "amount",
                "type": "uint256"
            }
        ],
        "name": "AlephMintETH",
        "type": "event"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "receiver",
                "type": "bytes32"
            }
        ],
        "name": "lock",
        "outputs": [],
        "stateMutability": "payable",
        "type": "function"
    }
]