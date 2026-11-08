use std::io;

fn main() {
    println!("=== Анализ последовательности ДНК ===\n");

    println!("Введите последовательность ДНК:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");

    let dna = input.trim().to_uppercase();// верхний регистр

    println!("\nПоследовательность в верхнем регистре: {}", dna);


    let count_a = dna.chars().filter(|&c| c == 'A').count();// разбиваем по символам(char), оставляем только то что нам нужно ну и считаем
    let count_t = dna.chars().filter(|&c| c == 'T').count();
    let count_g = dna.chars().filter(|&c| c == 'G').count();
    let count_c = dna.chars().filter(|&c| c == 'C').count();

    println!("\nПодсчёт нуклеотидов:");
    println!("A: {}", count_a);
    println!("T: {}", count_t);
    println!("G: {}", count_g);
    println!("C: {}", count_c);

    //в Rust len возвращает количество байт, а не символов
    // Для Unicode-строк нужно использовать .chars().count()
    let total = dna.chars().count();

    println!("\nОбщая длина: {} нуклеотидов", total);

    println!("\nПроцентное содержание:");
    println!("A: {:.2}%", (count_a as f64 / total as f64) * 100.0);
    println!("T: {:.2}%", (count_t as f64 / total as f64) * 100.0);
    println!("G: {:.2}%", (count_g as f64 / total as f64) * 100.0);
    println!("C: {:.2}%", (count_c as f64 / total as f64) * 100.0);
}