#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
pub mod eth_bridge {
    use ink_storage::{traits::{SpreadAllocate, PackedLayout, SpreadLayout, }, Mapping};

    use scale::{Encode, Decode};
    use sha3::{Keccak256, Digest};

    use scale::alloc::vec::Vec;

    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[derive(Default, Decode, Encode, Eq, PartialEq, SpreadLayout, PackedLayout)]
    pub struct H256([u8; 32]);

    impl H256 {
        fn from_slice(data: &[u8]) -> Self {
            if data.len() != 32 {
                panic!("The length must be 32 bytes");
            }
            let mut arr = [0u8; 32];
            // arr.copy_from_slice(data);

            H256(arr)
        }
    }

    fn keccak256(digest: &[u8]) -> H256 {
        let mut hasher = Keccak256::new();
        hasher.update(digest);
        let generic_array = hasher.finalize();
        H256::from_slice(generic_array.as_slice())
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct EthBridge {
        block_hashes: Mapping<u64, H256>,

        admin: AccountId
    }

    use ink_lang::utils::initialize_contract;
    impl EthBridge {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.admin = caller;
            })
        }

        fn verify_only_admin(&self) {
            let caller = Self::env().caller();  
            if caller != self.admin {
                panic!("This method should be callable only by admin");
            } 
        }

        /// Used to add a block header.
        /// TODO: should validate Ethereum PoW (PoS) + support re-orgs.
        #[ink(message)]
        pub fn add_block_header(&mut self, block_number: u64, block_header: Vec<u8>) {
            self.verify_only_admin();
            // TODO: validate the block header with proof of work
            // For now, we trust the admin to submit only correct blocks.
            
            self.block_hashes.insert(block_number, &keccak256(&block_header));
        }

        /// Verifies whether a certain tx receipt happened on Ethereum block.
        #[ink(message)]
        pub fn verify_tx_receipt(&self, block_number: u64, rlp_encoded_block_header: Vec<u8>, tx_receipt: Vec<u8>, receipt_path: Vec<u8>, receipt_witness: Vec<u8>) -> bool {
            self.verify_tx_receipt_for_any_block( 
                block_number,
                &rlp_encoded_block_header,
                &tx_receipt,
                &receipt_path,
                &receipt_witness
            )
        }
        
        /// Verifies that a certain event happened on Ethereum
        #[ink(message)]
        pub fn verify_log(
            &self, 
            block_number: u64, 
            rlp_encoded_block_header: Vec<u8>, 
            tx_receipt: Vec<u8>, 
            receipt_path: Vec<u8>,
            receipt_witness: Vec<u8>,
            log_index: u32,
            log_data: Vec<u8>
        ) -> bool {
            if !self.verify_tx_receipt_for_any_block(
                block_number,
                &rlp_encoded_block_header,
                &tx_receipt,
                &receipt_path,
                &receipt_witness
            ) {
                return false;
            }

            if !Self::has_log(&tx_receipt, log_index, &log_data) {
                return false;
            }

            true
        }
        
        fn has_log(rlp_encoded_tx_receipt: &[u8], log_index: u32, log_data: &[u8]) -> bool {
            // TODO: implement check whether the log with index *log_index* has the same data as in `log_data`
            true
        }

        fn verify_tx_receipt_for_any_block(
            &self,
            block_number: u64,
            rlp_encoded_block_header: &[u8],
            rlp_encoded_receipt: &[u8],
            receipt_path: &[u8],
            receipt_witness: &[u8]
        ) -> bool {
            let saved_block_hash: H256 = self.block_hashes.get(block_number).expect("Block not found");

            // Sanity check that the block hash corresponds to the provided block header
            if keccak256(rlp_encoded_block_header) != saved_block_hash {
                return false;
            }

            let receipts_root = Self::extract_receipts_root(&rlp_encoded_block_header);

            // TODO: implement actual tx receipt inclusion
            Self::verify_merkle_proof(rlp_encoded_receipt, receipt_path, receipt_witness, receipts_root)
        }

        fn extract_receipts_root(
            rlp_encoded_block_header: &[u8]
        ) -> H256 {
            let mut result = [0u8; 32];
            result.copy_from_slice(&rlp_encoded_block_header[189..221]);
            H256(result)
        }

        /// Verifies that a merkle patricia tree proof
        /// # Arguments
        /// 
        /// * `value` - The terminating value in the trie
        /// * `encoded_path` - The path in the trie leading to value
        /// * `rlp_parent_nodes` - The path in the trie leading to value
        /// * `root` - The root hash of the trie
        fn verify_merkle_proof(
            value: &[u8],
            encoded_path: &[u8],
            rlp_parent_nodes: &[u8],
            root: H256
        ) -> bool {
            // TODO: implement the verification
            true
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::eth_bridge::EthBridge;
        use ink_env::{test, DefaultEnvironment};
        use ink_lang as ink;
    }
}
