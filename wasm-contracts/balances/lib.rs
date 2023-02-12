#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod mycontract {
    use ink_storage::{
        Mapping,
        traits::SpreadAllocate
    };

     use openbrush::{
         traits::{
             Storage,
        },
    };

    #[derive(SpreadAllocate,Storage)]
    #[ink(storage)]
    pub struct MyContract {
        /// Assign a balance to every account ID
        balances: Mapping<AccountId, Balance>,
    }

    impl MyContract {
        /// Constructor to initialize the contract with an empty mapping.
        #[ink(constructor, payable)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.balances = Mapping::default();
            })
        }

        /// Retrieve the balance of the caller.
        #[ink(message)]
        pub fn get_balance(&self) -> Option<Balance> {
            let caller = self.env().caller();
            self.balances.get(&caller)
        }

        /// Credit more money to the contract.
        #[ink(message, payable)]
        pub fn transfer(&mut self) {
            let caller = self.env().caller();
            let balance = self.balances.get(&caller).unwrap_or(0);
            let endowment = self.env().transferred_value();
            self.balances.insert(&caller, &(balance + endowment));
        }

        /// Withdraw all your balance from the contract.
        pub fn withdraw(&mut self) {
            let caller = self.env().caller();
            let balance = self.balances.get(&caller).unwrap();
            self.balances.remove(&caller);
            self.env().transfer(caller, balance).unwrap()
        }
    }
}
