use std::io;

fn main() {
    println!("Введите группу крови донора (I, II, III, IV):");
    let mut donor_input = String::new();
    io::stdin().read_line(&mut donor_input).expect("Ошибка ввода");
    let donor = donor_input.trim().to_uppercase();

    println!("Введите группу крови реципиента (I, II, III, IV):");
    let mut recipient_input = String::new();
    io::stdin().read_line(&mut recipient_input).expect("Ошибка ввода");
    let recipient = recipient_input.trim().to_uppercase();

    let can_donate = if donor == "I" { // if возвращает значение
        true  // I группа может отдавать всем
    } else if donor == "II" {
        recipient == "II" || recipient == "IV"
    } else if donor == "III" {
        recipient == "III" || recipient == "IV"
    } else if donor == "IV" {
        recipient == "IV"
    } else {
        println!("Неверная группа крови донора");
        return;
    };

    if can_donate {
        println!("Переливание возможно: донор {} может отдать кровь реципиенту {}", donor, recipient);
    } else {
        println!("Переливание невозможно: донор {} не может отдать кровь реципиенту {}", donor, recipient);
    }
}