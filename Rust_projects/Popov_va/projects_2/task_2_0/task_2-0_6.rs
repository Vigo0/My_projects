use std::fs;

fn main() {
    let info = "Меня зовут Иван.\nЯ изучаю Rust.\nМне 20 лет."; // так как это макрос то функционал ограничен
    fs::write("output.txt", info).expect("Ошибка записи в файл");
}