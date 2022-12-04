#[aoc(day4, part1, Char)]
pub fn solve_part1(input: &str) -> i32 {
  let mut result = 0;
  for s in input.split("\n"){
    let a = to_string(s.split(",").nth(0).unwrap());
    let b = to_string(s.split(",").nth(1).unwrap());
    
    if a.contains(&b) || b.contains(&a) {
      result += 1
    }
  }
  
  return result
}

fn to_string(input: &str) -> String {
  let start: i32 = input.split("-").nth(0).unwrap().parse().unwrap();
  let end: i32 = input.split("-").nth(1).unwrap().parse().unwrap();
  
  let mut result = "".to_owned(); 
  
  for i in start..end+1 {
    result = format!("{}'{}'", result, i)
  }

  return result
}

// #[aoc(day4, part2, Char)]
// pub fn solve_part2(input: &str) -> i32 {
// }