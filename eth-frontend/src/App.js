import './App.css';
import TransferForm from './form/Form';
import { Web3ReactProvider } from '@web3-react/core';
import Web3 from 'web3';

function getLibrary(provider, connector) {
  return new Web3(provider);
}

function App() {
  return (
    <Web3ReactProvider getLibrary={getLibrary}>
      <div style={{display: 'flex', flexDirection: 'column'}}>
        <h1 style={{alignSelf: 'center'}}>Transfer ETH to Aleph Zero</h1>
        <div style={{ display: 'flex', justifyContent: 'center' }}>
          <TransferForm />
        </div>
      </div>
    </Web3ReactProvider>
  );
}

export default App;
