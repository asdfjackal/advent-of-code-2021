use std::fs;
use std::cmp::Ordering;

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");
  let lines: Vec<Vec<i32>> = data.lines().map(|line| {
    line.chars().map(|c| {
      c.to_digit(10).unwrap() as i32
    }).collect()
  }).collect();


  let oxygen = get_code(&lines, true);
  let co2 = get_code(&lines, false);
  println!("{}", oxygen);
  println!("{}", co2);

  println!("{}", oxygen*co2);
}

fn binary_to_int(binary: &Vec<i32>) -> i32 {
  let mut result = 0;
  for bit in binary {
    result = (result << 1) + bit
  }
  result
}

fn get_code(input: &Vec<Vec<i32>>, filter_high: bool) -> i32 {
  let mut filtered_lines = input.to_vec();
  for i in 0..filtered_lines[0].len() {
    let count: i32 = filtered_lines.to_vec().into_iter().map(|line| {
      line[i]
    }).sum();
    let total = filtered_lines.len() as i32 / 2;

    let target = match count.cmp(&total) {
      Ordering::Greater => if filter_high {1} else {0},
      Ordering::Equal => if filter_high {1} else {0},
      Ordering::Less => if filter_high {0} else {1},
    };

    filtered_lines = filtered_lines.to_vec().into_iter().filter(|line| {
      line[i] == target
    }).collect();
    if filtered_lines.len() == 1 {
      break
    }
  }
  return binary_to_int(filtered_lines.first().unwrap())
}