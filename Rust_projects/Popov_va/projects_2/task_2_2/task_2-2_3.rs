fn main() {
    let device_name = "Микроскоп";
    let inventory_number = "INV-001";
    let is_working = "Исправен";
    let quantity = 5;

    println!("Прибор\t\tИнв. номер\tСостояние\tКол-во");
    println!("{}\t{}\t\t{}\t{}", device_name, inventory_number, is_working, quantity);
}