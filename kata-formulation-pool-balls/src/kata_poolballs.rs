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



        return 1; 
    }
    fn swapped_balls(&self) -> String {
        let len = self.input.len();
        if len == 1 {
            return "amount:0".to_string();
        }

        let mut chars: Vec<char> = self.input.chars().collect();
        let mut output = Vec::new();

        let mut rep_indexes: Vec<i32> = Vec::new();
        for index in 1..len as i32 {
            let indexSize = index as usize;
            let prev_char = chars[(indexSize -1) as usize ];
            let current_char =  chars[indexSize as usize];
            if current_char == prev_char {    
                rep_indexes.push(index-1);
            }
        }


        let mut num_swaps = 0;
        for rep_index in rep_indexes {
            for (i,c) in chars.iter().enumerate() {
                if c != chars[rep_index as usize] {
                    output.push(format!("swap:{},{}",i,rep_index));
                }
            }
        }
        

        /*
        for index in 0..len as i32 {

            println!("{}",chars[index as usize]);
            output.push("swap:0,1".to_string());
            num_swaps+=1;
        }
*/
        /*
        let mut index = 0;
        for c in chars {
            println!("{}",c);
            index+=1;
        }*/
        output.insert(0,format!("amount:{}",num_swaps).to_string()); 
        return output.join("\n");
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


#[test]
fn triangle_should_does_swap_two_times_for_three_chars() {
    let triangle = Triangle::new("RRRYY".to_string());
    assert_eq!(triangle.swapped_balls(),"amount:1\nswap:1,3\nswap:3,4");
}
