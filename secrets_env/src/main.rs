//Add dotenv = "0.15" to Cargo.toml
use dotenv::dotenv;
use std::env;

fn main() {
    //Загружаем переменные из .env в окружение
    dotenv().ok();
    //Читаем данные из переменных окружения
    //expect() используется для обработки ошибок, если переменная не найдена
    let email = env::var("EMAIL").expect("EMAIL must be set in .env file");
    let password = env::var("PASSWORD").expect("PASSWORD must be set in .env file");

    println!("Email: {}", email);
    println!("Password: {}", password);
}
