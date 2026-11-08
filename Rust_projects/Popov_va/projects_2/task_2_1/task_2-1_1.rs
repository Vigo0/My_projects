use std::io;

fn main() {
    // 1. Ввод имени
    println!("Введите имя:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Не удалось прочитать имя");
    let name = name.trim().to_string();// это важно потому что trim делает срез. и тогда переменная это ссылка на кусок строки а не полноценная строка

    // 2. Ввод возраста
    println!("Возраст:");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Не удалось прочитать возраст");
    let age = age.trim().to_string();

    // 3. Вывод информации
    println!("Студент : {}!", name);
    println!("Возраст : {}!", age);
}