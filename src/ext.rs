use near_sdk::json_types::U64;
use near_sdk::{ext_contract, AccountId};

// FT transfer interface
#[ext_contract(ft_contract)]
trait FT {
    fn ft_balance_of(&self, account_id: AccountId) -> U64;
}