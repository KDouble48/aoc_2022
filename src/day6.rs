use std::collections::HashSet;

#[aoc(day6, part1, Char)]
pub fn solve_part1(input: &str) -> usize {
  for x in 0..input.len() {
    let values = &mut input[x..x+4].chars().collect::<Vec<char>>();

    let mut uniques = HashSet::new();
    values.retain(|v| uniques.insert(v.clone()));

    if values.len() == 4 {
      return x+4;
    }
  }

  return 0;
}

// #[aoc(day6, part2, Char)]
// pub fn solve_part2(input: &str) -> i32 {

// }