import { useWeb3React } from "@web3-react/core";
import { injected } from '../components/wallet/connectors';
import { useState } from 'react';
import Web3 from 'web3';
import { abi } from '../components/contract/abi';
import { contractAddress } from '../components/contract/address';

const gweiInEth = 1000000000000000000;

export default function TransferForm() {
    const { active, account, library, connector, activate, deactivate } = useWeb3React();
    const [destAddress, setDestAddress] = useState();
    const [amount, setAmount] = useState();

    async function connect() {
        try {
            await activate(injected);
        } catch (ex) {
            console.log(ex);
        }
    }

    async function disconnect() {
        try {
            deactivate();
        } catch (ex) {
            console.log(ex);
        }
    }

    const onDestAddressChange = (e) => {
        setDestAddress(e.target.value);
    }

    const onAmountChange = (e) => {
        setAmount(Number(e.target.value));
    }

    const send = async () => {
        const web3 = new Web3(window.ethereum);
        await window.ethereum.enable();

        const contract = new web3.eth.Contract(abi, contractAddress);
        contract.methods.lock(destAddress).send({
            from: account,
            value: amount * gweiInEth,
        });
    }

    return (
        <div style={{ display: 'flex', flexDirection: 'column', width: '50%', justifyContent: 'center', textAlign: 'left', marginTop: '100px', backgroundColor: '#b0e5ff', padding: '50px', borderRadius: '35px' }}>
            <span>Your address: {active ? <span style={{ alignSelf: 'flex-end' }}>{account}</span> : ''}</span>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-around' }}>
                <button onClick={connect}>Connect</button>
                <button onClick={disconnect}>Disconnect</button>
            </div>
            <p>Destination</p>
            <input type='text' onChange={onDestAddressChange} />
            <p>ETH Amount</p>
            <input type='text' onChange={onAmountChange} />
            <button style={{ width: '50%', alignSelf: 'center', margin: '10px', height: '45px', borderRadius: '15px' }} onClick={send}>Send</button>
        </div>
    )
}
