#![deny(unsafe_code)]
#![forbid(non_ascii_idents)]
#![forbid(unsafe_attr_outside_unsafe)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![forbid(unused_qualifications)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::{env, io};

fn main() {
  let mut args = env::args();
  args.next();
  let day = args.next().unwrap().parse().unwrap();
  let part = args.next().unwrap().parse().unwrap();
  let solve = match (day, part) {
    (1, 1) => day1::part_one,
    (1, 2) => day1::part_two,
    (2, 1) => day2::part_one,
    (2, 2) => day2::part_two,
    (3, 1) => day3::part_one,
    (3, 2) => day3::part_two,
    (4, 1) => day4::part_one,
    (4, 2) => day4::part_two,
    (5, 1) => day5::part_one,
    (5, 2) => day5::part_two,
    (6, 1) => day6::part_one,
    (6, 2) => day6::part_two,
    (7, 1) => day7::part_one,
    (7, 2) => day7::part_two,
    (8, 1) => day8::part_one,
    (8, 2) => day8::part_two,
    _ => panic!(),
  };
  let input = io::read_to_string(io::stdin()).unwrap();
  let output = solve(input);
  println!("{output}");
}
