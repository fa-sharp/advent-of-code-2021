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
  for line_result in read_raw_lines_from_file("input.txt").expect("Couldn't read file!") {
    let line = line_result.expect("Error while reading file!");
    if let Some(incorrect_chunk) = find_incorrect_chunk(&line) {
      match incorrect_chunk {
        Chunk::CloseAngle => syntax_error_score += 25137,
        Chunk::CloseBrace => syntax_error_score += 1197,
        Chunk::CloseParen => syntax_error_score += 3,
        Chunk::CloseSquare => syntax_error_score += 57,
        _ => panic!("Unexpected incorrect opening chunk '{:#?}'", incorrect_chunk),
      }
    }
  }
  println!("Syntax error score: {}", syntax_error_score);
}

/** Returns the first incorrect chunk, if it exists.
 * Uses a stack (push/pop) to check proper open and closing braces */
fn find_incorrect_chunk(line: &str) -> Option<Chunk> {
  let chunks = convert_line_to_chunks(line);
  let mut stack: Vec<Chunk> = vec![];

  for chunk in chunks {
    match chunk {
      Chunk::OpenAngle | Chunk::OpenBrace | Chunk::OpenParen | Chunk::OpenSquare => stack.push(chunk),
      Chunk::CloseAngle => {
        if let Some(last_chunk) = stack.pop() {
          if last_chunk != Chunk::OpenAngle {
            return Some(chunk);
          }
        }
      }
      Chunk::CloseBrace => {
        if let Some(last_chunk) = stack.pop() {
          if last_chunk != Chunk::OpenBrace {
            return Some(chunk);
          }
        }
      }
      Chunk::CloseParen => {
        if let Some(last_chunk) = stack.pop() {
          if last_chunk != Chunk::OpenParen {
            return Some(chunk);
          }
        }
      }
      Chunk::CloseSquare => {
        if let Some(last_chunk) = stack.pop() {
          if last_chunk != Chunk::OpenSquare {
            return Some(chunk);
          }
        }
      }
    }
  }
  None // if we get here, chunks are valid!
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
