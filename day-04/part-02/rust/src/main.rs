use std::fs;

fn main() {
  let data = fs::read_to_string("../input.txt").expect("Something went wrong reading the file");

  let (calls, boards) = process_input(data);

  let mut board_masks : Vec<[bool; 25]> = Vec::new();
  for _ in 0..boards.len() {
    board_masks.push([false; 25])
  }

  let mut calls_iter = calls.into_iter();

  let mut filtered_boards : Vec<([i32; 25], [bool; 25])> = boards.into_iter().zip(board_masks).collect();

  loop {
    let mut call = calls_iter.next().unwrap();
    for i in 0..filtered_boards.len() {
      for j in 0..25 {
        if filtered_boards[i].0[j] == call {
          filtered_boards[i].1[j] = true;
        }
      }
    }
    filtered_boards = filtered_boards.into_iter().filter(|(_, mask)| {
      !is_winner(*mask)
    }).collect::<Vec<([i32;25], [bool;25])>>().to_vec();
    if filtered_boards.len() == 1 {
      let (board, mut mask) = filtered_boards[0];
      loop {
        call = calls_iter.next().unwrap();
        for j in 0..25{
          if board[j] == call {
            mask[j] = true;
          }
        }
        if is_winner(mask) {
          break
        }
      }
      process_winner(board, mask, call);
      break;
    }
  }
}

fn process_winner(board: [i32; 25], mask: [bool; 25], call: i32) {
  let mut total = 0;
  for (i, cell) in board.iter().enumerate() {
    if !mask[i] {
      total += cell
    }
  }

  println!("{}", total*call)

}

fn process_input(input: String) -> (Vec<i32>, Vec<[i32; 25]>) {
  let lines : Vec<String> = input.lines().map(String::from).collect();

  let calls = lines.first()
    .unwrap()
    .split(",").map(|call| {
      call.parse::<i32>().unwrap()
    }).collect();

  let board_lines : Vec<Vec<i32>> = lines[1..].into_iter().filter(|line| {
    *line != ""
  }).map(|line| {
    line.trim().split_whitespace().map(|n| {
      n.parse::<i32>().unwrap()
    }).collect()
  }).collect();

  return (calls, create_boards(board_lines));
}

fn create_boards(input: Vec<Vec<i32>>) -> Vec<[i32; 25]> {
  let mut boards : Vec<[i32; 25]> = Vec::new();
  let mut input_iter = input.into_iter();
  let mut temp_board : [i32; 25] = [0; 25];
  let mut index = 0;
  loop {
    match input_iter.next() {
      None => break,
      Some(row) => {
        for cell in row {
          temp_board[index] = cell;
          index += 1;
        }
      }
    }
    if index == 25 {
      boards.push(temp_board);
      temp_board = [0; 25];
      index = 0;
    }
  }
  boards
}

fn is_winner(mask: [bool; 25]) -> bool {
  let winning_masks : [u32; 10] = [
    0b1000010000100001000010000,
    0b0100001000010000100001000,
    0b0010000100001000010000100,
    0b0001000010000100001000010,
    0b0000100001000010000100001,
    0b1111100000000000000000000,
    0b0000011111000000000000000,
    0b0000000000111110000000000,
    0b0000000000000001111100000,
    0b0000000000000000000011111
  ];

  let binary = mask_to_binary(mask);
  for test in winning_masks {
    if binary & test == test {
      return true
    }
  }
  false
}

fn mask_to_binary(mask: [bool;25]) -> u32 {
  let mut binary = 0;
  for bit in mask {
    binary = binary << 1;
    if bit {binary +=1};
  }
  binary
}