#[aoc(day5, part1, Char)]
pub fn solve_part1(input: &str) -> String {
  let mut state: Vec<Vec<char>> = vec![vec![];9];

  let lines = input.split("\n").collect::<Vec<&str>>();

  for (_x,s) in lines[0..8].iter().enumerate() {
    for y in 0..9 {
      let new_char = s.chars().nth(y*4+1).unwrap();
      if new_char != ' ' {
        state[y].insert(0, new_char);
      }
    }
  }

  for s in lines[10..lines.len()].iter() {
    let mut ss = s.split(" ");
    
    let times = ss.nth(1).unwrap().parse().unwrap();
    let from: usize = ss.nth(1).unwrap().parse().unwrap();
    let to: usize = ss.nth(1).unwrap().parse().unwrap();

    for _ in 0..times {
      let value = state[from-1].pop().unwrap();
      state[to-1].push(value);
    }
  }

  let mut result = "".to_owned();

  for x in state {
    result = format!("{}{}", result, x.last().unwrap());
  }

  return result
}

#[aoc(day5, part2, Char)]
pub fn solve_part2(input: &str) -> String {
  let mut state: Vec<Vec<char>> = vec![vec![];9];

  let lines = input.split("\n").collect::<Vec<&str>>();

  for (_x,s) in lines[0..8].iter().enumerate() {
    for y in 0..9 {
      let new_char = s.chars().nth(y*4+1).unwrap();
      if new_char != ' ' {
        state[y].insert(0, new_char);
      }
    }
  }

  for s in lines[10..lines.len()].iter() {
    let mut ss = s.split(" ");
    
    let amount: usize = ss.nth(1).unwrap().parse().unwrap();
    let from: usize = ss.nth(1).unwrap().parse().unwrap();
    let to: usize = ss.nth(1).unwrap().parse().unwrap();

    let from_stack = &mut state[from-1];
    let values = from_stack[from_stack.len()-amount..from_stack.len()].to_owned();
    
    for x in (from_stack.len()-amount..from_stack.len()).rev() {
      from_stack.remove(x);
    }
    
    state[to-1].extend(values);
  }

  let mut result = "".to_owned();

  for x in state {
    result = format!("{}{}", result, x.last().unwrap());
  }

  return result
}