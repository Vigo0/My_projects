use std::io; // ввод вывод из стандартной библиотеки

fn main(){
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("error");

    let user_input = user_input.trim();

    let proceed_input = user_input.to_uppercase();

    println!("{} -> {}", user_input, proceed_input);
}