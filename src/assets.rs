use rand::prelude::*;

// Returns the toml config file as a String
pub fn get_toml_data() -> String {
    let contents = include_str!("..\\assets\\wordle.toml");
    contents.to_string()
}

// Returns a random word chosen from the embedded file "words.txt"
pub fn get_random_word() -> Vec<u8> {
    let words: Vec<String> = include_str!("..\\assets\\words.txt")
        .split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    let last = words.len();
    let index =  rand::thread_rng().gen_range(0..last);
    words[index].as_bytes().to_vec()
}
