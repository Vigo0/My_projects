use std::io;
use std::fs;

fn main() {
    // 1. Собираем данные
    println!("Введите ФИО исследователя:");
    let mut fio = String::new();
    io::stdin().read_line(&mut fio).expect("Ошибка ввода");
    let fio = fio.trim().to_string();

    println!("Введите дату (ДД.ММ.ГГГГ):");
    let mut date = String::new();
    io::stdin().read_line(&mut date).expect("Ошибка ввода");
    let date = date.trim().to_string();

    println!("Введите название эксперимента:");
    let mut experiment = String::new();
    io::stdin().read_line(&mut experiment).expect("Ошибка ввода");
    let experiment = experiment.trim().to_string();

    println!("Введите вывод:");
    let mut conclusion = String::new();
    io::stdin().read_line(&mut conclusion).expect("Ошибка ввода");
    let conclusion = conclusion.trim().to_string();

    // 2. Формируем рамку
    // Ширина рамки 60 символов
    let width = 60;
    let border = "+".to_string() + &"-".repeat(width) + "+";

    // 3. Собираем весь текст журнала
    let journal = format!(
        "{border}\n\
         | Электронный лабораторный журнал{padding1}|\n\
         {border}\n\
         | ФИО исследователя : {fio}{padding2}|\n\
         | Дата              : {date}{padding3}|\n\
         | Эксперимент       : {experiment}{padding4}|\n\
         {border}\n\
         | Вывод: {conclusion}{padding5}|\n\
         {border}",
        border = border,
        fio = fio,
        date = date,
        experiment = experiment,
        conclusion = conclusion,
        // Исправленные формулы: используем .chars().count() и безопасное вычитание
        padding1 = " ".repeat(width.saturating_sub(33)), 
        padding2 = " ".repeat(width.saturating_sub(22 + fio.chars().count())),
        padding3 = " ".repeat(width.saturating_sub(22 + date.chars().count())),
        padding4 = " ".repeat(width.saturating_sub(22 + experiment.chars().count())),
        padding5 = " ".repeat(width.saturating_sub(9 + conclusion.chars().count())),
    );

    // 4. Записываем в файл через байты
    fs::write("journal.txt", journal.as_bytes())
        .expect("Не удалось записать в файл");

    println!("Журнал сохранён в journal.txt");
}