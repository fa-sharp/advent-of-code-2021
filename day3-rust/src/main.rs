use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
  println!("Opening input.txt!");

  /*
  Using an array, keep track of total number of 0s and 1s for each index/position
  e.g. [ position: (total_zeros, total_ones) ]
  e.g. [ 0: (32, 21), 1: (23, 42), etc... ]
  */
  let mut zeros_and_ones: [(u32, u32); 12] = [(0,0); 12];

  let lines = read_lines("input.txt").expect("Couldn't read file!");

  for line_result in lines {
    let bits = line_result.expect(&format!("Error reading line!"));

    for (position, bit) in bits.chars().enumerate() {

      // update the bits array. notice use of mutable references
      let (ref mut zeros, ref mut ones) = zeros_and_ones[position];
      match bit {
        '0' => *zeros += 1,
        '1' => *ones += 1,

        _ => eprintln!("Unrecognized bit '{}'!", bit)
      }
    }
  }

  /* Calculate gamma (pick most common bit in each position) */
  let mut gamma_binary_str = String::from("");
  for (zeros, ones) in zeros_and_ones {
    if zeros > ones {
      gamma_binary_str.push('0');
    } else if ones > zeros {
      gamma_binary_str.push('1');
    } else {
      panic!("Equal number of zeros and ones found!")
    }
  }

  /* Calculate epsilon (pick least common bit in each position) */
  let mut epsilon_binary_str = String::from("");
  for (zeros, ones) in zeros_and_ones {
    if ones < zeros {
      epsilon_binary_str.push('1');
    } else if zeros < ones {
      epsilon_binary_str.push('0');
    } else {
      panic!("Equal number of zeros and ones found!")
    }
  }

  let gamma_value = usize::from_str_radix(&gamma_binary_str, 2).expect(&format!("Error parsing gamma value from '{}'", gamma_binary_str));
  let epsilon_value = usize::from_str_radix(&epsilon_binary_str, 2).expect(&format!("Error parsing epsilon value from '{}'", epsilon_binary_str));

  println!();
  println!("RESULTS");
  println!("-----------------------");
  for (position, (zeros, ones)) in zeros_and_ones.iter().enumerate() {
    println!("position: {}, zeros: {}, ones: {}", position, zeros, ones);
  }
  println!();
  println!("-----------------------");
  println!("Gamma: {} ({})", gamma_value, gamma_binary_str);
  println!("Epsilon: {} ({})", epsilon_value, epsilon_binary_str);
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
