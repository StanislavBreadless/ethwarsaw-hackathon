#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract]
mod aleph_weth {
    use ink_storage::{traits::{SpreadAllocate, PackedLayout, SpreadLayout, }, Mapping};

    use scale::{Encode, Decode};
    use scale::alloc::vec::Vec;
    use scale_info::TypeInfo;

    use mytoken::Mytoken;


    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct AlephWeth {
        total_supply: u32,
        bridge_address: AccountId,
        admin: AccountId,
        balances: Mapping<AccountId, u128>,
    }
    use ink_lang::utils::initialize_contract;
    impl AlephWeth {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(bridge_address: AccountId) -> Self {
            initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.bridge_address = bridge_address;
                contract.admin = caller;
            })
        }

        fn verify_only_admin(&self) {
            let caller = Self::env().caller();  
            if caller != self.admin {
                panic!("This method should be callable only by admin");
            } 
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
            // verify 
            
            let (minted_amount, minted_to) = Self::parse_get_minted(&log_data);

            let current_balance = self.balances.get(minted_to).unwrap_or_default();
            self.balances.insert(minted_to, &(current_balance + minted_amount));
        }

        /// Parses the log data provided by the user and returns the amount of tokens that
        /// should be minted to the receipt and the recipient's address itself.
        fn parse_get_minted(
            log_data: &[u8],
        ) -> (u128, AccountId) {
            let mut bytes: &[u8] = &[0u8; 32];
            (0, AccountId::decode(&mut bytes).unwrap())
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

    }
}
