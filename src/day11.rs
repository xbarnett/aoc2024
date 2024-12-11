use std::collections::HashMap;

fn solve(input: String, blinks: u64) -> String {
  let mut rocks: HashMap<u64, u64> = HashMap::new();
  for n in input.split_whitespace() {
    *rocks.entry(n.parse().unwrap()).or_default() += 1;
  }
  for _ in 0 .. blinks {
    let mut new_rocks = HashMap::new();
    for (rock, count) in rocks {
      let name = rock.to_string();
      if rock == 0 {
        *new_rocks.entry(1).or_default() += count;
      } else if name.len() % 2 == 0 {
        let (left, right) = name.split_at(name.len() / 2);
        *new_rocks.entry(left.parse().unwrap()).or_default() += count;
        *new_rocks.entry(right.parse().unwrap()).or_default() += count;
      } else {
        *new_rocks.entry(rock * 2024).or_insert(0) += count;
      }
    }
    rocks = new_rocks;
  }
  rocks.into_values().sum::<u64>().to_string()
}

pub fn part_one(input: String) -> String {
  solve(input, 25)
}

pub fn part_two(input: String) -> String {
  solve(input, 75)
}
