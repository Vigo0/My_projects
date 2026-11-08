use std::io;
use std::fs;

fn main(){
    println!("Введите название реактива:");
    let mut reagent_name = String::new();
    io::stdin()
        .read_line(&mut reagent_name)
        .expect("error during reading");
    let reagent_name = reagent_name.trim().to_string();// независимая копия.

    println!("Введите количество (целое число):");
    let mut quantity_str = String::new();
    io::stdin()
        .read_line(&mut quantity_str)
        .expect("Не удалось прочитать количество");
    let quantity: u32 = quantity_str // задаем тип беззнаковое целое
        .trim() 
        .parse() //превращение в число строки
        .expect("Enter number from Z ring");

    let report = format!("Реактив [{}] поступил на склад в количестве [{}] шт.", reagent_name, quantity);
    // формат возвращает строку а не печатает в терминал

    println!("{}", report);

    fs::write("inventory.txt", &report).expect("Не удалось записать в файл");
    println!("Отчёт сохранён в inventory.txt");

}