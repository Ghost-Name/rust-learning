pub mod base_library {
    pub use crate::utils::cs_hall::base_cs_hall::CSHall;
    pub use crate::utils::t_library::TLibrary;

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
    }
}