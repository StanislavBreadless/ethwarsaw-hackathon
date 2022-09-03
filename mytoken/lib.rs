#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
mod mytoken {
    use ink_storage::{traits::{SpreadAllocate, PackedLayout, SpreadLayout, }, Mapping};
    use parity_crypto::Keccak256;

    use scale::{Encode, Decode};

    use scale::alloc::vec::Vec;


    // #[derive(Decode, TypeInfo,Encode)]
    // pub struct H512([u64; 8]);
    // #[derive(Decode, TypeInfo,Encode)]
    // pub struct H128([u64; 2]);

    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[derive(Default, Decode, Encode, Eq, PartialEq, SpreadLayout, PackedLayout)]
    pub struct H256([u8; 32]);


    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Mytoken {
        total_supply: u32,
        balances: Mapping<AccountId, u32>,
        block_hashes: Mapping<u64, H256>,

        admin: AccountId
    }

    use ink_lang::utils::initialize_contract;
    impl Mytoken {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new_token(supply: u32) -> Self {
            initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.balances.insert(&caller, &supply);
                contract.total_supply = supply;

                contract.admin = caller;
            })
        }

        fn verify_only_admin(&self) {
            let caller = Self::env().caller();  
            if caller != self.admin {
                panic!("This method should be callable only by admin");
            } 
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
        }
        
        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u32 {
            match self.balances.get(&account) {
                Some(value) => value,
                None => 0,
            }
        }

        #[ink(message)]
        pub fn transfer(&mut self, recipient: AccountId, amount: u32) {
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);
            if sender_balance < amount {
                return;
            }
            self.balances.insert(sender, &(sender_balance - amount));
            let recipient_balance = self.balance_of(recipient);
            self.balances.insert(recipient, &(recipient_balance + amount));
        }

        /// Used to add a block header.
        /// TODO: should validate Ethereum PoW (PoS) + support re-orgs.
        #[ink(message)]
        pub fn add_block_header(&mut self, block_number: u64, block_header: Vec<u8>) {
            self.verify_only_admin();
            // TODO: validate the block header with proof of work
            // For now, we trust the admin to submit only correct blocks.
            
            self.block_hashes.insert(block_number, &H256(block_header.keccak256()));
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
            if H256(rlp_encoded_block_header.keccak256()) != saved_block_hash {
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
        use crate::mytoken::Mytoken;
        use ink_env::{test, DefaultEnvironment};
        use ink_lang as ink;
    
        #[ink::test]
        fn total_supply_works() {
            let mytoken = Mytoken::new_token(1000);
            assert_eq!(mytoken.total_supply(), 1000);
        }
    
        #[ink::test]
        fn balance_of_works() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            test::set_caller::<DefaultEnvironment>(accounts.alice);
            let mytoken = Mytoken::new_token(1000);
            assert_eq!(mytoken.balance_of(accounts.alice), 1000);
            assert_eq!(mytoken.balance_of(accounts.bob), 0);
        }
    
        #[ink::test]
        fn transfer_works() {
            let accounts = test::default_accounts::<DefaultEnvironment>();
            test::set_caller::<DefaultEnvironment>(accounts.alice);
            let mut mytoken = Mytoken::new_token(1000);
            assert_eq!(mytoken.balance_of(accounts.alice), 1000);
            assert_eq!(mytoken.balance_of(accounts.bob), 0);
            mytoken.transfer(accounts.bob, 100);
            assert_eq!(mytoken.balance_of(accounts.alice), 900);
            assert_eq!(mytoken.balance_of(accounts.bob), 100);
        }
    }
}
