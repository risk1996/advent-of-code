use std::collections::HashSet;

use aoc::bootstrap;

fn get_next_valid_char(old: &char) -> (char, bool) {
  if *old == 'z' {
    ('a', true)
  } else if *old == 'h' || *old == 'k' || *old == 'n' {
    let next = *old as u8;
    // NOTE: Skips 'i', 'l', and 'o' chars
    let next = (next + 2) as char;
    (next, false)
  } else {
    let next = *old as u8;
    let next = (next + 1) as char;
    (next, false)
  }
}

fn get_next_password_chars(chars: &Vec<char>, i: usize) -> Vec<char> {
  let mut parts = chars.clone();
  let (next, carry) = get_next_valid_char(&parts[i]);
  parts[i] = next;

  // NOTE: Assuming leftmost character will not overflow
  match carry {
    | true => get_next_password_chars(&parts, i - 1),
    | false => parts,
  }
}

fn get_next_password(old: &str) -> String {
  let parts: Vec<char> = old.chars().collect();
  get_next_password_chars(&parts, parts.len() - 1)
    .iter()
    .collect()
}

fn has_increasing_3_letters(pwd: &str) -> bool {
  std::iter::zip(
    std::iter::zip(pwd.chars(), pwd.chars().skip(1)),
    pwd.chars().skip(2),
  )
  .map(|((a, b), c)| (a as u8, b as u8, c as u8))
  .any(|(a, b, c)| b - a == 1 && c - b == 1)
}

fn no_invalid_characters(pwd: &str) -> bool {
  pwd.chars().all(|c| c != 'i' && c != 'o' && c != 'l')
}

fn has_2_same_letter_pairs(pwd: &str) -> bool {
  std::iter::zip(pwd.chars(), pwd.chars().skip(1))
    .filter(|(a, b)| a == b)
    .map(|(c, _)| c)
    .collect::<HashSet<_>>()
    .len()
    >= 2
}

fn is_password_valid(pwd: &str) -> bool {
  has_increasing_3_letters(pwd) && no_invalid_characters(pwd) && has_2_same_letter_pairs(pwd)
}

fn get_next_valid_password(old: &str) -> String {
  let mut next = old.to_string();
  loop {
    next = get_next_password(&next);

    if is_password_valid(&next) {
      return next;
    }
  }
}

fn main() {
  let (input, mut output) = bootstrap();

  let part_1 = get_next_valid_password(&input);
  output.write_part(1, &part_1);

  let part_2 = get_next_valid_password(&part_1);
  output.write_part(2, &part_2);
}
