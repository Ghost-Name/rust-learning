pub mod base_scientific_book {
    use crate::utils::{ book::base_book::Book, t_book::TraitBook};

    pub struct ScientificBook {
        book: Book,
        index: i16
    }

    pub mod empty_scientific_book {
        use crate::utils;
        use super::ScientificBook;

        //пустая научная книга
        pub fn new() -> ScientificBook {
            ScientificBook {
                book: utils::empty::new(),
                index: 0
            }
        }
    }
    impl ScientificBook {
        pub fn new(book: Book, index: i16) -> Self {
            Self { book, index }
        }
    }

    impl TraitBook for ScientificBook {
        fn print(&self) -> String {
            format!("{} {}", self.book.print(), self.index)
        }
    }
}