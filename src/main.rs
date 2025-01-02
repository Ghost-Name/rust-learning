mod utils;
use utils::t_book::TraitBook;
//use crate::utils::children_book::base_children_book::ChildrenBook;


fn main() {    
    //использование структуры Book с задаваемымми полями
    let book_1_0 = utils::base::new("Title".to_string(), 
        "Author".to_string(),
        100.,
        2004);
    
    //использование структуры Book без зад-ых полей
    let book_1_1 = utils::empty::new();


    //использование структуры Book для создания базового экземпляра ChildrenBook
    let mut book_2_0 = utils::base_cbook::new(book_1_0.clone(), 10);

    //использование структуры ChildrenBook без зад-ых полей
    let mut book_2_1 = utils::empty_cbook::new();

    //использование структуры Book для создания базового экземпляра ScientificBook
    let book_3_0 = utils::base_sbook::new(book_1_0.clone(), 3);

    //использование структуры ScientificBook без зад-ых полей
    let book_3_1 = utils::empty_sbook::new();

    book_1_0.print();
    book_1_1.print();

    println!("Get (book_1_0): {} {} {} {}", book_1_0.get_title(), book_1_0.get_author(), book_1_0.get_price(), book_1_0.get_age());
    println!("Get (book_1_1): {} {} {} {}", book_1_1.get_title(), book_1_1.get_author(), book_1_1.get_price(), book_1_1.get_age());

    println!("\nFrom trait (book_1_0): {}", book_1_0.print());
    println!("From trait (book_2_0): {}", book_2_0.print());
    println!("From trait (book_2_1): {}", book_2_1.print());
    println!("From trait (book_3_0): {}", book_3_0.print());
    println!("From trait (book_3_1): {} \n", book_3_1.print());

    //создание детского зала с заданными полями (вектор)
    let mut book_2_2 = book_2_1.clone();
    let mut book_2_3 = book_2_1.clone();
    let mut book_2_4 = book_2_1.clone();

    let mut update_book = book_2_0.clone();

    //изменяемый вектор
    let mut vector= vec![&mut book_2_0, &mut book_2_1, &mut book_2_2, &mut book_2_3, &mut book_2_4];

    let mut chall_1_0 = utils::base_chall::new("Children_Hall".to_string(), &mut vector);
    println!("(Use vector) Print Hall: {} ", &chall_1_0.print());

    //замена книги в зале с индексом 1
    chall_1_0.rename_book(1, &mut update_book);

    //добавление книги в конец списка
    chall_1_0.add_new_book(&mut update_book);

    //вывод изменённого зала
    println!("(Use vector) Print refreshed Hall: {} ", &chall_1_0.print());
    println!("Refreshed book_2_1: {}", book_2_1.print()) //какого-то хуя книга с базовыми полями не обновляется, хотя всё менял по ссылке
    

}