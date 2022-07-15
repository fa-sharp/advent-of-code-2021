use std::{
  fs::File,
  io::{self, BufRead},
  path::Path
};

fn main() {
  println!("Hello, world!");
}

fn get_lines_from_input() -> io::Result<Vec<((u32, u32), (u32, u32))>> {
  let mut lines_vector: Vec<((u32, u32), (u32, u32))> = Vec::new();
  let raw_lines_from_file = read_raw_lines_from_file("input.txt")?;

  /* Lines in input.txt are in the format "x,y -> x,y" e.g. "510,771 -> 510,322"
   * We'll want to split by " -> " and then by "," */
  for (line_num, raw_line_result) in raw_lines_from_file.enumerate() {
    let raw_line = raw_line_result.expect(&format!("Failed to read line #{}", line_num));

    if let Some((raw_point1, raw_point2)) = raw_line.split_once(" -> ") {
      let point1 = raw_point1.parse::<i32>();
      let point2 = raw_point2.parse::<i32>();
    }
  }

  Ok(lines_vector)
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_raw_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
