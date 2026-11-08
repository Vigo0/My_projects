use std::io;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    println!("Введите имя оператора:");
    let mut operator_name = String::new();
    io::stdin()
        .read_line(&mut operator_name)
        .expect("Не удалось прочитать имя оператора");
    let operator_name = operator_name.trim().to_string();

    println!("Введите текущее значение давления (Па):");
    let mut pressure_str = String::new();
    io::stdin()
        .read_line(&mut pressure_str)
        .expect("Не удалось прочитать значение давления");
    let pressure_str = pressure_str.trim().to_string();

    let log_entry = format!("{}\t{}\n", operator_name, pressure_str);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("sensor_log.txt")
        .expect("Не удалось открыть файл");

    file.write_all(log_entry.as_bytes())
        .expect("Не удалось записать");

    println!("Данные успешно сохранены в sensor_log.txt");
}