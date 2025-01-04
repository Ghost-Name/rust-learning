pub mod utils;
use utils::base_csbook;
use utils::empty_csbook;
use utils::base_cshall;

pub use utils::cs_book::base_cs_book;

fn main() {    
   //book
    let book_1_0 = utils::base::new("Title".to_string(), 
        "Author".to_string(),
        99.90,
        2004);
    
    let book_1_1 = utils::empty::new();

    book_1_0.print();
    book_1_1.print();

    println!("book_1_0: {}", book_1_0.print());
    println!("book_1_1: {}\n", book_1_1.print());

    //cs_book
    let mut csbook_1_0 = base_csbook::new(book_1_0, base_cs_book::ChangeType::Index(0.82), "Scientific");
    let mut csbook_1_1 = empty_csbook::new("Children");
    let mut csbook_1_2 = empty_csbook::new("Scientific");
    let mut new_csbook = empty_csbook::new("Children");
    let mut add_csbook = empty_csbook::new("Scientific");

    println!("csbook_1_0: {}", csbook_1_0.print());
    println!("csbook_1_1: {}", csbook_1_1.print());
    println!("csbook_1_2: {}", csbook_1_2.print());

    csbook_1_2.rewriting_book(&csbook_1_0);
    println!("Rewrite csbook_1_2: {}", csbook_1_2.print());

    //cs_hall
    let mut vector = vec![&mut csbook_1_0, &mut csbook_1_1, &mut csbook_1_2];
    let mut cs_hall = base_cshall::new(String::from("hall"), &mut vector);
    cs_hall.add_book(&mut new_csbook);
    cs_hall.add_book_by_index(3, &mut add_csbook);
    println!("\ncs_hall: {}", cs_hall.print());
    println!("\nBest book: {}", cs_hall.best_book().print())

    //children_library
    //..

    //scientific_lybrary
    //..

}