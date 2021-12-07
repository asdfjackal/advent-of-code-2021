use std::fs;
use std::slice::Iter;

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");
  let crabs: Vec<i32> = data.split(",").map(|crab| {
    crab.parse().unwrap()
  }).collect();
  let max = *crabs.iter().max().unwrap();
  let min = *crabs.iter().min().unwrap();

  let mut lowest = get_fuel_usage(crabs.iter(), min);
  for i in min+1..=max {
    let distance = get_fuel_usage(crabs.iter(), i);
    if distance < lowest {
      lowest = distance;
    }
  }

  println!("{}", lowest);
}

fn get_fuel_usage(crabs: Iter<i32>, target: i32) -> i32 {
  crabs.map(|crab| {
    (crab-target).abs()
  }).collect::<Vec<i32>>().iter().sum()
}
