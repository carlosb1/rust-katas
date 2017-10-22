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

    fn is_last(&self, index: i32) -> bool{
        let number_books = self.books.len() as i32;
        return index >= (number_books - 1);
    }

    pub fn checkout(&mut self) -> f32  {

        let number_books = self.books.len() as i32;
       
        let mut different = false;
        for index in 0..number_books {
                if self.is_last(index) {
                    break;
                }
                
                let mut books_to_find = self.books.split_off((index as usize)+1);
                if ! books_to_find.contains(&self.books[index as usize]) {
                    different = true;
                    break;
                }
        }
        if different {
                //TODO move this magic number
                let sum_books_price = (number_books as f32 * 8.0);
                //TODO move this percent
                return sum_books_price * 0.95;
        }
        
        return (number_books as f32 * 8.0);
    }

}
