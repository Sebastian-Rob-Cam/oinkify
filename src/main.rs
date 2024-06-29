fn vowel_check(word: &String) -> bool {
    word.to_lowercase().starts_with(&['a', 'e', 'i', 'o', 'u'])
}

fn hay_treatment(word: String) -> String {
    let s = format!("{word}-hay");

    s
}

fn main() {
    // wtt = word_to_test
    let wtt_vowel_function = String::from("Eat");
    assert!(vowel_check(&wtt_vowel_function));
}
