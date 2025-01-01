pub mod base_hall {
    use crate::utils::{children_book::base_children_book::ChildrenBook, t_book::TraitBook};
    

    // использование среза
    pub struct ChildrenHall<'a> {
        name: String,
        cbooks: &'a mut Vec<&'a ChildrenBook>
    }

    impl <'a>ChildrenHall<'a> {
        //конструктор с задаваемыми полями (Название, массив книг)
        pub fn new(name: String, cbooks: &'a mut Vec<&'a ChildrenBook>) -> Self {
            Self { name, cbooks }
        }

        //вывод информации о зале
        pub fn print(&self) -> String {
            let mut result = String::new();
            result.push_str(&format!("{} \n" , self.name));

            for cbook in self.cbooks.iter() {
                result.push_str(&cbook.print());
                result.push_str("\n");
            }
            result
        }

        //передача (ссылка на себя, ссылка на новую книгу)
        pub fn rename_book (&mut self, index: usize, book: &'a ChildrenBook) {

            self.cbooks[index] = book;
            //нужно реализовать отдельный метод глубого клонирования объекта, если 
            //хочу менять не только ссылки в векторе, но и поля оригинального экземпляра
            //..
        }

        fn clone_book(out: ChildrenBook, to: ChildrenBook) {
            //...
        }

        

        //добавление книги в конец вектора
        pub fn add_new_book (&mut self, book: &'a ChildrenBook) {
            self.cbooks.push(book);
        }

        //получение длины вектора
        //...

        //геттер для названия зала
        //...


    }

    
}