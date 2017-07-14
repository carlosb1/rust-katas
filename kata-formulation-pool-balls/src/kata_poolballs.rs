struct Triangle {
   input: String
}



impl Triangle {
    fn new(input: String) -> Triangle {
        return Triangle {input: input}
    }
    fn minimum(&self) -> i32 {
        let len = self.input.len();
        let mut chars: Vec<char> = self.input.chars().collect();

        if len==1 { 
            return 0;
        }

        /*
        for c in chars {
            println!("{}",c);
        }
        */
        return 1; 
    }
}



#[test]
fn triangle_should_does_not_swap_for_one_char() {
    println!("hello world");
    let triangle = Triangle::new("R".to_string());
    assert_eq!(triangle.minimum(),0);
}


#[test]
fn triangle_should_does__swap_one_times_for_three_chars() {
    println!("hello world");
    let triangle = Triangle::new("RYY".to_string());
    assert_eq!(triangle.minimum(),1);
}
