pub mod utils;
use utils::base_csbook;
use utils::empty_csbook;

pub use utils::cs_book::base_cs_book;

fn main() {    
   
    //использование структуры Book с задаваемымми полями
    let book_1_0 = utils::base::new("Title".to_string(), 
        "Author".to_string(),
        99.90,
        2004);
    
    //использование структуры Book без зад-ых полей
    let book_1_1 = utils::empty::new();

    book_1_0.print();
    book_1_1.print();

    println!("book_1_0: {}", book_1_0.print());
    println!("book_1_1: {}\n", book_1_1.print());

    let csbook_1_0 = base_csbook::new(book_1_0, base_cs_book::ChangeType::Index(0.82), "Scientific");
    let csbook_1_1 = empty_csbook::new("Children");
    let mut csbook_1_2 = empty_csbook::new("Scientific");
    println!("csbook_1_0: {}", csbook_1_0.print());
    println!("csbook_1_1: {}", csbook_1_1.print());
    println!("csbook_1_2: {}", csbook_1_2.print());

    csbook_1_2.rewriting_book(csbook_1_0);
    println!("Rewrite csbook_1_2: {}", csbook_1_2.print())

}