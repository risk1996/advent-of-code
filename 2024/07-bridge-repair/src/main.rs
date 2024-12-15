use std::collections::HashSet;

use aoc::bootstrap;

fn parse_input(line: &str) -> (u64, Vec<u64>) {
  let slices = line.split(": ").collect::<Vec<_>>();
  let result = slices[0].parse::<u64>().unwrap();
  let operands = slices[1]
    .split(" ")
    .map(|n| n.parse::<u64>().unwrap())
    .collect::<Vec<_>>()
    .iter()
    .rev() // NOTE: Reverse to act as a stack
    .map(|n| *n)
    .collect::<Vec<_>>();

  (result, operands)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Operators {
  Add,
  Mul,
  Concat,
}

fn test_add_mull_operator_permutations(
  result: u64,
  operands: &Vec<u64>,
  allowed_operators: &HashSet<Operators>,
) -> bool {
  if operands.len() == 0 {
    return false;
  } else if operands.len() == 1 {
    return operands[0] == result;
  } else {
    let mut operands = operands.clone();
    let first = operands.pop().unwrap();
    let second = operands.pop().unwrap();

    if allowed_operators.contains(&Operators::Add) {
      operands.push(first + second);
      if test_add_mull_operator_permutations(result, &operands, allowed_operators) {
        return true;
      }
      operands.pop().unwrap();
    }

    if allowed_operators.contains(&Operators::Mul) {
      operands.push(first * second);
      if test_add_mull_operator_permutations(result, &operands, allowed_operators) {
        return true;
      }
      operands.pop().unwrap();
    }

    if allowed_operators.contains(&Operators::Concat) {
      operands.push(format!("{first}{second}").parse::<u64>().unwrap());
      if test_add_mull_operator_permutations(result, &operands, allowed_operators) {
        return true;
      }
      operands.pop().unwrap();
    }

    false
  }
}

fn main() {
  let (input, mut output) = bootstrap();
  let input = input.split("\n").map(parse_input).collect::<Vec<_>>();

  let part_1_ops = HashSet::from([Operators::Add, Operators::Mul]);
  let part_1 = input
    .iter()
    .filter(|(r, ops)| test_add_mull_operator_permutations(*r, ops, &part_1_ops))
    .map(|(result, _)| result)
    .sum::<u64>();
  output.write_part(1, &part_1);

  let part_2_ops = HashSet::from([Operators::Add, Operators::Mul, Operators::Concat]);
  let part_2 = input
    .iter()
    .filter(|(r, ops)| test_add_mull_operator_permutations(*r, ops, &part_2_ops))
    .map(|(result, _)| result)
    .sum::<u64>();
  output.write_part(2, &part_2);
}
