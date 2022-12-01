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