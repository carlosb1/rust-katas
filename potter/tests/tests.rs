extern crate potter;

use potter::Potter;
use std::string::String;

#[test]
fn should_be_created_empty()  {
    let pot = Potter::new();
    let total = pot.checkout();

    assert_eq!(total,0);
}

