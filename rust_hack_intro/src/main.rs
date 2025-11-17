/**********************************

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let os = env::consts::OS;
    let current_dir = env::current_dir().unwrap();
    println!("[*] Запущен с аргументами :{:?}", args);
    println!("[*] Операционная система: {}", os);
    println!("[*] Текущая директория: {:?}", current_dir);
}

***********************************/

/***********************************

macro_rules! log {
    ($level:expr, $msg:expr) => {
        println!("[{}] {}", $level, $msg);
    };
}
fn main() {
    log!("INFO", "Сканирование портов начато");
    log!("ERROR", "Таймаут подключения")

}
************************************/


/* Обработка ошибок  Result Option*/
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("secret.txt")?;
    println!("[+] Файл открыт");
    Ok(())
}
