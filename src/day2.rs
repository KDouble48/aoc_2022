use phf::phf_map;

static RPSMAP: phf::Map<char,i32> = phf_map! {
  'A' => 1,
  'B' => 2,
  'C' => 3,
  
  'X' => 1,
  'Y' => 2,
  'Z' => 3,
};

#[aoc(day2, part1, Char)]
pub fn solve_part1(input: &str) -> i32 {
  let mut score = 0;

  for s in input.split("\n") {
    let elf = s.chars().nth(0).unwrap();
    let player = s.chars().nth(2).unwrap();
    
    score += RPSMAP.get(&player).unwrap();

    match wrap(RPSMAP.get(&player).unwrap() - RPSMAP.get(&elf).unwrap()) {
      0 => score += 3,
      1 => score += 6,
      -1 => score += 0,
      _ => panic!(),
    }
  }

  return score
}

fn wrap(input: i32) -> i32 {
  if input > 1 {
    return wrap(input - 3)
  }
  if input < -1 {
    return  wrap(input + 3)
  }

  return input;
}