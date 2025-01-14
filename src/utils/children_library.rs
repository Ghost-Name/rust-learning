pub mod base_library {
    pub use crate::utils::cs_hall::base_cs_hall::CSHall;
    pub use crate::utils::t_library::TLibrary;
    pub use crate::utils::cs_book::base_cs_book::CSBook;

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
    }
}