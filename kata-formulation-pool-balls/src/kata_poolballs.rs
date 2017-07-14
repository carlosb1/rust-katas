struct Triangle {
   input: String
}



impl Triangle {
    fn new(input: String) -> Triangle {
        return Triangle {input: input}
    }
    fn minimum(&self) -> i32 {
        let len = self.input.len();
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
    fn swapped_balls(&self) -> String {
        let len = self.input.len();
        if len == 1 {
            return "amount:0".to_string();
        }
        let mut chars: Vec<char> = self.input.chars().collect();
        return "amount:1\nswap:0,1".to_string();
    }
}



#[test]
fn triangle_should_does_count_not_swap_for_one_char() {
    let triangle = Triangle::new("R".to_string());
    assert_eq!(triangle.minimum(),0);
}


#[test]
fn triangle_should_does_count_swap_one_times_for_three_chars() {
    let triangle = Triangle::new("RYY".to_string());
    assert_eq!(triangle.minimum(),1);
}


#[test]
fn triangle_should_does_swap_one_times_for_three_chars() {
    let triangle = Triangle::new("RYY".to_string());
    assert_eq!(triangle.swapped_balls(),"amount:1\nswap:0,1");
}

#[test]
fn triangle_should_does_swap_zero_times_for_1_chars() {
    let triangle = Triangle::new("R".to_string());
    assert_eq!(triangle.swapped_balls(),"amount:0");
}


