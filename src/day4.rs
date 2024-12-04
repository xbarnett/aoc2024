use std::collections::HashMap;

fn parse(input: String) -> (HashMap<(i32, i32), char>, i32, i32) {
  let grid: HashMap<_, _> = input.lines().enumerate().map(|(i, line)| {
    line.chars().enumerate().map(move |(j, c)| ((i as i32, j as i32), c))
  }).flatten().collect();
  let rows = grid.keys().map(|k| k.0).max().unwrap() + 1;
  let cols = grid.keys().map(|k| k.1).max().unwrap() + 1;
  (grid, rows, cols)
}

pub fn part_one(input: String) -> String {
  let deltas = [
    (-1, -1), (-1, 0), (-1, 1), (0, -1),
    (0, 1), (1, -1), (1, 0), (1, 1),
  ];

  let (grid, rows, cols) = parse(input);
  let mut result = 0;
  for i in 0 .. rows {
    for j in 0 .. cols {
      for delta in deltas {
        let word: Option<String> = (0 .. 4).map(|n| {
          grid.get(&(i + n * delta.0, j + n * delta.1))
        }).collect();
        if word == Some("XMAS".to_string()) {
          result += 1;
        }
      }
    }
  }
  result.to_string()
}

pub fn part_two(input: String) -> String {
  let coords = [
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
  ];

  let (grid, rows, cols) = parse(input);
  let mut result = 0;
  for i in 0 .. rows - 2 {
    for j in 0 .. cols - 2 {
      if coords.iter().map(|cs| cs.iter().map(|(x, y)| {
        grid[&(i + x, j + y)]
      }).collect()).all(|word: String| word == "MAS" || word == "SAM") {
        result += 1;
      }
    }
  }
  result.to_string()
}
