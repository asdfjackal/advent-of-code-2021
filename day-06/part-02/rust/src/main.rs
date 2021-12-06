use std::fs;

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");
  let starting_fish: Vec<i32> = data.split(",").map(|fish| {
      fish.parse().unwrap()
  }).collect();

  let mut fish_state : [u128; 9] = [0; 9];

  for fish in starting_fish {
    fish_state[fish as usize] += 1;
  }

  for _ in 1..=256 {
    fish_state = process_day(fish_state);
  }

  println!("{}", fish_state.iter().sum::<u128>())
}

fn process_day(state: [u128; 9]) -> [u128; 9] {
  let mut new_state = [0; 9];
    for i in 1..9 {
      new_state[i-1] = state[i];
    }
    new_state[6] += state[0];
    new_state[8] += state[0];
  new_state
}
