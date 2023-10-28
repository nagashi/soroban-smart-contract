use super::{DecrementorContract, DecrementorContractClient};
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn increment_decrement() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DecrementorContract);
    let client = DecrementorContractClient::new(&env, &contract_id);

    assert_eq!(client.increment_decrement(), 1);
    assert_eq!(client.increment_decrement(), 2);
    assert_eq!(client.increment_decrement(), 3);
    assert_eq!(client.increment_decrement(), 4);
    assert_eq!(client.increment_decrement(), 5);
    assert_eq!(client.increment_decrement(), 6);

    assert_eq!(client.get_decrement_value(), 5);
    assert_eq!(client.get_decrement_value(), 4);
    assert_eq!(client.get_decrement_value(), 3);
    assert_eq!(client.get_decrement_value(), 2);
    assert_eq!(client.get_decrement_value(), 1);
    assert_eq!(client.get_decrement_value(), 0);

    std::println!("{}", env.logs().all().join("\n"));
}