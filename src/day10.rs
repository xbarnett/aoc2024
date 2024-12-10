use std::collections::{HashMap, HashSet, VecDeque};

fn solve(input: String) -> Vec<Vec<usize>> {
  let map: HashMap<(i32, i32), i32> = input.lines().enumerate().map(|(i, l)| 
    l.chars().enumerate().map(move |(j, c)|
      ((i as i32, j as i32), c.to_string().parse().unwrap())
    )
  ).flatten().collect();

  let mut result = vec![];
  for (&(i, j), &h) in &map {
    if h != 0 {
      continue;
    }
    let mut num_paths = HashMap::from([((i, j), 1)]);
    let mut remaining = VecDeque::from([(i, j)]);
    let mut processed = HashSet::new();
    while let Some((i, j)) = remaining.pop_front() {
      if !processed.insert((i, j)) {
        continue;
      }
      for (di, dj) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
        let next = (i + di, j + dj);
        if map.get(&next) == Some(&(map[&(i, j)] + 1)) {
          *num_paths.entry(next).or_insert(0) += num_paths[&(i, j)];
          remaining.push_back(next);
        }
      }
    }
    result.push(num_paths.into_iter().filter(|e| map[&e.0] == 9).
      map(|e| e.1).collect()
    );
  }
  result
}

pub fn part_one(input: String) -> String {
  solve(input).iter().map(Vec::len).sum::<usize>().to_string()
}

pub fn part_two(input: String) -> String {
  solve(input).iter().flatten().sum::<usize>().to_string()
}
