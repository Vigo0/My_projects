use std::io;

fn main() {
    println!("Введите значение pH:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");
    let ph: f64 = input.trim().parse().expect("Вы ввели не число");

    if ph < 7.0 {
        println!("Кислая среда");
    } else if ph > 7.0 {
        println!("Щелочная среда");
    } else {
        println!("Нейтральная среда"); //nконвертировать число в строку
    }
}