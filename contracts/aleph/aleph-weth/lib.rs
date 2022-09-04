#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod aleph_weth {
    use ink_storage::{traits::{SpreadAllocate, PackedLayout, SpreadLayout}, Mapping};

    use scale::{Encode, Decode};
    use scale::alloc::vec::Vec;

    use sha3::{Keccak256, Digest};

    use eth_bridge::eth_bridge::{EthBridgeRef, EthBridge};

    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[derive(Debug, Default, Decode, Encode, Eq, PartialEq, SpreadLayout, PackedLayout, Copy, Clone)]
    pub struct H256([u8; 32]);

    impl H256 {
        fn from_slice(data: &[u8]) -> Self {
            if data.len() != 32 {
                panic!("The length must be 32 bytes");
            }
            let mut arr = [0u8; 32];
            arr.copy_from_slice(data);

            Self(arr)
        }

        /// Converts the H256 to u128. Note that all the bits
        /// above the 128th one are ignored.
        fn as_be_u128(self) -> u128 {
            let mut uint128_bytes = [0u8; 16];
            uint128_bytes.copy_from_slice(&self.0[16..]);
            u128::from_be_bytes(uint128_bytes)
        } 
    }


    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
    #[derive(Debug, Default, Decode, Encode, Eq, PartialEq, SpreadLayout, PackedLayout, Copy, Clone, ink_storage::traits::SpreadAllocate)]
    pub struct H160([u8; 20]);

    impl H160 {
        fn from_slice(data: &[u8]) -> Self {
            if data.len() != 20 {
                panic!("The length must be 32 bytes");
            }
            let mut arr = [0u8; 20];
            arr.copy_from_slice(data);

            Self(arr)
        }
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct AlephWeth {
        total_supply: Balance,
        bridge_address: AccountId,
        eth_bridge_address: H160,
        admin: AccountId,
        balances: Mapping<AccountId, Balance>,
        used_logs: Mapping<H256, bool>,
    }
    use ink_lang::utils::initialize_contract;
    impl AlephWeth {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(bridge_address: AccountId, eth_bridge_address: H160) -> Self {
            initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.total_supply = 0;
                contract.bridge_address = bridge_address;
                contract.eth_bridge_address = eth_bridge_address;
                contract.admin = caller;
            })
        }

        fn get_bridge_ref(&self) -> EthBridgeRef {
            ink_env::call::FromAccountId::from_account_id(self.bridge_address)
        }

        fn verify_only_admin(&self) {
            let caller = Self::env().caller();  
            if caller != self.admin {
                panic!("This method should be callable only by admin");
            } 
        }

        fn keccak256(digest: &[u8]) -> H256 {
            let mut hasher = Keccak256::new();
            hasher.update(digest);
            let generic_array = hasher.finalize();
            H256::from_slice(generic_array.as_slice())
        }    

        fn log_id(
            block_number: u64, 
            tx_receipt: &[u8],
            log_index: u32,
        ) -> H256 {
            let mut digest = Vec::<u8>::new();

            digest.extend(block_number.to_be_bytes().to_vec());
            digest.extend(tx_receipt);
            digest.extend(log_index.to_be_bytes().to_vec());

            Self::keccak256(&digest)
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn claim(
            &mut self,
            block_number: u64, 
            rlp_encoded_block_header: Vec<u8>, 
            tx_receipt: Vec<u8>,
            receipt_path: Vec<u8>,
            receipt_witness: Vec<u8>,
            log_index: u32,
            log_data: Vec<u8>
        ) {
            // collision-resistance protection:
            // the triple <block-number>, <tx_receipt>, <log_index> is unique
            // since tx-receipt contains the cumulative gas used, meaning it is unique for 
            // each transaction in a block:
            let log_id = Self::log_id(block_number, &tx_receipt, log_index);
            match self.used_logs.get(log_id)  {
                Some(true) => {
                    panic!("This log has been already used")
                },
                _ => {
                    self.used_logs.insert(log_id, &true);
                }
            }

            // verify the proof
            let is_valid_proof = !self.get_bridge_ref().verify_log(
                block_number,
                rlp_encoded_block_header, 
                tx_receipt.clone(), 
                receipt_path,
                receipt_witness,
                log_index,
                log_data.clone()
            );
            if !is_valid_proof {
                panic!("The proof is invalid");
            }
            
            let (minted_amount, minted_to) = self.parse_get_minted(&log_data);

            let current_balance = self.balances.get(minted_to).unwrap_or_default();
            self.balances.insert(minted_to, &(current_balance + minted_amount));
            self.total_supply += minted_amount;
        }

        /// Parses the log data provided by the user and returns the amount of tokens that
        /// should be minted to the receipt and the recipient's address itself.
        pub fn parse_get_minted(
            &self,
            log_data: &[u8],
        ) -> (u128, AccountId) {
            // The signature of event is the following:
            // event AlephMintETH(bytes32 indexed receiver, uint256 amount);

            if log_data.len() != 116 { // 20 + 32 * 3
                panic!("The correct log must be 116 bytes long");
            }

            let emitter = H160::from_slice(&log_data[0..20]);
            if emitter != self.eth_bridge_address {
                panic!("The log must be emitted by the Ethereum bridge contract");
            }

            // TODO: cache the required topic:
            let event_signature = Self::keccak256("AlephMintETH(bytes32,uint256)".as_bytes());
            let topic0 = H256::from_slice(&log_data[20..52]);
            if event_signature != topic0 {
                panic!("The event topic is incorrect");
            }
            
            
            let topic1 = H256::from_slice(&log_data[52..84]);
            let mut account_bytes: &[u8] = &topic1.0;

            let receiver = AccountId::decode(&mut account_bytes).unwrap();

            let amount = H256::from_slice(&log_data[84..116]);

            // We can safely ignore the topmost 128 bits, since it is not ever feasible
            // to mint more than 3 * 10^20 ETH
            (amount.as_be_u128(), receiver)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        fn account_from_text(hex_str: &str) -> AccountId {
            let mut bytes: &[u8] = &hex::decode(hex_str).expect("Invalid hex");
            AccountId::decode(&mut bytes).expect("Invalid account address")
        }

        const etherem_bridge_address: H160 = H160([0x5F, 0xbD, 0xB2, 0x31, 0x56, 0x78, 0xaf, 0xec, 0xb3, 0x67, 0xf0, 0x32, 0xd9, 0x3F, 0x64, 0x2f, 0x64, 0x18, 0x0a, 0xa3]);
        const log_data: [u8; 116] = [
            0x5f, 0xbd, 0xb2, 0x31, 0x56, 0x78, 0xaf, 0xec, 0xb3, 0x67, 0xf0, 0x32, 0xd9, 0x3f, 0x64, 0x2f, 0x64, 0x18, 0x0a, 0xa3, 0x8a, 0xb8, 0x61, 0x6c, 0xbf, 0x81, 0x54, 0x6f, 0xc5, 0x3f, 0xe1, 0xa6, 0xc6, 0x56, 0x6d, 0xc7, 0xe7, 0xf3, 0x67, 0x0d, 0xda, 0x4a, 0xfb, 0x2d, 0xa6, 0xc0, 0x3c, 0x8d, 0x88, 0xf4, 0x34, 0x2c, 0xd4, 0x35, 0x93, 0xc7, 0x15, 0xfd, 0xd3, 0x1c, 0x61, 0x14, 0x1a, 0xbd, 0x04, 0xa9, 0x9f, 0xd6, 0x82, 0x2c, 0x85, 0x58, 0x85, 0x4c, 0xcd, 0xe3, 0x9a, 0x56, 0x84, 0xe7, 0xa5, 0x6d, 0xa2, 0x7d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0xe0, 0xb6, 0xb3, 0xa7, 0x64, 0x00, 0x00
        ];

        
        /// Test the "parse_get_minted" fuction
        #[ink::test]
        fn test_parse_get_minted() {
            // Address of the L1 bridge

            let alepth_weth = AlephWeth::new(
                AccountId::default(),
                etherem_bridge_address
            );

            let (minted_amount, receiver) = alepth_weth.parse_get_minted(
                &log_data,
            );

            // 1 ether
            let expected_minted_amount: u128 = 1000000000000000000;
            let expected_receiver = account_from_text("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");

            assert_eq!(minted_amount, expected_minted_amount, "Parsed minted amount is incorrect");
            assert_eq!(receiver, expected_receiver, "Parsed minted amount is incorrect");
        }
    }
}
