#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<i32> {
  let mut result: Vec<i32> = vec![];
  let mut current_value = 1;
  result.push(current_value);

  for s in input.split("\n") {
    result.push(current_value);

    if s != "noop" {
      current_value += s.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
      result.push(current_value);
    }
  }

  return result
}

#[aoc(day10, part1, Char)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
  let cycles = [20,60,100,140,180,220];
  let mut result: i32 = 0;

  for x in cycles {
    result += x as i32 * input[x-1];
  }

  return result;
}

#[aoc(day10, part2, Char)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
  let mut input_iter = input.iter();

  for _ in 0..6 {
    let mut line = "".to_owned();
    for x in 0..40 {
      if (x - *input_iter.next().unwrap()).abs() <= 1 {
        line = format!("{}{}", line, "#")
      } else {
        line = format!("{}{}", line, ".")
      }
    }
    println!("{}", line);
  }

  return 0
}