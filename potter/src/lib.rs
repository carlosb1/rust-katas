use std::string::String;
use std::vec::Vec;

pub struct Potter {
        books: Vec<String> 

    }

impl Potter  {
    pub fn new() -> Potter {
         let books = Vec::new();
         return Potter {books: books};
    }
    
    pub fn add(&mut self, new_book: String)  {
        self.books.push(new_book);
    }

    pub fn checkout(&self) -> i32  {

        let number_books = self.books.len() as i32;
        return (number_books * 8);
    }
}
