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