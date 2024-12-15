use std::collections::{HashSet, HashMap};

fn parse(input: String, wide: bool) -> (HashMap<(i32, i32), char>, Vec<(i32, i32)>) {
  let (board_str, moves) = input.split_once("\n\n").unwrap();

  let mut board = HashMap::new();
  for (i, line) in board_str.lines().enumerate() {
    for (j, c) in line.chars().enumerate() {
      let (i, j) = (i as i32, j as i32);
      if wide {
        let (a, b) = match c {
          '#' => ('#', '#'),
          'O' => ('[', ']'),
          '.' => ('.', '.'),
          '@' => ('@', '.'),
          _ => panic!(),
        };
        board.insert((i, 2 * j), a);
        board.insert((i, 2 * j + 1), b);
      } else {
        board.insert((i, j), c);
      }
    }
  }

  let moves = moves.chars().map(|c| match c {
    '^' => vec![(-1, 0)],
    'v' => vec![(1, 0)],
    '<' => vec![(0, -1)],
    '>' => vec![(0, 1)],
    _ => vec![],
  }).flatten().collect();

  (board, moves)
}

fn solve(mut board: HashMap<(i32, i32), char>, moves: Vec<(i32, i32)>) -> String {
  let mut position = *board.iter().filter(|(_, &v)| v == '@').next().unwrap().0;
  for (di, dj) in moves {
    let mut good = true;
    let mut to_process = vec![position];
    let mut to_push = HashSet::new();
    while let Some((i, j)) = to_process.pop() {
      let c = board[&(i, j)];
      if c == '#' {
        good = false;
        break;
      }
      if c == '.' {
        continue;
      }
      to_push.insert((i, j));
      to_process.push((i + di, j + dj));
      if dj == 0 {
        if c == '[' && !to_push.contains(&(i, j + 1)) {
          to_process.push((i, j + 1));
        }
        if c == ']' && !to_push.contains(&(i, j - 1)) {
          to_process.push((i, j - 1));
        }
      }
    }
    if good {
      position = (position.0 + di, position.1 + dj);
      let mut new_board = board.clone();
      for &(i, j) in &to_push {
        new_board.insert((i, j), '.');
      }
      for &(i, j) in &to_push {
        new_board.insert((i + di, j + dj), board[&(i, j)]);
      }
      board = new_board;
    }
  }

  let mut res = 0;
  for ((i, j), c) in board {
    if c == 'O' || c == '[' {
      res += 100 * i + j;
    }
  }
  res.to_string()
}

pub fn part_one(input: String) -> String {
  let (board, moves) = parse(input, false);
  solve(board, moves)
}

pub fn part_two(input: String) -> String {
  let (board, moves) = parse(input, true);
  solve(board, moves)
}
