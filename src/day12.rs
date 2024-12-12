use std::collections::{HashMap, HashSet};

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn parse(input: String) -> Vec<HashSet<(i32, i32)>> {
  let map: HashMap<_, _> = input.lines().enumerate().map(|(i, l)| 
    l.chars().enumerate().map(move |(j, c)| ((i as i32, j as i32), c))
  ).flatten().collect();
  let mut result = vec![];
  let mut processed: HashSet<(i32, i32)> = HashSet::new();
  for (&(i, j), c) in &map {
    if processed.contains(&(i, j)) {
      continue;
    }
    let mut region = HashSet::new();
    let mut to_process = vec![(i, j)];
    while let Some((i, j)) = to_process.pop() {
      region.insert((i, j));
      to_process.extend(DIRS.iter().map(|&(di, dj)| (i + di, j + dj)).
        filter(|adj| map.get(adj) == Some(c) && !region.contains(adj))
      );
    }
    processed.extend(&region);
    result.push(region);
  }
  result
}

fn num_sides(region: HashSet<(i32, i32)>, combine: bool) -> usize {
  let mut edges: HashMap<_, _> = DIRS.iter().map(|&dir|
    (dir, HashSet::new())
  ).collect();
  for (i, j) in region {
    for (di, dj) in DIRS {
      if !edges.get_mut(&(-di, -dj)).unwrap().remove(&(i + di, j + dj)) {
        edges.get_mut(&(di, dj)).unwrap().insert((i, j));
      }
    }
  }
  if !combine {
    return edges.values().map(HashSet::len).sum()
  }

  let mut result = 0;
  for ((i, j), edges) in edges {
    let mut processed = HashSet::new();
    for edge in &edges {
      if processed.contains(edge) {
        continue;
      }
      for (di, dj) in [(j, i), (-j, -i)] {
        let (mut x, mut y) = edge;
        while edges.contains(&(x, y)) {
          processed.insert((x, y));
          (x, y) = (x + di, y + dj);
        }
      }
      result += 1;
    }
  }
  result
}

fn solve(input: String, combine: bool) -> String {
  parse(input).into_iter().map(|region|
    region.len() * num_sides(region, combine)
  ).sum::<usize>().to_string()
}

pub fn part_one(input: String) -> String {
  solve(input, false)
}

pub fn part_two(input: String) -> String {
  solve(input, true)
}
