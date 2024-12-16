use std::collections::{HashMap, HashSet};

fn solve(input: String) -> (i32, Vec<(i32, i32, i32)>,
    HashMap<(i32, i32, i32), Vec<(i32, i32, i32)>>) {
  let board: HashMap<_, _> = input.lines().enumerate().map(|(i, line)|
    line.chars().enumerate().map(move |(j, c)| ((i as i32, j as i32), c)
  )).flatten().collect();
  let mut states = HashMap::new();
  let mut weights = HashMap::new();
  let mut start = None;
  let mut ends = None;
  for (&(i, j), c) in &board {
    match c {
      '#' => continue,
      'S' => start = Some((i, j, 0)),
      'E' => ends = Some((0 .. 4).map(|k| (i, j, k)).collect::<Vec<_>>()),
      _ => (),
    };
    for (k, &(di, dj)) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter().enumerate() {
      let k = k as i32;
      let mut next_states = vec![
        ((i, j, (k + 1) % 4), 1000), ((i, j, (k + 3) % 4), 1000)
      ];
      if board[&(i + di, j + dj)] != '#' {
        next_states.push(((i + di, j + dj, k), 1));
      }
      states.insert((i, j, k), next_states.iter().map(|&v| v.0).collect::<Vec<_>>());
      weights.extend(next_states.iter().map(|&(next, weight)|
        (((i, j, k), next), weight)
      ));
    }
  }
  let (start, ends) = (start.unwrap(), ends.unwrap());

  let mut distances: HashMap<_, _> = states.keys().map(|&k|
    (k, if k == start { Some(0) } else { None })
  ).collect();
  let mut predecessors: HashMap<_, _> = states.keys().map(|&k|
    (k, vec![])
  ).collect();
  let mut to_process: HashSet<_> = states.keys().map(|&k| k).collect();
  loop {
    let mut closest = None;
    for &state in &to_process {
      if closest.is_none() || distances[&closest.unwrap()].is_none() ||
          (distances[&state].is_some() && distances[&state] <= distances[&closest.unwrap()]) {
        closest = Some(state);
      }
    }
    let Some(closest) = closest else { break; };
    to_process.remove(&closest);

    for &next in &states[&closest] {
      let distance = Some(distances[&closest].unwrap() + weights[&(closest, next)]);
      if distances[&next].is_none() || distance <= distances[&next] {
        if distances.insert(next, distance) != Some(distance) {
          predecessors.insert(next, vec![]);
        }
        predecessors.get_mut(&next).unwrap().push(closest);
      }
    }
  }

  let mut distances: HashMap<(i32, i32, i32), i32> = HashMap::new();
  for &end in &ends {
    let mut state = end;
    let mut distance = 0;
    while let Some(&prev) = predecessors[&state].first() {
      distance += weights[&(prev, state)];
      state = prev;
    }
    distances.insert(end, distance);
  }
  let shortest_distance = *distances.values().min().unwrap();
  let ends = ends.iter().filter_map(|&e| {
    if distances[&e] == shortest_distance { Some(e) } else { None }
  }).collect();
  (shortest_distance, ends, predecessors)
}

pub fn part_one(input: String) -> String {
  solve(input).0.to_string()
}

pub fn part_two(input: String) -> String {
  let (_, ends, predecessors) = solve(input);
  let mut to_process = ends;
  let mut processed: HashSet<(i32, i32, i32)> = HashSet::new();
  while let Some(state) = to_process.pop() {
    processed.insert(state);
    for &prev in &predecessors[&state] {
      if !processed.contains(&prev) {
        to_process.push(prev);
      }
    }
  }
  let spots: HashSet<_> = processed.into_iter().map(|(i, j, _)| (i, j)).collect();
  spots.len().to_string()
}
