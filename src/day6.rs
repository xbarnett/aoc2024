use std::collections::{HashMap, HashSet};

fn parse(input: String) -> HashMap<(i32, i32), char> {
  input.lines().enumerate().map(|(i, line)|
    line.chars().enumerate().map(move |(j, c)| ((i as i32, j as i32), c))
  ).flatten().collect()
}

fn solve(map: &HashMap<(i32, i32), char>, obstacle: Option<(i32, i32)>) ->
  Option<HashSet<(i32, i32)>>
{
  let mut pos = *map.iter().filter(|(_, &v)| v == '^').next().unwrap().0;
  let mut dir = 0;
  let mut states = HashSet::new();
  loop {
    if !states.insert((pos, dir)) {
      break None;
    }
    let next = (pos.0 + [-1, 0, 1, 0][dir], pos.1 + [0, 1, 0, -1][dir]);
    let Some(&there) = map.get(&next) else {
      break Some(states.iter().map(|&s| s.0).collect());
    };
    if there == '#' || Some(next) == obstacle {
      dir = (dir + 1) % 4;
    } else {
      pos = next;
    }    
  }
}

pub fn part_one(input: String) -> String {
  solve(&parse(input), None).unwrap().len().to_string()
}

pub fn part_two(input: String) -> String {
  let map = parse(input);
  solve(&map, None).unwrap().iter().filter(|&&obstacle| {
    map[&obstacle] == '.' && solve(&map, Some(obstacle)).is_none()
  }).count().to_string()
}
