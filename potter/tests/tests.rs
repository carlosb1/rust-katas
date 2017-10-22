extern crate potter;

use potter::Potter;
use std::string::String;

#[test]
fn should_be_created_empty()  {
    let mut pot = Potter::new();
    let total = pot.checkout();

    assert_eq!(total,0);
}


#[test]
fn should_add_one_book()  {
    let mut pot = Potter::new();
    let mut book = "book1".to_string();
    pot.add(book);
    
    let total = pot.checkout();
    assert_eq!(total,8);
}
