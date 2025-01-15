use crate::utils::cs_book::base_cs_book::CSBook;
use std::hash::Hash;

pub trait TLibrary<'a> {
    fn print(&self) -> String; //Вывод информации о библиотеке, каждом зале и каждой книге по залам
    fn get_len_library(&self) -> usize; //Число книг в библиотеке
    fn print_best_book(&self) -> &CSBook<'a>; //Вывод самой дорогой книги в библиотеке
    fn calculate_hash<T: Hash>(t: &T) -> u64;
}