use std::collections::HashSet;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::fs::{self, File};
use rayon::prelude::*;

fn word_to_int(word: &str) -> u32 {
    let mut ret: u32 = 0;
    for c in word.chars() {
        // Skip words that are not alphabetical
        if ('a'..='z').contains(&c) {
            let idx = (c as u8 - b'a') as u32;
            ret |= 1 << idx;
        }
    }
    ret
}

fn build_reverse_map(words: &HashSet<String>) -> HashMap<u32, Vec<String>> {
    let mut map: HashMap<u32, Vec<String>> = HashMap::new();
    for word in words {
        let mask = word_to_int(word);
        map.entry(mask).or_default().push(word.clone());
    }
    map
}

fn can_play(guess: u32, state: u32) -> bool {
    state & guess == 0
}

fn get_all_that_can_play(guesses: &Vec<u32>, state: u32) -> Vec<u32> {
    guesses
        .iter()
        .filter(|&g| can_play(*g, state))
        .copied()
        .collect()
}

fn search_for_bad_games(
    allowed_guesses: &Vec<u32>,
    state: u32,
    answer: &u32,
    remaining_guesses: usize,
    word_list: &mut Vec<u32>,
    last_word: &u32,
    reverse_allow: &HashMap<u32, Vec<String>>,
    reverse_answer: &HashMap<u32, Vec<String>>,
    writer: &mut File
) -> bool {
    // Found a solution
    if remaining_guesses == 0 {
        let allow: Vec<String> = word_list.iter()
            .map(|&mask| {
                reverse_allow
                    .get(&mask)
                    .map(|v| v.join("/"))
                    .unwrap_or_else(|| format!("{:026b}", mask))
            })
            .collect();

        let answer = reverse_answer
            .get(answer)
            .map(|v| v.join("/"))
            .unwrap_or_else(|| format!("{:026b}", answer)); // fallback


        writeln!(writer, "{} -> {}", answer, allow.join(", ")).unwrap();
        return false;
    }

    let can_play : Vec<u32> = get_all_that_can_play(allowed_guesses, state);

    if can_play.len() == 0 {
        return false;
    }

    for new_guess in &can_play {
        if (new_guess <= last_word) {
            continue;
        }
        word_list.push(*new_guess);
        let must_stop : bool = search_for_bad_games(&can_play, state | new_guess, answer, remaining_guesses - 1, word_list, new_guess, reverse_allow, reverse_answer, writer);
        word_list.pop();
        if must_stop {
            return true;
        }
    }
    false
}

fn search(allowed_path: &str, answer_path: &str, output_folder: &str, number_of_guesses: usize) {
    let allowed_content = fs::read_to_string(allowed_path).unwrap();
    let allowed_words_str: HashSet<String> = allowed_content.lines().map(|line| line.to_string()).collect();
    let answer_content = fs::read_to_string(answer_path).unwrap();
    let answer_words_str: HashSet<String> = answer_content.lines().map(|line| line.to_string()).collect();
    let mut allowed_words: Vec<u32> = allowed_words_str.iter().map(|w| word_to_int(w)).collect();
    let mut answer_words: Vec<u32> = answer_words_str.iter().map(|w| word_to_int(w)).collect();

    // Sorting these makes it so that they have some kind of order.
    // The order is useful for optimisations. Lets say we have words indexed from
    // 1 to 100, and lets say at depth 0 we're at index 9, we know, to avoid
    // duplicates, that for the next depth we should start at index > 9.
    allowed_words.sort();
    answer_words.sort();

    // These are used to know what word(s) each integer word corresponds to.
    let reverse_allow = build_reverse_map(&allowed_words_str);
    let reverse_answer = build_reverse_map(&answer_words_str);

    let out_dir = Path::new(output_folder);
    fs::create_dir_all(out_dir).unwrap();

    // Multithreading loop for all answers
    answer_words.par_iter().for_each(|&answer| {
        let answer_str = reverse_answer
            .get(&answer)
            .map(|v| v.join("_"))
            .unwrap_or_else(|| format!("{:026b}", answer));

        let file_path = out_dir.join(format!("{}.txt", answer_str));
        let mut file = File::create(file_path).unwrap();

        search_for_bad_games(
            &(allowed_words.iter().copied().collect()),
            answer,
            &answer,
            number_of_guesses,
            &mut Vec::with_capacity(number_of_guesses),
            &answer,
            &reverse_allow,
            &reverse_answer,
            &mut file
        );

        println!("Done with answer '{}'", answer_str);
    });

    println!("Done!");
}
fn main() {
    search("data/allowed.txt", "data/answers.txt", "data/solutions", 6);
}
