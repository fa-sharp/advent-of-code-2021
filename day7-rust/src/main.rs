use std::{
  fs::File,
  io::{self, BufRead},
  path::Path,
};

use statrs::statistics::{Data, OrderStatistics};

fn main() {
  let ref crab_positions =
    get_crab_positions_from_input().expect("Error reading positions from input!");

  let median_position = Data::new(crab_positions.clone()).median();
  let mean_position = (crab_positions.iter().sum::<f64>() / crab_positions.len() as f64) as u64;

  
  let total_fuel_needed_1 = crab_positions.iter().fold(0_f64, |fuel, position| {
    fuel + (median_position - *position).abs()
  });
  let total_fuel_needed_2 = crab_positions.iter().fold(0_u64, |fuel, position| {
    fuel + calc_fuel(mean_position.abs_diff(*position as u64))
  });

  println!("\nRESULTS");
  println!("----------------\n");
  println!("Total fuel needed (part 1): {}", total_fuel_needed_1);
  println!("Total fuel needed (part 2): {}\n", total_fuel_needed_2);
}

fn calc_fuel(steps: u64) -> u64 {
  let mut fuel = 0;
  for step in 1..=steps {
    fuel += step;
  }
  fuel
}

fn get_crab_positions_from_input() -> io::Result<Vec<f64>> {
  let mut crab_positions: Vec<f64> = Vec::new();
  let raw_input = read_raw_lines_from_file("input.txt")?
    .next()
    .expect("Couldn't read input numbers!")?;
  let raw_positions = raw_input.split(",");
  for raw_position in raw_positions {
    let position = raw_position
      .parse::<f64>()
      .expect(&format!("Unable to parse position '{}'", raw_position));
    crab_positions.push(position);
  }

  Ok(crab_positions)
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_raw_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
