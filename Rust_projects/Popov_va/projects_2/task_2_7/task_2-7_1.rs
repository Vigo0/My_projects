fn main() {
    let files = vec!["seq1", "seq2", "seq3", "seq4"];
    let date = "2024-01-15";

    for name in files {
        // name тип &str срез строк
        let name_string = String::from(name);
        
        let new_name = name_string + "_" + date + ".fasta";
        
        println!("{}", new_name);
    }
}