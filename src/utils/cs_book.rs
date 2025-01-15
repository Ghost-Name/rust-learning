pub mod base_cs_book {
    pub use crate::utils::book::base_book;

    #[derive(Clone)]
    pub enum ChangeType {
        Index(f32),
        MinAge(i32),
    }

    #[derive(Clone)]
    pub struct CSBook <'a >{
        book: base_book::Book,
        input: ChangeType,
        book_type: &'a str,
    }

    impl <'a> CSBook <'a> {
        pub fn new(book: base_book::Book, input: ChangeType, book_type: &'a str) -> Self {
            Self { book, input, book_type }
        }

        pub fn print(&self) -> String {
            match &self.input {
                ChangeType::Index(index) => format!("{} {} {}", self.book.print(), index, self.book_type.to_string()),
                ChangeType::MinAge(min_age) => format!("{} {} {}", self.book.print(), min_age, self.book_type.to_string()),
            }
        }

        pub fn set_input(&mut self, input: ChangeType) {
            match input {
                ChangeType::Index(index) => self.input = ChangeType::Index(index),
                ChangeType::MinAge(min_age) => self.input = ChangeType::MinAge(min_age),
            }
        }

        pub fn get_input(&self) -> ChangeType {
            match self.input {
                ChangeType::Index(index) => ChangeType::Index(index),
                ChangeType::MinAge(min_age) => ChangeType::MinAge(min_age)
            }
        }

        pub fn get_price(&self) -> f32 {
            self.book.get_price()
        }

        
        
        pub fn rewriting_book(&mut self, rewritable_book: &CSBook) {
            //book
            self.book.set_title(rewritable_book.book.get_title());
            self.book.set_author(rewritable_book.book.get_author());
            self.book.set_price(rewritable_book.book.get_price());
            self.book.set_age(rewritable_book.book.get_pub_age());
            //cs_book
            self.set_input(rewritable_book.get_input());
            //self.book_type = rewritable_book.book_type; //невозможно т.к. &str неизменяемый тип
        }
    }

    pub mod empty_cs_book {
        use super::{base_book, CSBook};

        pub fn new(book_type: &str) -> CSBook {
            let book = base_book::empty_book::new();
            match book_type {
                "Scientific" => CSBook { book, input: super::ChangeType::Index(0.0), book_type },
                _ => CSBook { book, input: super::ChangeType::MinAge(0), book_type }
            }
        }
    }
}