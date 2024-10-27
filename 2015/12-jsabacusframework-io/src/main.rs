use aoc::bootstrap;
use serde_json::{from_str, Value};

fn sum_all_leaf_number(value: &Value, count: i64) -> i64 {
  match value {
    | Value::Array(vec) => vec
      .iter()
      .fold(count, |acc, item| acc + sum_all_leaf_number(item, 0)),
    | Value::Null => 0,
    | Value::Bool(_) => 0,
    | Value::String(_) => 0,
    | Value::Number(v) => v.as_i64().unwrap(),
    | Value::Object(maps) => maps
      .values()
      .fold(count, |acc, item| acc + sum_all_leaf_number(item, 0)),
  }
}

fn sum_all_leaf_number_without_red(value: &Value, count: i64) -> i64 {
  match value {
    | Value::Array(vec) => vec.iter().fold(count, |acc, item| {
      acc + sum_all_leaf_number_without_red(item, 0)
    }),
    | Value::Null => 0,
    | Value::Bool(_) => 0,
    | Value::String(_) => 0,
    | Value::Number(v) => v.as_i64().unwrap(),
    | Value::Object(maps) => {
      let has_red = maps
        .values()
        .any(|v| v.eq(&Value::String("red".to_string())));

      match has_red {
        | true => 0,
        | false => maps.values().fold(count, |acc, item| {
          acc + sum_all_leaf_number_without_red(item, 0)
        }),
      }
    },
  }
}

fn main() {
  let (input, mut output) = bootstrap();

  let json: Value = from_str(&input).unwrap();

  let part_1 = sum_all_leaf_number(&json, 0);
  output.write_part(1, &part_1);

  let part_2 = sum_all_leaf_number_without_red(&json, 0);
  output.write_part(2, &part_2);
}
