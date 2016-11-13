pub fn fizzbuzz(input: i32) -> String { 
    let mut value = "1";
    if input==2 {
        value = "2";
    } else if input==3 {
        value = "fizz";
    }

    value.to_string()
}



#[test]
fn fizz_buzz_when_is_one_then_returns_one() {
    let val = fizzbuzz(1);
    assert_eq!("1",&val);
}
#[test]
fn fizz_buzz_when_is_two_then_returns_two() {
    let val = fizzbuzz(2);
    assert_eq!("2",&val);
}
#[test]
fn fizz_buzz_when_is_three_then_returns_fizz() {
    let val = fizzbuzz(3);
    assert_eq!("fizz",&val);
}
#[test]
fn fizz_buzz_when_is_four_then_returns_four() {
    let val = fizzbuzz(4);
    assert_eq!("4",&val);
}


fn main() {
}
