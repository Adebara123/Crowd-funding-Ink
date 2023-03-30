#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod crowd_funding {
    use ink::storage::Mapping;
    // use openbrush::contracts::psp22::*;

    use openbrush::contracts::{
            traits::psp22::PSP22Ref,
        }; 
    
    use ink::prelude::{string::String, vec::Vec};
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    #[derive(scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]


    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    pub enum Error {
        AddressZeroCheck,
        FundNotCompleted
    }
 



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
        psp22_token: ink::ContractRef<PSP22>,
        go_fund_id: u64
    }

    impl CrowdFunding {
       
        #[ink(constructor)]
        pub fn new(psp22_token: ink::ContractRef<PSP22>,) -> Self {
            Self {
                go_funds: Mapping::new(),
                psp22_token,
                go_fund_id: 1
            }
        }

        fn zero_address(&self) -> AccountId {
            [0u8; 32].into()
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

        }

       
        #[ink(message)]
        pub fn fund_go_fund (&mut self, addr: AccountId, _amount: Balance) {
            self.psp22_token.transfer_from(self.env().caller(), self.env().account_id(),_amount);
            self.go_funds.get(addr).unwrap().amount_gotten += _amount;
            
        }

        #[ink(message)]
        pub fn withdraw_from_go_fund(&self, addr: AccountId)  -> Result<(), Error>{
         if addr == self.zero_address() {
            return Err(Error::AddressZeroCheck)
         }
        let fund_gotten = self.go_funds.get(self.env().caller()).unwrap().amount_gotten;
        let fund_needed = self.go_funds.get(self.env().caller()).unwrap().amount_needed;
        
        if &fund_gottten != &fund_need {
            return Err(Error::FundNotCompleted)
        }

        self.go_funds.get(self.env().caller()).unwrap().amount_gotten = 0;
        self.psp22_token.transfer_from( self.env().account_id(),self.env().caller(),fund_gotten);
       
        Ok(())
        }


    }

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
