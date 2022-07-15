use std::{
  collections::{HashMap},
  fs::File,
  io::{self, BufRead},
  num::ParseIntError,
  path::Path, cmp,
};

#[derive(Eq, Hash, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  /* A list of all lines read from the input, in the form (Point, Point) */
  let lines_vector = get_lines_from_input().expect("Error while parsing lines in input file");

  /* A hashmap to keep track of how many times each point is traversed by a line */
  let mut traversed_points_map: HashMap<Point, u32> = HashMap::new();

  /* Utility function to either add or update a traversed point value */
  let mut upsert_traversed_point = |point: Point| {
    if traversed_points_map.contains_key(&point) {
      let traversed_value = traversed_points_map.get_mut(&point).unwrap();
      *traversed_value += 1;
    }
    else {
      traversed_points_map.insert(point, 1);
    }
  };

  for (point1, point2) in &lines_vector {

    // vertical line
    if point1.x == point2.x {
      for y in cmp::min(point1.y, point2.y)..=cmp::max(point1.y, point2.y) {
        let traversed_point = Point { x: point1.x, y };
        upsert_traversed_point(traversed_point);
      }
    }
    // horizontal line 
    else if point1.y == point2.y {
      for x in cmp::min(point1.x, point2.x)..=cmp::max(point1.x, point2.x) {
        let traversed_point = Point { x, y: point1.y };
        upsert_traversed_point(traversed_point);
      }
    }
  }

  /* Retain points that have been traversed at least 2 times */
  traversed_points_map.retain(|_, traversed_value| *traversed_value >= 2);

  println!("RESULTS");
  println!("-------------------");
  println!("Number of points that have been traversed at least 2 times: {}", traversed_points_map.len());

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
