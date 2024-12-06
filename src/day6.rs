use std::collections::{HashMap, HashSet};

fn parse(input: String) -> HashMap<(i32, i32), char> {
  input.lines().enumerate().map(|(i, line)|
    line.chars().enumerate().map(move |(j, c)| ((i as i32, j as i32), c))
  ).flatten().collect()
}

fn solve(map: &HashMap<(i32, i32), char>, obstacle: Option<(i32, i32)>) -> Option<usize> {
  let mut pos = *map.iter().filter(|(_, &v)| v == '^').next().unwrap().0;
  let mut dir = 0;
  let mut states = HashSet::new();
  loop {
    if states.contains(&(pos, dir)) {
      return None;
    }
    states.insert((pos, dir));

    let next = (pos.0 + [-1, 0, 1, 0][dir], pos.1 + [0, 1, 0, -1][dir]);
    let Some(&there) = map.get(&next) else {
      return Some(states.iter().map(|&s| s.0).collect::<HashSet<_>>().len());
    };

    if there == '#' || Some(next) == obstacle {
      dir = (dir + 1) % 4;
    } else {
      pos = next;
    }    
  }
}

pub fn part_one(input: String) -> String {
  solve(&parse(input), None).unwrap().to_string()
}

pub fn part_two(input: String) -> String {
  let map = parse(input);
  let mut result = 0;
  for (&k, &v) in &map {
    if v == '.' && solve(&map, Some(k)).is_none() {
      result += 1;
    }
  }
  result.to_string()
}
