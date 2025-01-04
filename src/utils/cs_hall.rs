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

        pub fn set_hall_name(&mut self, new_name: String) {
            self.hall_name = new_name;
        }

        pub fn add_book(&mut self, book: &'a mut CSBook<'a>) {
            self.books.push(book);
        }

        pub fn add_book_by_index(&mut self, index: usize, book: &'a mut CSBook<'a>) {
            
            self.books.insert(index, book);
        }

        pub fn best_book(&self) -> CSBook<'a> {
            
            let mut price: f32 = 0.0;
            let mut iter_index: usize = 0;
            let mut best_index: usize = 0;

            for book in self.books.iter() {
                if book.get_price() >= price {
                    price = book.get_price();
                    best_index = iter_index;
                }
                iter_index+=1;
            }
            let best_book= self.books[best_index].clone();
            best_book
        }

        pub fn transfer_book(out: &'a mut CSHall<'a>, index: usize, to: &mut CSHall<'a>) {
            to.books.push(out.books[index]);
            //out.books.pop(out.books[index]);
        }

    }
}