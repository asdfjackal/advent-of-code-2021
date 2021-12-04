use std::fs;

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");
  let lines: Vec<Vec<i32>> = data.lines().map(|line| {
    line.chars().map(|c| {
      c.to_digit(10).unwrap() as i32
    }).collect()
  }).collect();

  let mut gamma = 0;
  let mut mask = 0;

  let num_lines = lines.len() as i32;

  let mut all_bits: Vec<i32> = vec![0; lines[0].len()];

  for code in lines {
    for (i, bit) in code.iter().enumerate() {
      all_bits[i] += bit;
    }
  }

  for count in all_bits {
    gamma = gamma << 1;
    mask = mask << 1;
    if count > (num_lines / 2 ) {
      gamma += 1;
    }
    mask += 1;
  }

  let epsilon = gamma^mask;

  println!("{}", gamma*epsilon);
}
