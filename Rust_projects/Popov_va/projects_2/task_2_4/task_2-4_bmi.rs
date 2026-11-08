use std::io;

fn main() {
    println!("Введите ваш вес (кг):");
    let mut weight_input = String::new();
    io::stdin().read_line(&mut weight_input).expect("Ошибка ввода");
    let weight: f64 = weight_input.trim().parse().expect("Вы ввели не число");

    println!("Введите ваш рост (см):");
    let mut height_input = String::new();
    io::stdin().read_line(&mut height_input).expect("Ошибка ввода");
    let height: f64 = height_input.trim().parse().expect("Вы ввели не число");

    let height_meters = height / 100.0;
    let bmi = weight / (height_meters.powi(2));// нету возведения в квадрат, есть метод

    println!("\n--- Отчет о состоянии здоровья ---");
    println!("Рост:\t{} см", height);
    println!("Вес:\t{} кг", weight);
    println!("Индекс массы тела:\t{:.2}", bmi); // 2 знака после запятой
}