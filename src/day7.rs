use std::collections::HashMap;

#[aoc(day7, part1, Char)]
pub fn solve_part1(input: &str) -> i32 {
  let mut current_path = "/".to_owned();
  let mut data: HashMap<String, i32> = HashMap::new();

  for s in input.split("\n") {
    if s.chars().nth(0).unwrap() == '$' {
      let mut command = s.split(" ");

      match command.nth(1).unwrap() {
        "cd" => {
          let dir = command.next().unwrap();
    
          match dir {
            "/" => current_path ="/".to_string(),
            ".." => {
              let parts:Vec<&str> = current_path.split("/").collect();
              current_path = parts[0..parts.len()-1].join("/");
            },
            _ => current_path =  format!("{}{}/", &current_path, dir),
          }
        },
        _ => continue
      }
    } else if s.split(" ").nth(0).unwrap() != "dir" {
      let value = s.split(" ").nth(0).unwrap().parse::<i32>().unwrap();

      let mut buildup = "".to_string();
      let parts: Vec<&str> = current_path.split("/").collect();

      for p in 0..&parts.len()-1 {
        buildup = format!("{}{}/", buildup, parts[p].to_string());
        data.entry(buildup.clone()).and_modify(|f| *f += value).or_insert(value);
      }
    }
  }

  let mut result = 0;

  for x in data.values() {
    if *x <= 100_000 {
      result += *x;
    }
  }

  return result
}