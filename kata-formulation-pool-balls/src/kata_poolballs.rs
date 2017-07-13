struct Triangle {
    input: String
}

impl Triangle {
    fn new(input: String) -> Triangle {
        return Triangle {input: input}
    }
    fn minimum(&self) -> i32 { return 0; }
}




    #[test]
    fn it_works() {
        println!("hello world");
        let triangle = Triangle::new("R".to_string());
            assert_eq!(triangle.minimum(),0);

        
    }
