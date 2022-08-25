use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise};

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

mod views;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub beneficiary: AccountId,
    pub donations: UnorderedMap<AccountId, u128>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            beneficiary: "v1.faucet.nonofficial.testnet".parse().unwrap(),
            donations: UnorderedMap::new(b"d"),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    ///
    /// Public but only callable by env::currecnt_account_id()
    ///
    /// Initialize the contract with a specific beneficiary
    ///
    pub fn new(beneficiary: AccountId) -> Self {
        assert!(!env::state_exists(), "Already Initialized");
        Self {
            beneficiary,
            donations: UnorderedMap::new(b"d"),
        }
    }

    ///
    /// A method in which the users attache NEAR in to donate
    ///
    #[payable] // Public - People can attach money
    pub fn donate(&mut self) -> U128 {
        // Get the caller and how much near they attached
        let donor: AccountId = env::predecessor_account_id();
        let dontation_amount: Balance = env::attached_deposit();

        let mut donated_so_far = self.donations.get(&donor).unwrap_or(0);

        let to_transfer: Balance = if donated_so_far == 0 {
            // This is the user's first donataion, lets register it, which increases storage
            assert!(
                dontation_amount > STORAGE_COST,
                "Attach at least {} yoctoNEAR",
                STORAGE_COST
            );

            // Subtract storage cost to the amount to transfer
            dontation_amount - STORAGE_COST
        } else {
            dontation_amount
        };

        // persist in storage the amount donated so far
        donated_so_far += dontation_amount;
        self.donations.insert(&donor, &donated_so_far);

        log!(
            "Thank you {} for donating {}! You donated a total of {}",
            donor.clone(),
            dontation_amount,
            donated_so_far
        );

        // Send money to the beneficiary
        Promise::new(self.beneficiary.clone()).transfer(to_transfer);

        // Return the total amount donated so far
        U128(donated_so_far)
    }

    #[private] // Public - but only callable by env::current_account_id(). Sets the beneficiary
    pub fn change_beneficiary(&mut self, beneficiary: AccountId) {
        self.beneficiary = beneficiary;
    }
}
