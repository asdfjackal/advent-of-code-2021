use std::fs;
use std::slice::Iter;

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");
  let crabs: Vec<i32> = data.split(",").map(|crab| {
    crab.parse().unwrap()
  }).collect();
  let max = *crabs.iter().max().unwrap();
  let min = *crabs.iter().min().unwrap();

  let mut lowest = get_total_fuel_usage(crabs.iter(), min);
  for i in (min+1)..=max {
    let distance = get_total_fuel_usage(crabs.iter(), i);
    if distance < lowest {
      lowest = distance;
    }
  }

  println!("{}", lowest);
}

fn get_total_fuel_usage(crabs: Iter<i32>, target: i32) -> i32 {
  crabs.map(|crab| {
    get_individual_fuel_usage((crab-target).abs())
  }).sum()
}

fn get_individual_fuel_usage(distance: i32) -> i32 {
  match distance % 2 {
    0 => (distance+1) * (distance/2),
    1 => ((distance+1) * (distance/2)) + ((distance/2)+1),
    _ => unreachable!()
  }
}
