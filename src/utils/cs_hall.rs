pub mod base_cs_hall {

    pub use crate::base_cs_book;

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

        pub fn get_len(&self) -> usize {
            self.books.len()
        }
        pub fn set_hall_name(&mut self, new_name: String) {
            self.hall_name = new_name;
        }

        pub fn add_book(&mut self, book: &'a mut CSBook<'a>) {
            self.books.push(book);
        }

        pub fn add_book_by_index(&mut self, index: usize, book: &'a mut CSBook<'a>) {
            
            self.books.insert(index, book);
        }

        pub fn best_book(&self) -> &CSBook<'a> {
            let mut best_book: &CSBook<'a> = &self.books[0];
            let mut price: f32 = best_book.get_price();

            for book in self.books.iter() {
                if book.get_price() >= price {
                    price = book.get_price();
                    best_book = book;
                }
            }
            
            best_book
        }

        pub fn transfer_book(out: &mut CSHall<'a>, index: usize, to: &mut CSHall<'a>) {
            if index < out.books.len() {
                let book = out.books.remove(index); //remove удаляет и возвращает объект
                to.add_book(book);
            }
        }

    }
}