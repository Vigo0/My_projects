use std::io;

fn main() {
    println!("Введите общее количество произведенных капсул:");
    let mut capsules_input = String::new();
    io::stdin().read_line(&mut capsules_input).expect("Ошибка ввода");
    let capsules: i32 = capsules_input.trim().parse().expect("Вы ввели не число");

    println!("Введите количество капсул в одной упаковке:");
    let mut pack_size_input = String::new();
    io::stdin().read_line(&mut pack_size_input).expect("Ошибка ввода");
    let pack_size: i32 = pack_size_input.trim().parse().expect("Вы ввели не число");

    let full_packs = capsules / pack_size;// если хотим не целочисленное надо использовать приведение типов (capsules as f64)
    
    let remainder = capsules % pack_size;

    println!("\n--- Отчет фасовочного цеха ---");
    println!("Полных упаковок:\t{}", full_packs);
    println!("Остаток капсул:\t{}", remainder);
}