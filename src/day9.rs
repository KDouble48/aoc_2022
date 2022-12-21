use core::panic;
use std::collections::HashSet;

#[derive(Debug)]
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

#[aoc(day9, part2, Char)]
pub fn solve_part2(input: &str) -> usize {
  let mut visited = HashSet::new();

  let mut rope: Vec<Point> = vec![];
  for _ in 0..10 {
    rope.push(Point{x: 0, y:0})
  }

  visited.insert(format!("[{}][{}]", rope[9].x, rope[9].y));

  for s in input.split("\n") {
    let direction = s.chars().nth(0).unwrap();
    let amount: i32 = s.split(" ").nth(1).unwrap().parse().unwrap();

    for _ in 0..amount {
      match direction {
        'R' => rope[0].x += 1,
        'U' => rope[0].y += 1,
        'L' => rope[0].x -= 1,
        'D' => rope[0].y -= 1,
        _ => panic!()
      }

      for x in 0..rope.len()-1 {
        let head = Point{x: rope[x].x, y: rope[x].y};
        let tail = &mut rope[x+1];
        
        let x_difference = head.x - tail.x; // 4
        let x_abs_difference = x_difference.abs(); // 4
        
        let y_difference = head.y - tail.y;
        let y_abs_difference = y_difference.abs();
        
        if x_abs_difference > 1 || y_abs_difference > 1 {
          if x_difference > 0 {
            tail.x += 1;
          }
          if x_difference < 0 {
            tail.x += -1;
          }
          
          if y_difference > 0 {
            tail.y += 1;
          }
          if y_difference < 0 {
            tail.y += -1;
          }
        }
      }

      visited.insert(format!("[{}][{}]", rope[9].x, rope[9].y));
    }

  }
  
  return visited.len()
}