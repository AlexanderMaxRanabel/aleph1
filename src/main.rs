mod generate_salt;

use std::env;

use colored::*;
use rand::*;

fn generate_hash(characters: [char; 70], salt: String, salt_length: u8 ,plain_text_length: u8, hash_number: u8) -> String {
    let mut rng = rand::thread_rng();
    let mut char_list: Vec<char> = Vec::new();
    let mut i = 0;
    let mut index = (plain_text_length * plain_text_length) - salt_length + hash_number;
    while i < plain_text_length {
        let index_usize = index as usize;
        if index < 70 || index == 70 {
            char_list.push(characters[index_usize]);
            let random_1_2 = rng.gen_range(0..=3);
            if random_1_2 == 1 {
                index += 1;
            } else if random_1_2 == 2 {
                index += 2;
            }
        } else {
            index -= 1;
        }
        i += 1;
    }
    println!("{:?}", char_list);
    let hash: String = char_list.iter().collect();
    if salt_length < 1 {
        return hash;
    } else {
        let full_hash: String = salt + hash.as_str();
        return full_hash;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let plain_text = args.get(1).unwrap_or_else(|| {
            println!("{}: No Plain Text Provided", "Error".red());
            std::process::exit(1);
        }).clone();

        let salt_length = args.get(2).unwrap_or_else(|| {
            println!("{}: No Salt Length Provided", "Error".red());
            std::process::exit(1);
        }).clone();

        let hash_number = args.get(3).unwrap_or_else(|| {
            println!("{}: No Hash Number Provided", "Error".red());
            std::process::exit(1);
        }).clone();

        let salt_length: u8 = salt_length.parse().expect("Failed to convert to u8");
        let hash_number: u8 = hash_number.parse().expect("Failled to convert to u8");
        let plain_text_length: u8 = plain_text.len() as u8;

        let salt = generate_salt::salt_generate(salt_length);

        let characters: [char; 70] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            '!', '@', '#', '%', '&', '?', '_', '+',
        ];

        let hash = generate_hash(characters, salt, salt_length,plain_text_length, hash_number);
        println!("{}", hash.magenta());
    } else {
        println!("{}", "Aleph1".yellow().bold());
        println!("{}", "A For-Fun hashing system.".bold())
    }
}
