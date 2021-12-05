use std::fs;

struct Point {
  x: i32,
  y: i32,
}

struct PointTransition {
  from: Point,
  to: Point,
}

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");
  let mut results : [[i32; 1000]; 1000] = [[0; 1000]; 1000];
  let transitions : Vec<PointTransition> = data.lines().map(|line| {
    let raw_points : Vec<String> = line.split(" -> ").map(|p| {p.to_string()}).collect();
    let raw_start : Vec<i32> = raw_points[0].split(",").map(|n| {n.parse().unwrap()}).collect();
    let start = Point {
      x: raw_start[0],
      y: raw_start[1],
    };

    let raw_end : Vec<i32> = raw_points[1].split(",").map(|n| {n.parse().unwrap()}).collect();
    let end = Point {
      x: raw_end[0],
      y: raw_end[1],
    };

    let t = PointTransition{
      from: start,
      to: end,
    };
    t
  }).collect();

  for t in transitions {
    if t.from.x == t.to.x {
      let (start, end) = if t.from.y < t.to.y {(t.from.y, t.to.y)} else {(t.to.y, t.from.y)};
      for i in start as usize..=end as usize {
        results[t.from.x as usize][i] += 1;
      }
    }
    if t.from.y == t.to.y {
      let (start, end) = if t.from.x < t.to.x {(t.from.x, t.to.x)} else {(t.to.x, t.from.x)};
      for i in start as usize..=end as usize {
        results[i][t.from.y as usize] += 1;
      }
    }
  }

  let mut count = 0;

  for i in 0..1000 {
    for j in 0..1000 {
      if results[i][j] > 1 {
        count += 1;
      }
    }
  }

  println!("{}", count);
}