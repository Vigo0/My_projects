use std::io;

fn main(){
    println!("Введите число белков");
    let mut proteins = String::new();
    io::stdin().read_line(&mut proteins).expect("Ошибка ввода");// в расте нет считывания числа, нужно парсить
    let proteins: i32 = proteins.trim().parse().expect("Вы ввели не число");
    println!("Принято число белков: {} г", proteins);
   
    println!("Введите число жиров");
    let mut fats = String::new();
    io::stdin().read_line(&mut fats).expect("Ошибка ввода");// в расте нет считывания числа, нужно парсить
    let fats: i32 = fats.trim().parse().expect("Вы ввели не число");
    println!("Принято число жиров: {} г", fats);

    println!("Введите число carbs");
    let mut carbs = String::new();
    io::stdin().read_line(&mut carbs).expect("Ошибка ввода");// в расте нет считывания числа, нужно парсить
    let carbs: i32 = carbs.trim().parse().expect("Вы ввели не число");
    println!("Принято число углей: {} г", carbs);

    let calories :i32 = (proteins * 4) + (fats * 2) + (carbs * 4);

    println!("Продукт содержащий {}г белков, {}г жиров и {}г углеводов\nCодержит {} калорий",
proteins, fats, carbs, calories);
}