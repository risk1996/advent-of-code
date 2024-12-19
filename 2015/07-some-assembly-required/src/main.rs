use std::collections::HashMap;

use aoc::bootstrap;

#[derive(Debug, PartialEq, Eq)]
enum Operand {
  Literal(u16),
  Wire(String),
}

fn parse_operand(raw: &str) -> Operand {
  if matches!(raw.chars().next().unwrap(), 'a'..='z') {
    Operand::Wire(raw.to_string())
  } else {
    Operand::Literal(raw.parse().unwrap())
  }
}

fn get_operand_value(
  system: &HashMap<String, Instruction>,
  cache: &mut HashMap<String, u16>,
  operand: &Operand,
) -> u16 {
  if let Operand::Wire(w) = operand {
    if let Some(v) = cache.get(w) {
      return *v;
    }
  }

  let v = match operand {
    | Operand::Literal(v) => *v,
    | Operand::Wire(w) => get_wire_value(system, cache, &w),
  };

  if let Operand::Wire(w) = operand {
    cache.insert(w.to_string(), v);
  }

  v
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
  Signal(Operand),
  And(Operand, Operand),
  Or(Operand, Operand),
  Not(Operand),
  LeftShift(Operand, Operand),
  RightShift(Operand, Operand),
}

fn parse_instruction(line: &str) -> (String, Instruction) {
  let parts = line.split(" -> ").collect::<Vec<_>>();
  let instruction = parts[0];
  let wire = parts[1].to_string();

  let instruction = if instruction.contains(" AND ") {
    let ops = instruction.split(" AND ").collect::<Vec<_>>();
    let l = parse_operand(ops[0]);
    let r = parse_operand(ops[1]);

    Instruction::And(l, r)
  } else if instruction.contains(" OR ") {
    let ops = instruction.split(" OR ").collect::<Vec<_>>();
    let l = parse_operand(ops[0]);
    let r = parse_operand(ops[1]);

    Instruction::Or(l, r)
  } else if instruction.contains("NOT ") {
    let v = parse_operand(&instruction.replace("NOT ", ""));

    Instruction::Not(v)
  } else if instruction.contains(" LSHIFT ") {
    let ops = instruction.split(" LSHIFT ").collect::<Vec<_>>();
    let l = parse_operand(ops[0]);
    let r = parse_operand(ops[1]);

    Instruction::LeftShift(l, r)
  } else if instruction.contains(" RSHIFT ") {
    let ops = instruction.split(" RSHIFT ").collect::<Vec<_>>();
    let l = parse_operand(ops[0]);
    let r = parse_operand(ops[1]);

    Instruction::RightShift(l, r)
  } else {
    let v = parse_operand(instruction);

    Instruction::Signal(v)
  };

  (wire, instruction)
}

fn get_wire_value(
  system: &HashMap<String, Instruction>,
  cache: &mut HashMap<String, u16>,
  wire: &str,
) -> u16 {
  match system.get(wire).unwrap() {
    | Instruction::Signal(v) => get_operand_value(system, cache, v),
    | Instruction::And(l, r) => {
      get_operand_value(system, cache, &l) & get_operand_value(system, cache, &r)
    },
    | Instruction::Or(l, r) => {
      get_operand_value(system, cache, &l) | get_operand_value(system, cache, &r)
    },
    | Instruction::Not(w) => !get_operand_value(system, cache, &w),
    | Instruction::LeftShift(l, r) => {
      get_operand_value(system, cache, &l) << get_operand_value(system, cache, &r)
    },
    | Instruction::RightShift(l, r) => {
      get_operand_value(system, cache, &l) >> get_operand_value(system, cache, &r)
    },
  }
}

fn main() {
  let (input, mut output) = bootstrap();

  let mut system = input.lines().map(|line| parse_instruction(line)).fold(
    HashMap::<String, Instruction>::new(),
    |mut acc, (key, value)| {
      acc.insert(key, value);
      acc
    },
  );
  let mut cache = HashMap::<String, u16>::new();
  let part_1 = get_wire_value(&system, &mut cache, "a");
  output.write_part(1, &part_1);

  system.insert(
    "b".to_string(),
    Instruction::Signal(Operand::Literal(part_1)),
  );
  cache.clear();
  let part_2 = get_wire_value(&system, &mut cache, "a");
  output.write_part(2, &part_2);
}
