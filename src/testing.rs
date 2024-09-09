use crate::*;
use rand::Rng;

#[test]
fn check_user_id(){
    let id = rand::thread_rng().gen_range(0..50000);
    let user = User::user_create(id, "test".to_string(), false);
    assert_eq!(id, user.id);
}