use std::collections::HashMap;

use aoc::bootstrap;

fn count_vowels(input: &str) -> usize {
  input
    .chars()
    .filter(|c| matches!(c, 'a' | 'i' | 'u' | 'e' | 'o'))
    .collect::<Vec<_>>()
    .len()
}

fn check_double_letters(input: &str) -> bool {
  std::iter::zip(input.chars(), input.chars().skip(1)).any(|(left, right)| left == right)
}

fn check_naughty_substring(input: &str) -> bool {
  input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy")
}

fn collect_letter_pair_indices(input: &str) -> HashMap<(char, char), Vec<usize>> {
  std::iter::zip(input.chars(), input.chars().skip(1))
    .enumerate()
    .fold(
      HashMap::<(char, char), Vec<usize>>::new(),
      |mut acc, (i, pair)| {
        acc.entry(pair).or_insert(Vec::new()).push(i);
        acc
      },
    )
}

fn check_same_letter_with_1_skip(input: &str) -> bool {
  std::iter::zip(input.chars(), input.chars().skip(2)).any(|(left, right)| left == right)
}

fn main() {
  let (input, mut output) = bootstrap();

  let step1 = input
    .split("\n")
    .filter(|line| {
      let has_three_vowels = count_vowels(&line) >= 3;
      let has_double_letters = check_double_letters(&line);
      let is_naughty = check_naughty_substring(&line);

      !is_naughty && has_three_vowels && has_double_letters
    })
    .count();
  let step2 = input
    .split("\n")
    .filter(|line| {
      let has_letter_pairs = collect_letter_pair_indices(&line)
        .iter()
        .any(|(_, indices)| indices.len() > 2 || indices.len() == 2 && indices[1] - indices[0] > 1);
      let has_same_letter_with_1_skip = check_same_letter_with_1_skip(&line);

      has_letter_pairs && has_same_letter_with_1_skip
    })
    .count();

  output.writeln(&format!("Step 1: {step1}\nStep 2: {step2}"));
}
