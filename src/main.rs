use std::collections::HashMap;

mod utils;

fn main() {
    let words = utils::get_words_from_file();

    let wrong_spelled_word = "lovr";

    let suggestions = spell_check(wrong_spelled_word, &words);

    println!("Closest suggestions for {}:", wrong_spelled_word);

    for (suggestion, distance) in suggestions {
        println!("- {} (distance: {})", suggestion, distance);
    }
}

fn spell_check(word: &str, words: &[String]) -> Vec<(String, i32)> {
    let mut distances: HashMap<String, i32> = HashMap::new();

    for correct_word in words.iter() {
        let distance = wagner_fischer(word, correct_word);
        distances.insert(correct_word.to_string(), distance);
    }

    let mut suggestions: Vec<_> = distances.into_iter().collect();

    suggestions.sort_by(|a, b| {
        let distance_cmp = a.1.cmp(&b.1);
    
        if distance_cmp == std::cmp::Ordering::Equal {
            a.0.to_lowercase().cmp(&b.0.to_lowercase())
        } else {
            distance_cmp
        }
    });     

    suggestions.truncate(10);

    suggestions
}

fn wagner_fischer(s1: &str, s2: &str) -> i32 {
    let len_s1 = s1.len();
    let len_s2 = s2.len();

    if len_s1 > len_s2 {
        return wagner_fischer(s2, s1);
    }

    let mut current_row = vec![0; len_s1 + 1];

    for i in 1..=len_s1 {
        current_row[i] = i;
    }

    for i in 1..=len_s2 {
        let previous_row = current_row.clone();

        current_row[0] = i;

        for j in 1..=len_s1 {
            let insertion = current_row[j - 1] + 1;
            let deletion = previous_row[j] + 1;
            let substituion = previous_row[j - 1]
                + if s1.chars().nth(j - 1) != s2.chars().nth(i - 1) {
                    1
                } else {
                    0
                };

            current_row[j] = std::cmp::min(insertion, std::cmp::min(deletion, substituion));
        }
    }
    current_row[len_s1].try_into().unwrap()
}
