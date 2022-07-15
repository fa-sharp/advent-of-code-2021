use std::{
  collections::HashMap,
  fs::File,
  io::{self, BufRead},
  num::ParseIntError,
  path::Path,
};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  /* A list of all lines read from the input, in the form (Point, Point) */
  let lines_vector = get_lines_from_input().expect("Error while parsing lines in input file");

  /* A hashmap to keep track of how many times each point is traversed by a line */
  let mut traversed_points_map: HashMap<Point, u32> = HashMap::new();

  // Process each line and update the traversed points hashmap
  for (point1, point2) in &lines_vector {
    let ref mut current_point = point1.clone();
    loop {
      traversed_points_map.insert(
        *current_point,
        *traversed_points_map
          .get(current_point)
          .unwrap_or_else(|| &0)
          + 1,
      );

      if point2.eq(current_point) {
        break;
      }

      if current_point.x < point2.x {
        current_point.x += 1;
      } else if current_point.x > point2.x {
        current_point.x -= 1;
      }

      if current_point.y < point2.y {
        current_point.y += 1;
      } else if current_point.y > point2.y {
        current_point.y -= 1;
      }
    }
  }

  // Retain points that have been traversed at least 2 times
  traversed_points_map.retain(|_, traversed_value| *traversed_value >= 2);

  println!("RESULTS");
  println!("-------------------");
  println!(
    "Number of points that have been traversed at least 2 times: {}",
    traversed_points_map.len()
  );
}

fn get_lines_from_input() -> io::Result<Vec<(Point, Point)>> {
  let mut lines_vector: Vec<(Point, Point)> = Vec::new();
  let raw_lines_from_file = read_raw_lines_from_file("input.txt")?;

  /* Loop over lines in input file, parse the line data */
  for (line_num, raw_line_result) in raw_lines_from_file.enumerate() {
    let raw_line = raw_line_result.expect(&format!("Failed to read line #{}", line_num + 1));

    let parsed_line = parse_line_from_raw_line(&raw_line)
      .expect(&format!("Error parsing point at line #{}", line_num + 1));
    lines_vector.push(parsed_line);
  }

  Ok(lines_vector)
}

/* Get points from raw value "x,y -> x,y" e.g. "510,771 -> 510,322" */
fn parse_line_from_raw_line(raw_line: &str) -> Result<(Point, Point), ParseIntError> {
  let raw_points = raw_line
    .split_once(" -> ")
    .expect("Didn't find ' -> ' in line input!");
  let point1 = parse_point(raw_points.0)?;
  let point2 = parse_point(raw_points.1)?;

  Ok((point1, point2))
}

/* Get point from raw value "x,y"  */
fn parse_point(raw_point: &str) -> Result<Point, ParseIntError> {
  let (raw_x, raw_y) = raw_point
    .split_once(",")
    .expect("Didn't find comma in coordinates input!");
  let x = i32::from_str_radix(raw_x, 10)?;
  let y = i32::from_str_radix(raw_y, 10)?;

  Ok(Point { x, y })
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_raw_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
