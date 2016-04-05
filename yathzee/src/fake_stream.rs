//use yathzee::StringStream;

mod yathzee;
use std::collections::LinkedList;

struct FakeStringStream {
     list_values:  LinkedList,
}


impl yathzee::StringStream for FakeStringStream {
        pub fn input ( input_msg: str){
            list_values.push_back(str);
        }
        pub fn output() -> str {
            return ""; 
        }
}
