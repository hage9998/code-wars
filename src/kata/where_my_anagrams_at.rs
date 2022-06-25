// Write a function that will find all the anagrams of a word from a list. You will be given two inputs a word and an array with words. 
// You should return an array of all the anagrams or an empty array if there are none. For example:
// anagrams('abba', ['aabb', 'abcd', 'bbaa', 'dada']) => ['aabb', 'bbaa']

fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let mut sorted_word = word.chars().collect::<Vec<char>>();
    sorted_word.sort();
    let vec_str: Vec<String> = words.iter().cloned().filter(|w| {
        let mut sorted_w = w.as_str().chars().collect::<Vec<char>>();
        sorted_w.sort();
        sorted_word == sorted_w
    }).collect();
    vec_str
}

// --------------------------------------------------------------------------------------

use itertools::Itertools;

fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let cs = word.chars().sorted().collect_vec();
    words.iter().filter(|s| s.chars().sorted().collect_vec() == cs).cloned().collect()
}

// --------------------------------------------------------------------------------------

fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let a = normalize(word);
    words
        .iter()
        .filter(|word| a == normalize(word))
        .cloned()
        .collect::<Vec<_>>()
}

fn normalize(word: &str) -> Vec<char> {
    let mut a = word.chars().collect::<Vec<_>>();
    a.sort();
    a
}