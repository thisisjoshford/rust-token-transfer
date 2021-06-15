use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, near_bindgen, AccountId, Promise};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner_id: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: ValidAccountId) -> Self {
        Self {
            owner_id: owner_id.to_string(),
        }
    }

    pub fn transfer(&self, account_id: ValidAccountId, amount: U128) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "ERROR: Only the owner of this contract can transfer tokens."
        );
        Promise::new(account_id.to_string()).transfer(amount.0);
    }
}
