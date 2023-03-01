/**
 * count_words
 * Count up words into tuples and sort high count -> low count
 * cargo run bacon.txt
 */
use std::cmp::Reverse;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let contents = match env::args().nth(1) {
        Some(f) => match fs::read_to_string(f) {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Could not read file: {}", e);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Program requires a file path argument: <file_path>");
            std::process::exit(1);
        }
    };

    // accumulate all words
    let all_words = contents.split_whitespace().collect::<Vec<&str>>();

    // count how many times each unique word occurs
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    for word in all_words.iter() {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    // determine the most commonly used word(s)
    let mut top_count = 0u32;
    let mut top_words: Vec<&str> = Vec::new();

    for (&key, &val) in word_counts.iter() {
        if val > top_count {
            top_count = val;
            top_words.clear();
            top_words.push(key)
        } else if val == top_count {
            top_words.push(key);
        }
    }

    println!("Top word is {:?} {} times.", top_words, top_count);

    // Step 2: get Vector from HashMap.
    let mut sorted: Vec<_> = word_counts.iter().collect();
    println!("{:?}", sorted);

    // Step 3: sort Vector by key from HashMap.
    // ... This sorts by HashMap keys.
    //     Each tuple is sorted by its first item.
    sorted.sort_by_key(|a| Reverse(*a.1));

    // Step 4: loop over sorted vector.
    for (key, value) in sorted.iter() {
        println!("KEY, VALUE: {} {}", key, value);
    }
}
