pub mod base_book {
    //структура книги
    #[derive(Clone)] //указатель на создание трейта. Делает структуру клонируемой
    pub struct Book {
        title: String,
        author: String,
        price: f32,
        pub_age: i16
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
                pub_age: 0
            }
        }
    }

    //реализация типа Book
    impl Book {
        //конструктор с задаваемымми значениями
        pub fn new(title: String, author: String, price: f32, pub_age: i16) -> Self {
            Self {title, author, price, pub_age}
        }
        pub fn print(&self) -> String {
            format!("{} {} {} {}", self.get_title(), self.get_author(), self.get_price(), self.get_pub_age())
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
        pub fn get_pub_age(&self) -> i16 {
            self.pub_age
        }

        //сеттеры
        pub fn set_title(&mut self, new_title: String) {
            self.title = new_title
        }
        pub fn set_author(&mut self, new_author: String) {
            self.author = new_author
        }
        pub fn set_price(&mut self, new_price: f32) {
            self.price = new_price
        }
        pub fn set_age(&mut self, new_age: i16) {
            self.pub_age = new_age
        }   
    }
}