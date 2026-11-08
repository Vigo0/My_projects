fn main() {
    let seq = vec!["ATATACGCGTA", "CTTCGGNGGA"];

    for s in seq {
        // s тип &str 
        println!("{}", s);
        
        for letter in s.chars() {
            // letter тип char (
            println!("{}", letter);
        }
    }
    
    println!("Цикл выполнен");
}