pub fn fizzbuzz(input: i32) -> String { 
    let mut value: String = "1".to_string();


    if (input%3)!=0 && (input%5)!=0 {
        value=input.to_string(); 
        return value;
    }
    if input%3==0 {
        value = "fizz".to_string();
     } else if input%5==0 {
        value = "buzz".to_string();
     }

    return value;
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
#[test]
fn fizz_buzz_when_is_five_then_returns_five() {
    let val = fizzbuzz(5);
    assert_eq!("buzz",&val);
}
#[test]
fn fizz_buzz_when_is_divisible_for_three_then_returns_fizz() {
    let val = fizzbuzz(6);
    assert_eq!("fizz",&val);
}

#[test]
fn fizz_buzz_when_is_divisible_for_five_then_returns_buzz() {
    let val = fizzbuzz(10);
    assert_eq!("buzz",&val);
}
#[test]
fn fizz_buzz_when_is_divisible_for_both_then_returns_fizzbuzz() {
    let val = fizzbuzz(15);
    assert_eq!("fizzbuzz",&val);
}


fn main() {
}
