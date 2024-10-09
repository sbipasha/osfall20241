use std::collections::HashMap;

fn most_frequent_word(text: &str) -> (String, usize) {
    let mut word_count = HashMap::new();

    // Split the text by whitespace and count the occurrences of each word
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    // Find the word with the maximum count
    let mut max_word = String::new();
    let mut max_count = 0;

    for (word, &count) in &word_count {
        if count > max_count {
            max_word = word.to_string();
            max_count = count;
        }
    }

    (max_word, max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
