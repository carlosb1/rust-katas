extern crate potter;

use potter::Potter;

#[test]
fn it_works() {
    let potter = Potter::new();
    assert_eq!(2 + 2, 4);
}

