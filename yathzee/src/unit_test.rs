#[cfg(not(test))]
//unit_test.rs
//Conditionally compile `main` only when the test-suite is *not* being run.
fn main () {
    println!("If you see this, tests were not compiled nor ran!");
}

//Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod test {
    #[test]
    fn play() {
        //FIXME start kata
        assert!(false);    
    }

}


