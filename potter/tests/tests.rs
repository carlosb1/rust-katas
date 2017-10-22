extern crate potter;

use potter::Potter;


#[test]
fn should_be_created_empty()  {
    let mut pot = Potter::new();
    let total = pot.checkout();

    assert_eq!(total,0.0);
}

#[test]
fn should_add_two_same_books()  {
    let mut pot = Potter::new();
    let  book = "book1".to_string();
    //TODO refactor to move this to_string, should it be static?
    pot.add(book.clone());
    pot.add(book.clone());

    let total = pot.checkout();
    assert_eq!(total,16.0);
}

#[test]
fn should_add_different_books()  {
    let mut pot = Potter::new();
    let  book = "book1".to_string();
    let  book2 = "book2".to_string();
    pot.add(book.clone());
    pot.add(book2.clone());
    
    let total = pot.checkout();
    assert_eq!(total,15.2);
}


