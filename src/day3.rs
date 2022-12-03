#[aoc(day3, part1, Char)]
pub fn solve_part1(input: &str) -> i32 {
  let mut result = 0;

  for s in input.split("\n") {
    let a = &s[0..s.chars().count() / 2];
    let b = &s[s.chars().count() / 2..s.chars().count()];

    for sa in a.chars() {
        match b.find(sa) {
            None => continue,
            Some(_) => {
                result += parse(sa);
                break;
            }
        }
    }
  }

  return result;
}

fn parse(input: char) -> i32 {
    let i = input as i32;

    if i <= 90 {
        return i - 38;
    } else {
        return i - 96;
    }
}

#[aoc(day3, part2, Char)]
pub fn solve_part2(input: &str) -> i32 {
  let mut result = 0;

  let s: Vec<&str> = input.split("\n").collect();

  for i in (0..s.len()).step_by(3) {
    for sa in s[i].chars() {
      let rb = s[i+1].find(sa);
      let rc = s[i+2].find(sa);
      
      match (rb, rc) {
        (Some(_), Some(_)) => {
          result += parse(sa);
          break;
        }
        _ => continue,
      }
    }
  }

  return result;
}
