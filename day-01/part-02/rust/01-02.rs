use std::fs;

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");
  let split_data: Vec<i32> = data.split_whitespace()
    .map(|x| x.parse::<i32>().unwrap())
    .collect();

  let mut last_sum = split_data[0] + split_data[1] + split_data[2];
  let mut increase_count = 0;
  for i in 1..split_data.len()-2 {
    let sum = split_data[i] + split_data[i+1] + split_data[i+2];
    if sum > last_sum {
      increase_count += 1;
    }
    last_sum = sum;
  }

  println!("{}", increase_count);
}
