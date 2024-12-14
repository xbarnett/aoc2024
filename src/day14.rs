use std::{cmp::Ordering, collections::HashSet};

fn parse(input: String) -> Vec<(i32, i32, i32, i32)> {
  input.lines().map(|line| {
    let nums: Vec<_> = line.split(&['=', ',', ' ']).map(|n| n.parse().ok()).collect();
    (nums[1].unwrap(), nums[2].unwrap(), nums[4].unwrap(), nums[5].unwrap())
  }).collect()
}

fn tick(robots: Vec<(i32, i32, i32, i32)>) -> Vec<(i32, i32, i32, i32)> {
  robots.into_iter().map(|(x, y, vx, vy)|
    ((x + vx).rem_euclid(101), (y + vy).rem_euclid(103), vx, vy)
  ).collect()
}

pub fn part_one(input: String) -> String {
  let mut robots = parse(input);
  for _ in 0 .. 100 {
    robots = tick(robots);
  }

  let mut counts = [0, 0, 0, 0];
  for (x, y, _, _) in robots {
    match (x.cmp(&50), y.cmp(&51)) {
      (Ordering::Less, Ordering::Less) => counts[0] += 1,
      (Ordering::Less, Ordering::Greater) => counts[1] += 1,
      (Ordering::Greater, Ordering::Less) => counts[2] += 1,
      (Ordering::Greater, Ordering::Greater) => counts[3] += 1,
      _ => (),
    };
  }
  counts.iter().product::<i32>().to_string()
}

pub fn part_two(input: String) -> String {
  let mut robots = parse(input);
  let mut time = 0;
  loop {
    let positions: HashSet<_> = robots.iter().map(|&(x, y, _, _)| (x, y)).collect();
    if (26 .. 59).all(|i| positions.contains(&(37, i))) {
      break;
    }
    time += 1;
    robots = tick(robots);
  }
  time.to_string()
}
