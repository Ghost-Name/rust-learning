use crate::utils::book::base_book::Book;


pub mod base_children_book {
    use crate::utils::t_book::TraitBook;

    //применение композиции
    #[derive(Clone)]
    pub struct ChildrenBook {
            pub book: super::Book,
            pub min_age: i8
    }

    
    pub mod empty_book {
        use crate::utils;
        use super::ChildrenBook;

        //создание пустой детской книги
        pub fn new() -> ChildrenBook {
            ChildrenBook {
                book: utils::empty::new(),
                min_age: 0
            }
            
        }
    }

    //конструктор с задаваемымми полями
    impl ChildrenBook {
        pub fn new(book: super::Book, min_age: i8) -> Self {
            Self { book, min_age }
        }

        //геттер
        pub fn get_min_age(&self) -> i8 {
            self.min_age
        }

        //сеттеры 
        pub fn set_min_age(&self, age: i8) {
            //...
        }
    }
    
    impl TraitBook for ChildrenBook {
        fn print(&self) -> String {
            format!("{} {}", self.book.print(), self.get_min_age())
        }
        
    }
    

}