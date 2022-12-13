use core::panic;
use std::collections::HashSet;

struct Point {
  pub x: i32,
  pub y: i32
}

#[aoc(day9, part1, Char)]
pub fn solve_part1(input: &str) -> usize {
  let mut visited = HashSet::new();

  let mut head = Point{x: 0, y: 0};  
  let mut tail = Point{x: 0, y: 0};

  visited.insert(format!("[{}][{}]", tail.x, tail.y));

  for s in input.split("\n") {
    let direction = s.chars().nth(0).unwrap();
    let amount: i32 = s.split(" ").nth(1).unwrap().parse().unwrap();

    for _ in 0..amount {
      let old_head = Point{x: head.x, y: head.y};

      match direction {
        'R' => head.x += 1,
        'U' => head.y += 1,
        'L' => head.x -= 1,
        'D' => head.y -= 1,
        _ => panic!()
      }

      let x_difference = (head.x - tail.x).abs();
      let y_difference = (head.y - tail.y).abs();

      if x_difference > 1 || y_difference > 1 {
        tail.x = old_head.x;
        tail.y = old_head.y;
      }

      visited.insert(format!("[{}][{}]", tail.x, tail.y));
    }

  }

  return visited.len()
}