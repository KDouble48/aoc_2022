pub struct Map {
  nodes: Vec<Vec<Node>>,
  possible_nodes: Vec<(u32, u32)>,
  start: (u32, u32),
  end: (u32, u32)
}

pub struct Node {
  height: u8,
  steps: u32,
}

impl Clone for Node {
  fn clone (&self) -> Node {
    Node {
      height: self.height,
      steps: self.steps,
    }
  }
}

impl Clone for Map {
  fn clone(&self) -> Map {
    Map {
      nodes: self.nodes.clone(),
      possible_nodes: self.possible_nodes.clone(),
      start: self.start,
      end: self.end
    }
  }
}

impl Map {
  pub fn steps(&mut self) -> u32 {
    self.possible_nodes.push(self.start);
    let modifier: [i32; 3] = [-1, 0, 1];
    self.nodes[self.start.1 as usize][self.start.0 as usize].steps = 0;

    loop {
      let closest_node = self.get_closest_possible_node();
      let (nodex, nodey) = closest_node;
      let steps = self.nodes[nodey as usize][nodex as usize].steps;
      let height = self.nodes[nodey as usize][nodex as usize].height;
  
      if closest_node == self.end {
        return steps
      }


      for ymod in modifier {
        let y = nodey as i32 + ymod;
        if y < 0 || y as usize >= self.nodes.len() {
          continue;
        }
        for xmod in modifier {
          let x = nodex as i32 + xmod; 
          if x < 0 || x as usize >= self.nodes[y as usize].len() {
            continue;
          }

          if xmod != 0 && ymod != 0 {
            continue;
          }

          let node = &mut self.nodes[y as usize][x as usize];

          if node.height > height && (node.height).abs_diff(height) > 1 {
            continue;
          }

          if steps + 1 < node.steps  {
            self.possible_nodes.push((x as u32,y as u32));
            node.steps = steps + 1;
          }

        }
      }
    }
  }

  pub fn distance_to(&self, from: &(u32, u32), to: &(u32, u32)) -> u32 {
    let (fx, fy) = from;
    let (tx, ty) = to;
    
    return fx.abs_diff(*tx)+fy.abs_diff(*ty);
  }

  pub fn get_closest_possible_node(&mut self) -> (u32, u32) {
    let mut shortest_distance = u32::MAX;
    let mut closest_node = (0,0);
    let mut index = 0;

    for (i, n) in self.possible_nodes.iter().enumerate() {
      let a = self.distance_to(n, &self.end);
      let b = self.distance_to(n, &self.start);
      if a+b < shortest_distance {
        shortest_distance = a;
        closest_node = *n;
        index = i;
      }
    }

    self.possible_nodes.remove(index);

    return closest_node
  }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Map {
  let mut result = Map{nodes: vec![], possible_nodes: vec![], start: (0,0), end: (0,0)};
  let mut start = (0,0);
  let mut end = (0,0);

  for (y, s) in input.split("\n").enumerate() {
    let mut row = vec![];
    
    for (x, ss) in s.chars().enumerate() {
      match ss {
        'S' => {
          start = (x as u32,y as u32);
          row.push(Node{height:'a' as u8, steps: u32::MAX })
        },
        'E' => {
          end = (x as u32,y as u32);
          row.push(Node{height: 'z' as u8, steps: u32::MAX })
        },
        _ => row.push(Node{height: ss as u8, steps: u32::MAX  })
    }
    }

    result.nodes.push(row)
  }

  result.start = start;
  result.end = end;

  return result
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Map) -> u32 {

  let mut map = input.clone();
  
  let steps = map.steps();

  return steps
}