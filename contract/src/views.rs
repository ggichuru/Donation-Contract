//! #Views
//!
//!
//!

use near_sdk::serde::Serialize;

use crate::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Donation {
    pub account_id: AccountId,
    pub total_amount: U128,
}

#[near_bindgen]
impl Contract {
    //Public - get donation by account ID
    pub fn get_donation_for_account(&self, account_id: AccountId) -> Donation {
        Donation {
            account_id: account_id.clone(),
            total_amount: U128(self.donations.get(&account_id).unwrap_or(0)),
        }
    }

    // Public get Total number of donations
    pub fn total_donations(&self) -> u64 {
        self.donations.len()
    }

    // Public - Paginate through all donations on the contract
    pub fn get_donations(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Donation> {
        // Where to start pagination if we have a from_index, use that, else start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        // Iterate through donation
        self.donations
            .keys()
            .skip(start as usize) // skip to index that we specified in the start variable
            .take(limit.unwrap_or(50) as usize) //take the first limit elements in the vector. If we didn't specify a limit use 50
            .map(|account| self.get_donation_for_account(account))
            .collect() // since we turned map into an iterator, we need to turn it back into a vector to return
    }

    /// Public - beneficiary getter
    pub fn beneficiary(&self) -> AccountId {
        self.beneficiary.clone()
    }
}
