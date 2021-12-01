use std::fs;

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");
  let split_data: Vec<i32> = data.split_whitespace()
    .map(|x| x.parse::<i32>().unwrap())
    .collect();

  let mut last_depth = split_data[0];
  let mut increase_count = 0;

  for i in split_data {
    if i > last_depth {
      increase_count += 1;
    }
    last_depth = i;
  }
  

  println!("{}", increase_count);
}
