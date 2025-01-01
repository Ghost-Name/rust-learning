pub mod book;
pub mod children_book;
pub mod scientific_book;
pub mod t_book;
pub mod children_hall;

//book
pub use book::base_book::Book as base;
pub use book::base_book::empty_book as empty;

//children_book
pub use children_book::base_children_book::ChildrenBook as base_cbook;
pub use children_book::base_children_book::empty_book as empty_cbook;

//scientific_book
pub use scientific_book::base_scientific_book::ScientificBook as base_sbook;
pub use scientific_book::base_scientific_book::empty_scientific_book as empty_sbook;

//children_hall
pub use children_hall::base_hall::ChildrenHall as base_chall;





