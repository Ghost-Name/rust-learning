pub mod utils;
use utils::base_csbook;
use utils::empty_csbook;
use utils::base_cshall;
use utils::ch_library;

pub use utils::cs_book::base_cs_book;
pub use utils::t_library::TLibrary;


fn main() {    
   //book
    let book_1_0 = utils::base::new(String::from("Title"), 
        String::from("Author"),
        99.90,
        2004);
    /* 
    let book_1_1 = utils::empty::new();

    book_1_0.print();
    book_1_1.print();

    println!("book_1_0: {}", book_1_0.print());
    println!("book_1_1: {}\n", book_1_1.print());
    */

    //cs_book
    let mut csbook_1_0 = base_csbook::new(book_1_0, base_cs_book::ChangeType::Index(0.82), "Scientific");
    let mut csbook_1_1 = empty_csbook::new("Children");
    let mut csbook_1_2 = empty_csbook::new("Scientific");
    let mut new_csbook = empty_csbook::new("Children");
    let mut add_csbook = empty_csbook::new("Scientific");

    let mut csbook_2_0 = empty_csbook::new("Children");
    let mut csbook_2_1 = empty_csbook::new("Children");
    let mut csbook_2_2 = empty_csbook::new("Scientific");

    /* 
    println!("csbook_1_0: {}", csbook_1_0.print());
    println!("csbook_1_1: {}", csbook_1_1.print());
    println!("csbook_1_2: {}", csbook_1_2.print());
    */

    csbook_1_2.rewriting_book(&csbook_1_0);
    //println!("Rewrite csbook_1_2: {}", csbook_1_2.print());

    //cs_hall
    let mut vector_1 = vec![&mut csbook_1_0, &mut csbook_1_1, &mut csbook_1_2];
    let mut cs_hall_1 = base_cshall::new(String::from("hall-1"), &mut vector_1);
    cs_hall_1.add_book(&mut new_csbook);
    cs_hall_1.add_book_by_index(3, &mut add_csbook);
    println!("\ncs_hall_1: {}", cs_hall_1.print());
    println!("\nBest book in cs_hall_1: {}", cs_hall_1.best_book().print());

    let mut vector_2 = vec![&mut csbook_2_0, &mut csbook_2_1, &mut csbook_2_2];
    let mut cs_hall_2 = base_cshall::new(String::from("hall-2"), &mut vector_2);
    println!("\ncs_hall_2: {}", cs_hall_2.print());

    base_cshall::transfer_book(&mut cs_hall_1, 0, &mut cs_hall_2);
    println!("\nRefreshed cs_hall_1: {} \n\nRefreshed cs_hall_2: {}", cs_hall_1.print(), cs_hall_2.print());

    //children_library
    let mut ch_vector = vec![&mut cs_hall_1, &mut cs_hall_2];
    let children_library = ch_library::new(String::from("Children library"), &mut ch_vector);
    println!("\nchildren_library: {}", children_library.print());

    //scientific_lybrary
    //..

}