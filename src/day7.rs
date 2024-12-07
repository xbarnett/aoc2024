use std::ops::{Add, Mul};

fn parse(input: String) -> Vec<(i64, Vec<i64>)> {
  input.lines().map(|line| {
    let (test, numbers) = line.split_once(": ").unwrap();
    (test.parse().unwrap(), numbers.split(' ').map(|n| n.parse().unwrap()).collect())
  }).collect()
}

fn can_make(test: i64, current: i64, numbers: &[i64], ops: &[fn(i64, i64) -> i64]) -> bool {
  if numbers.is_empty() {
    test == current
  } else {    
    ops.iter().map(|op|
      can_make(test, op(current, numbers[0]), &numbers[1 ..], ops)
    ).any(|b| b)
  }
}

fn solve(input: String, ops: &[fn(i64, i64) -> i64]) -> String {
  let mut result = 0;
  for (test, numbers) in parse(input) {
    if can_make(test, numbers[0], &numbers[1 ..], ops) {
      result += test;
    }
  }
  result.to_string()
}

pub fn part_one(input: String) -> String {
  solve(input, &[i64::add, i64::mul])
}

pub fn part_two(input: String) -> String {
  solve(input, &[i64::add, i64::mul,
    |x, y| [x.to_string(), y.to_string()].concat().parse().unwrap()
  ])
}
