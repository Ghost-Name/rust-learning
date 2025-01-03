pub mod book;
pub mod cs_book;

//book
pub use book::base_book::Book as base;
pub use book::base_book::empty_book as empty;

//cs_book
pub use cs_book::base_cs_book::CSBook as base_csbook;
pub use cs_book::base_cs_book::empty_cs_book as empty_csbook;