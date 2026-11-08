use std::io;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("Введите нужный объем раствора (мл):");
    let mut volume_input = String::new();
    io::stdin().read_line(&mut volume_input).expect("Ошибка ввода");
    let volume: f64 = volume_input.trim().parse().expect("Вы ввели не число");

    // Расчет массы соли: 0.9% концентрация
    let salt_mass = volume * 0.009;
    // Объем воды примерно равен общему объему
    let water_volume = volume;

    // Открываем файл для записи
    {
    let mut file = File::create("recipe.txt").expect("Не удалось создать файл");

    // Записываем рецепт в файл
    //file.write_str(&format!("Масса соли:\t{:.2} г\n", salt_mass)).expect("Ошибка"); можно так
    writeln!(file, "ОТЧЕТ ПО ПРИГОТОВЛЕНИЮ:\n").expect("Ошибка записи");
    writeln!(file, "-----------------------\n").expect("Ошибка записи");
    writeln!(file, "Общий объем:\t{:.2} мл", volume).expect("Ошибка записи");
    writeln!(file, "Масса соли:\t{:.2} г", salt_mass).expect("Ошибка записи");
    writeln!(file, "Объем воды:\t{:.2} мл", water_volume).expect("Ошибка записи");

    println!("Рецепт сохранен в файл recipe.txt");
    } // аналог with тут закроет файл сам
}