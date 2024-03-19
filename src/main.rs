use rand::Rng;
use serde::Deserialize;
use std::{fs, io, num::Saturating};

#[derive(Debug, Deserialize)]
struct Record {
    from: String,
    to: String,
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("ERROR: Couldn't read from stdin!");
    if input.trim().is_empty() {
        input.push_str("words.json");
    }
    input
}

fn main() {
    let file_name = get_input("Enter words file in json format(words.json): ");
    let file =
        fs::read_to_string(&file_name.trim()).expect(&format!("Couldn't read from {}", file_name));
    let words: Vec<Record> = serde_json::from_str(&file).unwrap();
    let mut rng = rand::thread_rng();
    let mut score: Saturating<u32> = Saturating(0);

    loop {
        let rand_num = rng.gen_range(0..words.len());
        let rand_record = words.get(rand_num).unwrap();

        println!("Word to translate: {}", rand_record.from);

        let mut input = String::with_capacity(rand_record.to.len() + 1);
        io::stdin()
            .read_line(&mut input)
            .expect("ERROR: Couldn't read from stdin!");
        let input = input.trim();

        if input == rand_record.to {
            score += 1;
        } else {
            println!("\nCorrect answer is: {}", rand_record.to);
            println!("You entered: {}\n", input);
            break;
        }
    }
    println!("Your score: {}", score);
}
