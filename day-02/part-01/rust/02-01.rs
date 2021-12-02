use std::fs;

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");
  // split each line into a tuple of a string and an integer
  let lines: Vec<(String, i32)> = data.lines().map(|line| {
    let mut parts = line.split_whitespace();
    (parts.next().unwrap().to_string(), parts.next().unwrap().parse::<i32>().unwrap())
  }).collect();

  let mut vertical = 0;
  let mut horizontal = 0;

  for (direction, distance) in lines {
    if direction == "forward" {
      vertical += distance;
    } else if direction == "down" {
      horizontal += distance;
    } else if direction == "up" {
      horizontal -= distance;
    }
  }

  println!("{}", vertical*horizontal);
}
