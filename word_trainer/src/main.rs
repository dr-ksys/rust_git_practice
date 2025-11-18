use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use rand::seq::SliceRandom;//Нужен крейт rand для перемешивания
use rand::thread_rng;

//Структура для хранения пары слов
struct WordPair {
    original: String,
    translation: String,
}

fn main() -> io::Result<()> {
    //Подключить крейт rand (добавить в Cargo.toml)
    let mut words = load_words("words")?;
    if words.is_empty() {
        println!("Файл со словами пуст или не содержит корректных пар");
        return Ok(());
    }

    //Перемешиваем слова в случайном порядке
    let mut rng = thread_rng();
    words.shuffle(&mut rng);

    let mut correct_count = 0;
    let total_count = words.len();

    println!("****** Тренажер слов ******");
    println!("Всего слов для изучения: {}", total_count);

    for (index, pair) in words.iter().enumerate() {
        println!("\n[{}/{}] Как переводится слово: {}", index + 1, total_count, pair.original);
        print!("Ваш ответ: ");
        //Принудительно выводим текст приглашения до ввода
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim().to_lowercase();

        if user_input == pair.translation.to_lowercase() {
            println!("Правильно!");
            correct_count += 1;
        } else {
            println!("Неправильно. Правильный ответ: {}", pair.translation);
        }
    }

    println!("\n****** Результат ******");
    println!("Правильных ответов: {} из {} ({}%)", correct_count, total_count, (correct_count as f32 / total_count as f32 * 100.0) as u32);
    Ok(())
    
}

//Функция для чтения файла и парсинга слов
fn load_words(filename: &str) -> io::Result<Vec<WordPair>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut pairs = Vec::new();

    for line in reader.lines() {
        let line = line?;
        //Ищем разделитель " - " или просто "-"
        if let Some(separator_pos) = line.find(" - ") {
            let original = line[..separator_pos].trim().to_string();
            let translation = line[separator_pos + 3..].trim().to_string();
            if !original.is_empty() && !translation.is_empty() {
                pairs.push(WordPair { original, translation });
            }
        } else if let Some(separator_pos) = line.find('-') {
            let original = line[..separator_pos].trim().to_string();
            let translation = line[separator_pos + 1..].trim().to_string();
            if !original.is_empty() && !translation.is_empty() {
                pairs.push(WordPair { original, translation });
            }
        }
    }

    Ok(pairs)
}
