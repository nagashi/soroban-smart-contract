use super::{GetCurrentValueContract, GetCurrentValueContractClient};
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn increment_current_value() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GetCurrentValueContract);
    let client = GetCurrentValueContractClient::new(&env, &contract_id);

    assert_eq!(client.increment_current_value(), 1);
    assert_eq!(client.increment_current_value(), 2);
    assert_eq!(client.increment_current_value(), 3);
    assert_eq!(client.increment_current_value(), 4);

    assert_eq!(client.get_current_value(), 4);

    std::println!("{}", env.logs().all().join("\n"));
}