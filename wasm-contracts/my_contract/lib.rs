#![cfg_attr(not(feature = "std"), no_std)]


use ink_lang as ink;


#[ink::contract]
mod my_contract {
    use othercontract::OtherContractRef;
    
    
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct MyContract {
        /// Stores a single `bool` value on the storage.
        //value: bool,
        othercontract: OtherContractRef,
    }
    
    impl MyContract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(
            init_value: bool,
            version: u32,
            othercontract_hash: Hash,
        ) -> Self {
            let salt = version.to_le_bytes();
            let othercontract = OtherContractRef::new(init_value)
                .endowment(0)
                .code_hash(othercontract_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "failed at instantiating the Accumulator contract: {:?}",
                        error
                    )
                });

            Self { othercontract }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.othercontract.flip();
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.othercontract.get()
            
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

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let my_contract = MyContract::default();
            assert_eq!(my_contract.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut my_contract = MyContract::new(false);
            assert_eq!(my_contract.get(), false);
            my_contract.flip();
            assert_eq!(my_contract.get(), true);
        }
    }
}
