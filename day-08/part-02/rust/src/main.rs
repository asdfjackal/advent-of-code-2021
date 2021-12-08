use std::fs;

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");
  let mut total = 0;
  for line in data.lines() {
    let input : Vec<&str> = line.split("|").nth(0).unwrap().trim().split(" ").collect();

    let output : Vec<&str> = line.split("|").last().unwrap().trim().split(" ").collect();
    let mappings = get_mappings_from_inputs(input);
    let result = get_result(output, mappings);
    total += result;

  }

  println!("{}", total);
}

fn get_mappings_from_inputs(inputs: Vec<&str>) -> Vec<(&str, i32)> {
  let mut results = Vec::new();
  let mut len_5 = Vec::new();
  let mut len_6 = Vec::new();
  let mut one = "";
  let mut four = "";
  for input in inputs {
    match input.len() {
      2 => {
        results.push((input, 1));
        one = input;
      },
      3 => results.push((input, 7)),
      4 => {
        results.push((input, 4));
        four = input;
      },
      7 => results.push((input, 8)),
      5 => len_5.push(input),
      6 => len_6.push(input),
      _ => unreachable!()
    };
  };
  len_6 = len_6.iter().filter(|input| {
    if !is_substring(one, **input) {
      results.push((**input, 6));
      return false
    }
    true
  }).map(|input| *input).collect();
  len_6 = len_6.iter().filter(|input| {
    if !is_substring(four, **input) {
      results.push((**input, 0));
      return false
    }
    true
  }).map(|input| *input).collect();
  let nine = len_6.iter().next().unwrap();
  results.push((nine, 9));

  len_5 = len_5.iter().filter(|input| {
    if is_substring(one, **input) {
      results.push((**input, 3));
      return false
    }
    true
  }).map(|input| *input).collect();
  len_5 = len_5.iter().filter(|input| {
    if is_substring(**input, nine) {
      results.push((**input, 5));
      return false
    }
    true
  }).map(|input| *input).collect();
  results.push((*len_5.iter().next().unwrap(), 2));

  results
}

fn get_result(outputs: Vec<&str>, mappings: Vec<(&str, i32)>) -> i32 {
  let mut result = 0;
  for output in outputs {
    result *= 10;
    for (key, value) in mappings.iter() {
      let sorted_key = sort_string(key);
      let sorted_output = sort_string(output);
      if sorted_key == sorted_output {
        result += value;
        break;
      }
    }
  }

  result
}

fn is_substring(short: &str, long: &str) -> bool {
  for char in short.chars() {
    if !long.contains(char) {
      return false
    }
  }
  true
}

fn sort_string(input: &str) -> String {
  let mut chars : Vec<char> = input.chars().collect();
  chars.sort();
  chars.into_iter().collect()
}