use std::fs;

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");
  let mut count = 0;
  for line in data.lines() {
    for output in line.split("|").last().unwrap().trim().split(" ") {
      match output.len() {
        2 | 3 | 4 | 7 => count += 1,
        _ => {}
      }
    };
  }

  println!("{}", count);
}
