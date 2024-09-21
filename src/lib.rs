use near_sdk::{log, near, AccountId, Promise, env, Gas, PromiseOrValue};
use near_sdk::store::IterableMap;
use near_sdk::json_types::{U128, U64};

mod ext;
use crate::ext::ft_contract;

#[near(contract_state)]
pub struct Contract {
    pub users_points: IterableMap<AccountId, U64>,
    pub previous_caller: Option<AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
        Self { 
            users_points: IterableMap::new(b"u"),
            previous_caller: None,
        }
    }
}

#[near]
impl Contract {
    // Add points based on how much usdc you have
    pub fn add_points(&mut self) -> PromiseOrValue<String> {
        self.previous_caller = Some(env::predecessor_account_id());
        PromiseOrValue::Promise(ft_contract::ext("dai.fakes.testnet".parse().unwrap())
            .with_static_gas(Gas::from_tgas(30))
            .ft_balance_of(env::predecessor_account_id())
            .then(Self::ext(env::current_account_id()).addPOINTScallback()))
    }

    // This method removes points from the callers account
    pub fn remove_points(&mut self, points_to_remove: U64, user: AccountId) {
        let account_id = env::signer_account_id();
        let points = self.users_points.get(&account_id);
        let new_points = points.unwrap().0 - points_to_remove.0;
        self.users_points.insert(account_id, U64(new_points));
    }

    // Lists the points for one user
    pub fn get_all_points(&self) -> Vec<(&AccountId, &U64)> {
        self.users_points.iter().collect()
    }

    // Call back for adding points
    pub fn addPOINTScallback(&mut self, #[callback_unwrap] points: U64) -> String {
        self.users_points.insert(self.previous_caller.as_ref().unwrap().clone(), points);
        "helppppp meeeee".to_string()
    }
}