use std::{
  collections::VecDeque,
  fs::File,
  io::{self, BufRead},
  path::Path,
};

fn main() {
  /* A queue (sort of) that keeps track of how many fish are on each day within the lifecycle
    E.g.:
    Day 0 - 2 fish   ↑
    Day 1 - 3 fish   ↑
    Day 2 - 4 fish   ↑
    .                ↑
    .                ↑
    Day 8 - 3 fish   ↑
    */
  let mut current_lifecycle = get_current_cycle_from_input().expect("Error reading input file!");

  for _day in 0..256 {
    simulate_one_day(&mut current_lifecycle);
  }

  let total_fish: u64 = current_lifecycle.iter().sum();

  println!("RESULTS");
  println!("-------------------");
  println!("After 256 days, there are {} fish", total_fish);
}

/** Simulate a single day and update fish cycles */
fn simulate_one_day(current_lifecycle: &mut VecDeque<u64>) {

  // Rotate the queue:
  // The # of fish that were on Day 3 are now on Day 2, etc.
  // The # of fish that were on Day 0 are now on Day 8, simulating the new fish babies
  current_lifecycle.rotate_left(1);

  // 'Reset' the mommy fish that gave birth, by adding the number of babies to Day 6
  let num_new_mommies = current_lifecycle[8];
  current_lifecycle[6] += num_new_mommies;
}

/** Read current state of the fish lifecycle from the input file */
fn get_current_cycle_from_input() -> io::Result<VecDeque<u64>> {

  let mut lifecycle: VecDeque<u64> = VecDeque::new();
  lifecycle.resize(9, 0); // fill with zeros, length of 9 to represent Days 0 - 8

  let raw_lines_from_file = read_raw_lines_from_file("input.txt")?;

  for raw_line_result in raw_lines_from_file {
    let raw_line = raw_line_result?;
    let raw_cycle_values = raw_line.split(",");

    for raw_day in raw_cycle_values {
      let day =
        usize::from_str_radix(raw_day, 10).expect(&format!("Failed to parse value '{}'", raw_day));
      
      lifecycle[day] += 1;
    }
  }

  Ok(lifecycle)
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_raw_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
