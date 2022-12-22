pub struct Monkey {
  holding: Vec<i32>,
  operator: char,
  modifier: String,
  test_value: i32,
  to_true: i32,
  to_false: i32,
  inspected_items: i32,
}

impl Clone for Monkey {
  fn clone(&self) -> Monkey {
    Monkey {
      holding: self.holding.clone(),
      operator: self.operator,
      modifier: self.modifier.clone(),
      test_value: self.test_value,
      to_true: self.to_true,
      to_false: self.to_false,
      inspected_items: self.inspected_items,
    }
  }
}

impl Monkey {
  fn operation(&self, x: i32) -> i32 {
    if self.operator == '*' {
      if self.modifier == "old" {
        x * x
      } else {
        x * self.modifier.parse::<i32>().unwrap()
      }
    } else {
      if self.modifier == "old" {
        x + x
      } else {
        x + self.modifier.parse::<i32>().unwrap()
      }
    }
  }

  fn test(&self, x: i32) -> i32 {
    if x % self.test_value == 0 {
      self.to_true
    } else {
      self.to_false
    }
  }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Monkey> {
  let mut result: Vec<Monkey> = vec![];
  let mut input_iter = input.split("\n");

  loop {
    let mut starting_items = input_iter.nth(1).unwrap().split_whitespace();
    let mut operation_text = input_iter.next().unwrap().split_whitespace();
    let mut test = input_iter.next().unwrap().split_whitespace();

    let mut starting_numbers: Vec<i32> = vec![];
    starting_items.nth(1);

    loop {
      if let Some(s) = starting_items.next() {
        match s.strip_suffix(",") {
          Some(ss) => starting_numbers.push(ss.parse().unwrap()),
          None => starting_numbers.push(s.parse().unwrap()),
        }
      } else {
        break;
      }
    }

    let op = operation_text.nth(4).unwrap();
    let y = operation_text.next().unwrap();

    let divide_number: i32 = test.nth(3).unwrap().parse().unwrap();
    let to_true: i32 = input_iter
      .next()
      .unwrap()
      .split_whitespace()
      .nth(5)
      .unwrap()
      .parse()
      .unwrap();
    let to_false: i32 = input_iter
      .next()
      .unwrap()
      .split_whitespace()
      .nth(5)
      .unwrap()
      .parse()
      .unwrap();

    let monkey = Monkey {
      holding: starting_numbers,
      to_false,
      to_true,
      test_value: divide_number,
      operator: op.chars().next().unwrap(),
      modifier: y.to_owned(),
      inspected_items: 0
    };
    result.push(monkey);

    if let Some(_) = input_iter.next() {
      continue;
    } else {
      break;
    }
  }

  return result;
}

#[aoc(day11, part1, Char)]
pub fn solve_part1(input: &Vec<Monkey>) -> i32 {
  let mut monkeys = input.to_vec();

  for _ in 1..=20 {
    for x in 0..monkeys.len() {
      let monkey = monkeys[x].clone();
      for item in &monkey.holding {
        let mut value = monkey.operation(*item);
        value = value / 3;
        let to = monkey.test(value);


        monkeys[to as usize].holding.push(value);
        monkeys[x].inspected_items += 1;
      }
      monkeys[x].holding.clear();
    }
  }

  let mut times: Vec<i32>= vec![];

  for m in monkeys {
    times.push(m.inspected_items);
  }

  let mut a = 0;
  let mut b = 0;

  for t in times {
    if t > a {
      b = a;
      a = t;
    } else if t > b {
      b = t;
    }
  }

  return a * b;
}

// #[aoc(day11, part2, Char)]
// pub fn solve_part2(input: &str) -> i32 {

// }
