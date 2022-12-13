use std::vec;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
  let mut result: Vec<Vec<u32>> = vec![];

  for (y, s) in input.split("\n").enumerate() {
    result.push(vec![]);
    for ss in s.chars() {
      result[y].push(ss.to_digit(10).unwrap());
    }
  }

  return result
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<Vec<u32>>) -> i32 {
  let mut result = 0;

  for (y, row) in input.iter().enumerate() {
    for (x, height) in row.iter().enumerate() {
      if x == 0 || y == 0 || x == row.len()-1 || y == input.len()-1 {
        result += 1;
        continue;
      }

      let left = row[0..x].iter().max().unwrap();
      if height > left {
        result += 1;
        continue;
      }

      let right = row[x+1..].iter().max().unwrap();
      if height > right {
        result += 1;
        continue;
      }

      let collumn: Vec<u32> = input.iter().map(|row| row[x]).collect();

      let up = collumn[0..y].iter().max().unwrap();
      if height > up {
        result += 1;
        continue;
      }

      let down = collumn[y+1..].iter().max().unwrap();
      if height > down {
        result += 1;
        continue;
      }
    }
  }

  return result
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<Vec<u32>>) -> u32 {
  let mut scores: Vec<u32> = vec![];

  for (y, row) in input.iter().enumerate() {
    for (x, height) in row.iter().enumerate() {
      if x == 0 || y == 0 || x == row.len()-1 || y == input.len()-1 {
        continue;
      }

      let left = row[0..x].iter();
      let mut score_left: u32 = 0;
      for i in left.rev() {
        score_left += 1;
        if i >= height {
          break;
        }
      }

      let right = row[x+1..].iter();
      let mut score_right: u32 = 0;
      for i in right {
        score_right += 1;
        if i >= height {
          break;
        }
      }

      let collumn: Vec<u32> = input.iter().map(|row| row[x]).collect();

      let up = collumn[0..y].iter();
      let mut score_up: u32 = 0;
      for i in up.rev() {
        score_up += 1;
        if i >= height {
          break;
        }
      }

      let down = collumn[y+1..].iter();
      let mut score_down: u32 = 0;
      for i in down {
        score_down += 1;
        if i >= height {
          break;
        }
      }

      scores.push(score_left * score_right * score_up * score_down);
    }
  }

  return *scores.iter().max().unwrap();
}