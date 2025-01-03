pub mod base_cs_hall {
    use std::vec;

    pub use crate::base_cs_book;
    use crate::utils::empty::new;

    use self::base_cs_book::CSBook;
    pub struct CSHall <'a> {
        hall_name: String,
        books: &'a mut Vec<&'a mut CSBook<'a>>,
    }
    impl <'a> CSHall <'a> {
        pub fn new(hall_name: String, books: &'a mut Vec<&'a mut CSBook<'a>>) -> Self {
            Self { hall_name, books }
        }

        pub fn print(&self) -> String {
            let mut result = String::new();
            result.push_str( &self.hall_name);

            for book in self.books.iter() {
                result.push_str("\n");
                result.push_str(&book.print());
                
            }
            result
        }

        pub fn set_hall_name(&mut self, new_name: String) {
            self.hall_name = new_name;
        }

        pub fn add_book(&mut self, book: &'a mut CSBook<'a>) {
            self.books.push(book);
        }

        pub fn add_book_by_index(mut self, index: usize, book: &'a mut CSBook<'a>) -> CSHall<'a> {
            
            let mut vector: Vec<&'a mut CSBook<'a>> = vec![];

            for num in 0..self.books.len()-1 {  

                if num == index {
                    vector.push(book);
                }
                else {
                    vector.push(self.books[num]);
                }
            }
            self.books = vector;
            CSHall { hall_name: self.hall_name, books: &mut vector }
            //нужно подумать, лучше стоит передать право владения над залом и вернуть новый зал
        }

    }
}