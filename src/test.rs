#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_1_happy_path_mvp_transaction() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AttendanceContract);
    let client = AttendanceContractClient::new(&env, &contract_id);

    let teacher = Address::generate(&env);
    let student = Address::generate(&env);
    
    // Initialize
    client.init(&teacher);
    
    // Mark attendance
    env.mock_all_auths();
    client.mark_attendance(&student, &1718928000, &true);

    // Verify
    let record = client.get_attendance(&student, &1718928000).unwrap();
    assert_eq!(record.present, true);
    assert_eq!(record.student, student);
}

#[test]
#[should_panic(expected = "attendance already marked for this date")]
fn test_2_edge_case_duplicate_entry() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AttendanceContract);
    let client = AttendanceContractClient::new(&env, &contract_id);

    let teacher = Address::generate(&env);
    let student = Address::generate(&env);
    
    client.init(&teacher);
    env.mock_all_auths();
    
    client.mark_attendance(&student, &1718928000, &true);
    // Should panic on duplicate entry
    client.mark_attendance(&student, &1718928000, &true);
}

#[test]
#[should_panic(expected = "not initialized")]
fn test_3_edge_case_not_initialized() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AttendanceContract);
    let client = AttendanceContractClient::new(&env, &contract_id);

    let student = Address::generate(&env);
    env.mock_all_auths();
    
    // Call without init
    client.mark_attendance(&student, &1718928000, &true);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_4_edge_case_duplicate_init() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AttendanceContract);
    let client = AttendanceContractClient::new(&env, &contract_id);

    let teacher = Address::generate(&env);
    client.init(&teacher);
    
    // Should panic
    client.init(&teacher);
}

#[test]
fn test_5_state_verification() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AttendanceContract);
    let client = AttendanceContractClient::new(&env, &contract_id);

    let teacher = Address::generate(&env);
    let student = Address::generate(&env);
    
    client.init(&teacher);
    env.mock_all_auths();
    
    // State should be None initially
    assert!(client.get_attendance(&student, &1718928000).is_none());
    
    client.mark_attendance(&student, &1718928000, &false);
    
    // State should reflect the update
    let record = client.get_attendance(&student, &1718928000).unwrap();
    assert_eq!(record.present, false);
    assert_eq!(record.date, 1718928000);
}
