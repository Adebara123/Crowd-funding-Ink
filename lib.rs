#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod crowd_funding {
    use ink::storage::Mapping;
    // use ink::prelude::{string::String, vec::Vec};
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
 



    pub struct IndividualFundMe {
        go_fund_id: u64,
        name: String,
        reason_for_fund: String,
        amount_needed: Balance,
        amount_gotten: Balance,
        status: bool,
        donators: Vec<AccountId>
    }

    #[ink(storage)]
    pub struct CrowdFunding {
        go_funds: Mapping<AccountId, IndividualFundMe>,
        // all_go_funds: Vec<IndividualFundMe>,
        // successfull_go_funds: Vec<IndividualFundMe>,
        go_fund_id: u64
    }

    impl CrowdFunding {
       
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                go_funds: Mapping::new(),
                // all_go_funds: Vec::new(),
                // successfull_go_funds: Vec::new(),
                go_fund_id: 1
            }
        }

       
        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new(Default::default())
        // }

      
        #[ink(message)]
        pub fn create_crowdfund(&mut self, _name: String, _reason_for_fund: String, _amount_needed: Balance)  {
            
            let new_gofund =  IndividualFundMe {
                go_fund_id: self.go_fund_id,
                name: _name.clone(),
                reason_for_fund: _reason_for_fund.clone(),
                amount_needed: _amount_needed,
                amount_gotten: 0,
                status: false,
                donators: Vec::new()
            };

            self.go_funds.insert(self.env().caller(), &new_gofund);
            // self.all_go_funds.push(new_gofund);
        }

       
        #[ink(message)]
        pub fn fund_go_fund (&mut self, addr: AccountId) {
            let _go_fund = self.go_funds.get(addr).unwrap().go_fund_id;
            
        }

        // #[ink(message)]
        // pub fn withdraw_from_go_fund(&self)  -> Result<(), Error>{
         
        // }_
    }

   
    // #[cfg(test)]
    // mod tests {
    //     /// Imports all the definitions from the outer scope so we can use them here.
    //     use super::*;

    //     /// We test if the default constructor does its job.
    //     #[ink::test]
    //     fn default_works() {
    //         let crowd_funding = CrowdFunding::default();
    //         assert_eq!(crowd_funding.get(), false);
    //     }

    //     /// We test a simple use case of our contract.
    //     #[ink::test]
    //     fn it_works() {
    //         let mut crowd_funding = CrowdFunding::new(false);
    //         assert_eq!(crowd_funding.get(), false);
    //         crowd_funding.flip();
    //         assert_eq!(crowd_funding.get(), true);
    //     }
    // }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        // async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        //     // Given
        //     let constructor = CrowdFundingRef::default();

        //     // When
        //     let contract_account_id = client
        //         .instantiate("crowd_funding", &ink_e2e::alice(), constructor, 0, None)
        //         .await
        //         .expect("instantiate failed")
        //         .account_id;

        //     // Then
        //     let get = build_message::<CrowdFundingRef>(contract_account_id.clone())
        //         .call(|crowd_funding| crowd_funding.get());
        //     let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
        //     assert!(matches!(get_result.return_value(), false));

        //     Ok(())
        // }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = CrowdFundingRef::new(false);
            let contract_account_id = client
                .instantiate("crowd_funding", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<CrowdFundingRef>(contract_account_id.clone())
                .call(|crowd_funding| crowd_funding.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<CrowdFundingRef>(contract_account_id.clone())
                .call(|crowd_funding| crowd_funding.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<CrowdFundingRef>(contract_account_id.clone())
                .call(|crowd_funding| crowd_funding.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
