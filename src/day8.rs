use std::collections::{HashMap, HashSet};

fn parse(input: String) -> (i32, i32, HashMap<char, Vec<(i32, i32)>>) {
  let height = input.lines().count();
  let width = input.lines().next().unwrap().len();
  let mut board = HashMap::new();
  for (i, line) in input.lines().enumerate() {
    for (j, c) in line.chars().enumerate() {
      if c != '.' {
        board.entry(c).or_insert(Vec::new()).push((i as i32, j as i32));
      }
    }
  }
  (height as i32, width as i32, board)
}

fn solve(input: String, resonance: bool) -> String {
  let (height, width, board) = parse(input);
  let mut antinodes = HashSet::new();
  for antennas in board.values() {
    for (x1, y1) in antennas {
      for (x2, y2) in antennas {
        if (x1, y1) == (x2, y2) {
          continue;
        }
        let rays: Vec<Box<dyn Iterator<Item = i32>>> = if resonance {
          vec![Box::new(0 ..), Box::new((1 ..).map(|x| -x))]
        } else {
          vec![Box::new([-1, 2].into_iter())]
        };
        for ray in rays {
          for n in ray {              
            let new = (x1 + n * (x2 - x1), y1 + n * (y2 - y1));
            if 0 <= new.0 && new.0 < height && 0 <= new.1 && new.1 < width {
              antinodes.insert(new);
            } else {
              break;
            }
          }
        }
      }
    }
  }
  antinodes.len().to_string()
}

pub fn part_one(input: String) -> String {
  solve(input, false)
}

pub fn part_two(input: String) -> String {
  solve(input, true)
}
