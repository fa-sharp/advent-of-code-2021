use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
  println!("Opening input.txt!");

  /*
  Using an array, keep track of total number of 0s and 1s for each index/position
  e.g. [ position: (total_zeros, total_ones) ]
  e.g. [ 0: (32, 21), 1: (23, 42), etc... ]
  */
  let mut bits_array: [(u32, u32); 12] = [(0,0); 12];

  let lines = read_lines("input.txt").expect("Couldn't read file!");

  for line_result in lines {
    let bits = line_result.unwrap_or_else(|err| format!("Error reading line: {:#?}", err));
        
    for (position, bit) in bits.chars().enumerate() {

      // update the bits array. notice use of mutable references
      let (ref mut zeros, ref mut ones) = bits_array[position];
      match bit {
        '0' => *zeros += 1,
        '1' => *ones += 1,

        _ => eprintln!("Unrecognized bit '{}'!", bit)
      }
    }
  }

  println!();
  println!("RESULTS");
  println!("-----------------------");
  for (position, (zeros, ones)) in bits_array.iter().enumerate() {
    println!("position: {}, zeros: {}, ones: {}", position, zeros, ones);
  }
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
