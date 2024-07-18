/*
    Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
*/

use std::collections::BTreeMap;
use std::io;

fn vowel_check(word: &String) -> bool {
    word.to_lowercase().starts_with(&['a', 'e', 'i', 'o', 'u'])
}

fn ay_treatment(word: String) -> String {
    let mut chars_btreemap: BTreeMap<i32, char> = BTreeMap::new();
    let mut result_string: String = String::new();
    let mut count: i32 = 0;

    for c in word.chars() {
        chars_btreemap.insert(count, c);
        count += 1;
    }

    for (k, c) in &chars_btreemap {
        if *k != 0 {
            result_string.push(*c);
        }
    }

    let first_consonant = chars_btreemap.get(&0).unwrap().to_string();
    let final_string = format!("-{}ay", first_consonant);

    result_string.push_str(&final_string);

    result_string
}

fn hay_treatment(word: String) -> String {
    format!("{}-hay", word)
}

fn main() {
    // Flow control for the user's word
    let mut user_word = String::new();

    // Print the instructions to use the program
    println!("Write any word: \n");
    // Take the user's input
    io::stdin()
        .read_line(&mut user_word)
        .expect("This isn't a word");

    match vowel_check(&user_word) {
        true => {
            println!("{}", hay_treatment(user_word));
        }
        false => {
            println!("{}", ay_treatment(user_word));
        }
    };
}
