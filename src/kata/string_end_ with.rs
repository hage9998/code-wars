// Complete the solution so that it returns true if the first argument(string) passed in ends with the 2nd argument (also a string).

fn solution(word: &str, ending: &str) -> bool {
  word.ends_with(ending)
}

fn solution2(word: &str, ending: &str) -> bool {
  let l_word = word.len();
  let l_end = ending.len();
  if l_end > l_word {
    false
  } else {
    (&word[l_word-l_end..]).eq(ending)
  }
}