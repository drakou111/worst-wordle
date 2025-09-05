use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;
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
    reverse_allow: &HashMap<u32, Vec<String>>,
    reverse_answer: &HashMap<u32, Vec<String>>,
) -> bool {
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

        println!("{} -> {}", answer, allow.join(", "));
        return true;
    }

    let can_play : Vec<u32> = get_all_that_can_play(allowed_guesses, state);

    if can_play.len() == 0 {
        return false;
    }

    for new_guess in &can_play {
        word_list.push(*new_guess);
        let must_stop : bool = search_for_bad_games(&can_play, state | new_guess, answer, remaining_guesses - 1, word_list, reverse_allow, reverse_answer);
        word_list.pop();
        if must_stop {
            return true;
        }
    }
    false
}

fn search(allowed_path: &str, answer_path: &str, number_of_guesses: usize) {
    let allowed_content = fs::read_to_string(allowed_path).unwrap();
    let allowed_words_str: HashSet<String> = allowed_content.lines().map(|line| line.to_string()).collect();
    let answer_content = fs::read_to_string(answer_path).unwrap();
    let answer_words_str: HashSet<String> = answer_content.lines().map(|line| line.to_string()).collect();
    let allowed_words: Vec<u32> = allowed_words_str.iter().map(|w| word_to_int(w)).collect();
    let answer_words: Vec<u32> = answer_words_str.iter().map(|w| word_to_int(w)).collect();

    // These are used to know what word(s) each integer word corresponds to.
    let reverse_allow = build_reverse_map(&allowed_words_str);
    let reverse_answer = build_reverse_map(&answer_words_str);

    // Multithreading loop for all answers
    answer_words.par_iter().for_each(|&answer| {
        let answer_str = reverse_answer
            .get(&answer)
            .map(|v| v.join("/"))
            .unwrap_or_else(|| format!("{:026b}", answer));

        // println!("Trying: {}", answer_str);

        search_for_bad_games(
            &(allowed_words.iter().copied().collect()),
            answer,
            &answer,
            number_of_guesses,
            &mut Vec::with_capacity(number_of_guesses),
            &reverse_allow,
            &reverse_answer
        );
    });

    println!("Done!");
}
fn main() {
    search("data/allowed.txt", "data/answers.txt", 6);
}
