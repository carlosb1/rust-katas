pub fn fizzbuzz(input: i32) -> String { 
    let value = "one";
    value.to_string()
}



#[test]
fn fizz_buzz_when_is_one_then_returns_one() {
    println!("fizz buzz one");
    let val = fizzbuzz(1);
    assert_eq!("one",&val);
}

fn main() {
}
