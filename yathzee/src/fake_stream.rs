//use yathzee::StringStream;

mod yathzee;

struct FakeStringStream;

impl yathzee::StringStream for FakeStringStream {
        pub fn input ( input_msg: str){
            
        }
        pub fn output() -> str {
            return ""; 
        }
}
