

    struct Yathzee;
    
    impl Yathzee {
         pub fn play(input_stream: Stream, output_stream: Stream) {
             
            }
    }

    //Implement interfaces to implement stubs 
    trait Stream<T> {
        fn input(input_msg: T);
        fn output() -> T;
    }
    pub trait StringStream: Stream {
       
        fn input(input_msg: str);
        fn output() -> str;

    }

