pub mod base_hall {
    use crate::utils::{children_book::base_children_book::ChildrenBook, t_book::TraitBook};
    

    // использование среза
    pub struct ChildrenHall<'a> {
        name: String,
        cbooks: &'a mut Vec<&'a mut ChildrenBook>
    }

    impl <'a>ChildrenHall<'a> {
        //конструктор с задаваемыми полями (Название, массив книг)
        pub fn new(name: String, cbooks: &'a mut Vec<&'a mut ChildrenBook>) -> Self {
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
        pub fn rename_book (&mut self, index: usize, book: &ChildrenBook) {

            //self.cbooks[index] = book;
            //нужно реализовать отдельный метод глубого клонирования объекта, если 
            //хочу менять не только ссылки в векторе, но и поля оригинального экземпляра
            //..
            clone_book(book, self.cbooks[index]);
        }

        

        

        //добавление книги в конец вектора
        pub fn add_new_book(&mut self, book: &'a mut ChildrenBook) {
            self.cbooks.push(book);
        }

        //получение длины вектора
        //...

        //геттер для названия зала
        //...


    }

    pub fn clone_book(out: &ChildrenBook, to: &mut ChildrenBook) {
        to.set_min_age(out.get_min_age());
        to.book.set_title(out.book.get_title());
        to.book.set_author(out.book.get_author());
        to.book.set_price(out.book.get_price());
        to.book.set_age(out.book.get_age());
    }

    
}