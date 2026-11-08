use std::io;
use std::fs;

fn main() {
    
    println!("Введите название питательной среды:");
    let mut medium_name = String::new();
    io::stdin()
        .read_line(&mut medium_name)
        .expect("Не удалось прочитать название среды");
    let medium_name = medium_name.trim().to_string();

    println!("Введите концентрацию агара (%):");
    let mut agar_concentration = String::new();
    io::stdin()
        .read_line(&mut agar_concentration)
        .expect("Не удалось прочитать концентрацию агара");
    let agar_concentration = agar_concentration.trim().to_string();

    println!("Введите температуру стерилизации (°C):");
    let mut sterilization_temp = String::new();
    io::stdin()
        .read_line(&mut sterilization_temp)
        .expect("Не удалось прочитать температуру стерилизации");
    let sterilization_temp = sterilization_temp.trim().to_string();

    let content = format!(
        "{}\n\
         - Концентрация агара: {}%\n\
         - Температура стерилизации: {}°C",
        medium_name, agar_concentration, sterilization_temp
    );
{
    fs::write("recipe.txt", &content)
        .expect("Не удалось записать файл");
}
    println!("Файл 'recipe.txt' успешно сформирован!");   
}
