use std::{
  fs::File,
  io::{self, BufRead},
  path::Path,
};

#[derive(PartialEq, Debug)]
enum Chunk {
  OpenParen,
  CloseParen,
  OpenSquare,
  CloseSquare,
  OpenBrace,
  CloseBrace,
  OpenAngle,
  CloseAngle,
}

fn main() {
  let mut syntax_error_score = 0;
  let mut autocomplete_scores: Vec<u64> = vec![];
  
  for line_result in read_raw_lines_from_file("input.txt").expect("Couldn't read file!") {
    let line = line_result.expect("Error while reading file!");
    let (incorrect_chunk, autocomplete_chunks) = process_chunks_in_line(&line);
    match incorrect_chunk {
      None => (),
      Some(Chunk::CloseAngle) => syntax_error_score += 25137,
      Some(Chunk::CloseBrace) => syntax_error_score += 1197,
      Some(Chunk::CloseParen) => syntax_error_score += 3,
      Some(Chunk::CloseSquare) => syntax_error_score += 57,
      _ => panic!("Unexpected incorrect opening chunk '{:#?}'", incorrect_chunk),
    }
    match autocomplete_chunks {
      None => (),
      Some(chunks) => {
        let mut score = 0_u64;
        for chunk in chunks {
          match chunk {
            Chunk::CloseAngle => score = score * 5 + 4,
            Chunk::CloseParen => score = score * 5 + 1,
            Chunk::CloseBrace => score = score * 5 + 3,
            Chunk::CloseSquare => score = score * 5 + 2,
            _ => panic!("Unexpected autocompleted opening chunk '{:#?}'", chunk),
          }
        }
        autocomplete_scores.push(score);
      }
    }
  }

  println!("Syntax error score: {}", syntax_error_score);

  autocomplete_scores.sort();
  let middle_score = autocomplete_scores.get((autocomplete_scores.len() - 1) / 2);
  println!("Middle auto-complete score: {}", middle_score.expect("No middle score found!"))
}

/** Returns a tuple of Options:
 * first incorrect chunk, if it exists.
 * if all chunks are correct, a vector of auto-completed chunks needed to complete this line.
 
 Uses a stack structure (push/pop) to check proper open and closing braces */
fn process_chunks_in_line(line: &str) -> (Option<Chunk>, Option<Vec<Chunk>>) {
  let chunks = convert_line_to_chunks(line);
  let mut stack: Vec<Chunk> = vec![];

  for chunk in chunks {
    match chunk {
      Chunk::OpenAngle | Chunk::OpenBrace | Chunk::OpenParen | Chunk::OpenSquare => stack.push(chunk),
      Chunk::CloseAngle => {
        if let Some(last_chunk) = stack.pop() {
          if last_chunk != Chunk::OpenAngle {
            return (Some(chunk), None);
          }
        }
      }
      Chunk::CloseBrace => {
        if let Some(last_chunk) = stack.pop() {
          if last_chunk != Chunk::OpenBrace {
            return (Some(chunk), None);
          }
        }
      }
      Chunk::CloseParen => {
        if let Some(last_chunk) = stack.pop() {
          if last_chunk != Chunk::OpenParen {
            return (Some(chunk), None);
          }
        }
      }
      Chunk::CloseSquare => {
        if let Some(last_chunk) = stack.pop() {
          if last_chunk != Chunk::OpenSquare {
            return (Some(chunk), None);
          }
        }
      }
    }
  }
  
  // If we get here, all chunks are valid! Auto-complete the line by examining the remaining stack
  // Stack should only contain opening chunks
  let mut autocomplete_chunks: Vec<Chunk> = vec![];
  while stack.len() > 0 {
    if let Some(remaining_chunk) = stack.pop() {
      match remaining_chunk {
        Chunk::OpenAngle => autocomplete_chunks.push(Chunk::CloseAngle),
        Chunk::OpenBrace => autocomplete_chunks.push(Chunk::CloseBrace),
        Chunk::OpenParen => autocomplete_chunks.push(Chunk::CloseParen),
        Chunk::OpenSquare => autocomplete_chunks.push(Chunk::CloseSquare),
        _ => panic!("Unexpected closing chunk found in stack: '{:#?}'", remaining_chunk)
      }
    }
  }

  (None, Some(autocomplete_chunks))
}

fn convert_line_to_chunks(line: &str) -> Vec<Chunk> {
  let mut chunks: Vec<Chunk> = vec![];
  for char in line.chars() {
    chunks.push(match char {
      '{' => Chunk::OpenBrace,
      '}' => Chunk::CloseBrace,
      '(' => Chunk::OpenParen,
      ')' => Chunk::CloseParen,
      '<' => Chunk::OpenAngle,
      '>' => Chunk::CloseAngle,
      '[' => Chunk::OpenSquare,
      ']' => Chunk::CloseSquare,
      _ => panic!("Unexpected character '{}'!", char),
    })
  }
  chunks
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_raw_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
