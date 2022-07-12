use std::{fs::File, io::{self, BufRead}, path::Path};

fn main() {
  println!("Opening input.txt!");

  /* Read all numbers from file as a vector */
  let mut numbers_vector: Vec<String> = Vec::new();
  let file_lines = read_lines("input.txt").expect("Couldn't read file!");
  for line_result in file_lines {
    let bits = line_result.expect(&format!("Error reading number!"));
    numbers_vector.push(bits);
  }

  /*
  Using an array, keep track of total number of 0s and 1s for each index/position
  e.g. [ position: (total_zeros, total_ones) ]
  e.g. [ 0: (32, 21), 1: (23, 42), etc... ]
  */
  let mut zeros_and_ones: [(u32, u32); 12] = [(0,0); 12];
  
  for bits in &numbers_vector {
    for (position, bit) in bits.chars().enumerate() {

      // update zeros and ones array. notice use of mutable references
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

  let gamma_value  = usize::from_str_radix(&gamma_binary_str, 2).expect(&format!("Error parsing gamma value from '{}'", gamma_binary_str));
  let epsilon_value = usize::from_str_radix(&epsilon_binary_str, 2).expect(&format!("Error parsing epsilon value from '{}'", epsilon_binary_str));



  /*  ------------PART 2--------------- */

  let mut oxygen_rating_finder = numbers_vector.clone();
  let mut co2_rating_finder = numbers_vector.clone();

  /* Calculate oxygen generator rating (consider each position individually, only keep numbers with most common bit in that position) */
  for position in 0..=11 {
    
    /* Calculate most common bit in this position) */
    let ref mut zeros = 0;
    let ref mut ones = 0;
    for bits in &oxygen_rating_finder {
      match bits.chars().nth(position) {
        Some('0') => *zeros += 1,
        Some('1') => *ones += 1,
        _ => eprintln!("Unrecognized bit!")
      }
    }

    let most_common_bit;
    if zeros > ones { most_common_bit = '0'; }
    else { most_common_bit = '1'; }

    /* Only keep numbers that have the most common bit in this position */
    oxygen_rating_finder.retain(|bits| bits.chars().nth(position) == Some(most_common_bit));

    if oxygen_rating_finder.len() == 1 { break; }
  }  

  /* Calculate CO2 scrubber rating (consider each position individually, only keep numbers with LEAST common bit in that position) */
  for position in 0..=11 {
    
    /* Calculate least common bit in this position) */
    let ref mut zeros = 0;
    let ref mut ones = 0;
    for bits in &co2_rating_finder {
      match bits.chars().nth(position) {
        Some('0') => *zeros += 1,
        Some('1') => *ones += 1,
        _ => eprintln!("Unrecognized bit!")
      }
    }

    let least_common_bit;
    if ones < zeros { least_common_bit = '1'; }
    else { least_common_bit = '0'; }

    /* Only keep numbers that have the least common bit in this position */
    co2_rating_finder.retain(|bits| bits.chars().nth(position) == Some(least_common_bit));

    if co2_rating_finder.len() == 1 { break; }
  }  

  let oxygen_rating_binary = oxygen_rating_finder.get(0).expect("No oxygen rating found!");
  let oxygen_rating_value = usize::from_str_radix(oxygen_rating_binary, 2).expect(&format!("Error parsing oxygen rating value from '{}'", oxygen_rating_binary));

  let co2_rating_binary = co2_rating_finder.get(0).expect("No CO2 rating found!");
  let co2_rating_value = usize::from_str_radix(co2_rating_binary, 2).expect(&format!("Error parsing CO2 rating value from '{}'", co2_rating_binary));



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
  println!("Power consumption (Gamma * Epsilon): {}", gamma_value * epsilon_value);
  println!("-----------------------");
  println!("Oxygen rating: {} ({})", oxygen_rating_value, oxygen_rating_binary);
  println!("CO2 rating: {} ({})", co2_rating_value, co2_rating_binary);
  println!("Life support rating (Oxygen * CO2): {}", oxygen_rating_value * co2_rating_value);
}


/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
