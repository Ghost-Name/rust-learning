pub mod base_book {
    use crate::utils::t_book::TraitBook;


    //структура книги
    #[derive(Clone)] //указатель на создание трейта. Делает структуру клонируемой
    pub struct Book {
        title: String,
        author: String,
        price: f32,
        age: i16
    }

    //модуль пустой книги
    pub mod empty_book {
        //super ссылается на родительскую область, self на области внутри главной 
        use super::Book; //импорт структуры из родительского объекта
        
        //дефолтный конструктор 
        pub fn new() -> Book {
            Book {
                title: "None".to_string(), 
                author: "None".to_string(), 
                price: 0., 
                age: 0
            }
        }
    }

    //реализация типа Book
    impl Book {
        //конструктор с задаваемымми значениями
        pub fn new(title: String, author: String, price: f32, age: i16) -> Self {
            Self {title, author, price, age}
        }
        //геттеры
        pub fn get_title(&self) -> String {
            self.title.to_string()
        }
        pub fn get_author(&self) -> String {
            self.author.to_string()
        }
        pub fn get_price(&self) -> f32 {
            self.price
        }
        pub fn get_age(&self) -> i16 {
            self.age
        }
    }

    //реализация trait
    impl TraitBook for Book {
        fn print(&self) -> String {
            format!("{} {} {} {}", self.title, self.author, self.price, self.age) //форматирует в String, но не выводит в консоль
        }
    }
}




