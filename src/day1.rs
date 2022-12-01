#[aoc(day1, part1, Char)]
pub fn solve_part1(input: &str) -> i32 {
  let mut result = 0;

  for s in input.split("\n\n") {
    let mut i = 0;
    for ss in s.split("\n") {
      i += ss.parse::<i32>().unwrap()
    }

    if i > result {
      result = i
    }
  }

  return result
}

#[aoc(day1, part2, Char)]
pub fn solve_part2(input: &str) -> i32 {
  let mut array: [i32; 3] = [0,0,0];

  for s in input.split("\n\n") {
    let mut i = 0;
    for ss in s.split("\n") {
      i += ss.parse::<i32>().unwrap()
    }

    let min = array.iter().min().unwrap();
    if i > *min {
      let index = array.iter().position(|&x| x == *min).unwrap();
      array[index] = i;
    }

  }

  return array.iter().sum()
}