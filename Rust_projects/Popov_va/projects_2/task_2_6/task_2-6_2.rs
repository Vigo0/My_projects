use std::io;

fn main() {
    println!("Какая температура листа?:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");
    let temp: f64 = input.trim().parse().expect("Вы ввели не число");

    if temp < 5.0 {
        println!("cold");
    } else if temp >= 5.0 && temp <= 25.0 {
        println!("normal_1");
    } else if temp >= 25.0 && temp <= 35.0 {
        println!("normal_2");
    } else {
        println!("hot");
    }
}