use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
  println!("Opening input.txt!");
  
  if let Ok(lines) = read_lines("input.txt") {

    println!("Found input.txt! Parsing directions...");

    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in lines {

      if let Some((direction, value)) = instruction.expect("Failed to read instruction!").split_once(" ") {
        
        let value_as_int = value.parse::<i32>().expect(&format!("Failed to parse '{}' as integer!", value));
        
        match direction {
          "forward" => {
            horizontal_pos += value_as_int;
            depth += value_as_int * aim;
          },
          "down" => aim += value_as_int,
          "up" => aim -= value_as_int,
          
          _ => println!("Unrecognized direction '{}'!", direction)
        }
      }
    }

    println!();
    println!("RESULTS");
    println!("-----------------------");
    println!("Final horizontal position: {}", horizontal_pos);
    println!("Final depth: {}", depth);
    println!("Final (position * depth): {}", horizontal_pos * depth);
  }
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}