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


    pub fn checkout(&self) -> f32  {

        let number_books = self.books.len() as i32;
       

        let mut different = false;
        for index in 0..number_books {
                if index >= (number_books - 1) {
                    break;
                }
                let mut found = false;
                for second_index_book in  (index+1).. number_books {
                    if (self.books[index as usize] == self.books[second_index_book as usize]) {     
                        found = true;
                        break;
                    }
                }
                if !found {
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
