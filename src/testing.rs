use crate::*;
use rand::Rng;

/*
Commands
cargo test -- --test-threads=1
cargo test -- --show-output
cargo test -- --ignored
cargo test -- --include-ignored

#[ignore] - to ignore in tests
*/

#[test]
fn check_user_id_assert(){
    let id = rand::thread_rng().gen_range(0..50000);
    let user = User::user_create(id, "test".to_string(), false);
    assert_eq!(id, user.id);
}

#[test]
fn check_user_id_result() -> Result<(), String>{
    let id = rand::thread_rng().gen_range(0..50000);
    let user = User::user_create(id, "test".to_string(), false);
    if id == user.id{
        Ok(())
    }
    else {
        Err(String::from("IDs are different"))
    }
}

#[test]
#[should_panic]
fn panic_user_create(){
    let user = User::user_create(0, "test".to_string(), false);
}