pub mod base_library {
    pub use crate::utils::cs_hall::base_cs_hall::CSHall;
    pub use crate::utils::t_library::TLibrary;
    pub use crate::utils::cs_book::base_cs_book::CSBook;
    pub use crate::utils::cs_book::base_cs_book::ChangeType;

    use std::hash::{Hash, Hasher, DefaultHasher};

    pub struct ChildrenLibrary <'a> {
        library_name: String,
        halls: &'a mut Vec<&'a mut CSHall<'a>>
    }

    impl <'a> ChildrenLibrary <'a> {
        pub fn new(library_name: String, halls: &'a mut Vec<&'a mut CSHall<'a>>) -> Self {
            Self { library_name, halls }
        }

        //добавить вывод лучшей книги в библиотеке
        //вывод общего числа книг в залах
        //поиграться с хешированием
        //поиграться с equals
        //..

        
    }

    impl <'a> TLibrary <'a> for ChildrenLibrary <'a>{
        fn print(&self) -> String {
            let mut result = String::new();
            result.push_str(&self.library_name);

            for hall in self.halls.iter() {
                result.push_str("\n");
                result.push_str(&hall.print());
                result.push_str("\n");
            }

            result
        }

        fn print_best_book(&self) -> &CSBook<'a> {
            let mut best_book : &CSBook<'a> = self.halls[0].best_book();
            let mut intermediate : &CSBook<'a>;

            for hall in self.halls.iter() {
                intermediate = hall.best_book();
                if best_book.get_price() < intermediate.get_price() {
                    best_book = intermediate;
                }
            }
            best_book
        }

        fn get_len_library(&self) -> usize {
            let mut len : usize = 0;
            for hall in self.halls.iter() {
                len += hall.get_len()
            }
            len
        }

        fn calculate_hash<T: Hash>(t: &T) -> u64 {
            let mut s = DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
    }

    //БИБА
    impl <'a> Hash for ChildrenLibrary <'a> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.library_name.hash(state);
            for hall in self.halls.iter() {
                hall.get_hall_name().hash(state);
                /* 
                for cs_book in hall.get_books().iter() {
                    cs_book.get_input().hash(state);

                    //дописать хеширование полей для книг
                }
                */
                
            }

        }
    }
    

    //БИБА поменьше
    impl Hash for ChangeType {
        fn hash<H: Hasher>(&self, state: &mut H) {
            match self {
                ChangeType::Index(index) => index.to_bits().hash(state),
                ChangeType::MinAge(min_age) => min_age.hash(state)
            }
        }
    }
    
}