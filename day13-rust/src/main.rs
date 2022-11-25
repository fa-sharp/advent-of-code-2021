use std::{
  collections::HashSet,
  fs::File,
  io::{self, BufRead},
  path::Path,
};

enum Axis {
  X,
  Y,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
  x: u32,
  y: u32,
}

fn main() {
  let mut points_hashset = get_points_from_input().expect("Couldn't read points!");
  let flip_instructions = get_flip_instructions_from_input().expect("Couldn't read instructions!");

  for (axis, value) in flip_instructions {
    match axis {
      Axis::X => flip_along_x(value, &mut points_hashset),
      Axis::Y => flip_along_y(value, &mut points_hashset),
    }
  }

  let drawing = draw_the_code(&points_hashset);

  println!("{} points left!", points_hashset.len());
  println!("{}", drawing);
  println!("Done!");
}

fn draw_the_code(points: &HashSet<Point>) -> String {
  let mut drawing = String::new();
  for y in 0..10_u32 {
    for x in 0..40_u32 {
      if points.contains(&Point { x, y }) {
        drawing += "O"
      } else {
        drawing += " ";
      }
    }
    drawing += "\n";
  }

  drawing
}

fn flip_along_x(x_line: u32, points: &mut HashSet<Point>) {
  let mut points_to_flip: Vec<(Point, Point)> = vec![];
  for point in points.to_owned() {
    if point.x > x_line {
      let new_point = Point {
        x: x_line - (point.x - x_line),
        y: point.y,
      };
      points_to_flip.push((point, new_point));
    }
  }
  for (old_point, new_point) in points_to_flip {
    points.remove(&old_point);
    points.insert(new_point);
  }
}

fn flip_along_y(y_line: u32, points: &mut HashSet<Point>) {
  let mut points_to_flip: Vec<(Point, Point)> = vec![];
  for point in points.to_owned() {
    if point.y > y_line {
      let new_point = Point {
        x: point.x,
        y: y_line - (point.y - y_line),
      };
      points_to_flip.push((point, new_point));
    }
  }
  for (old_point, new_point) in points_to_flip {
    points.remove(&old_point);
    points.insert(new_point);
  }
}

/** Generate a hashset of points */
fn get_points_from_input() -> io::Result<HashSet<Point>> {
  let mut points_hashset: HashSet<Point> = HashSet::new();
  let raw_lines_from_file = read_raw_lines_from_file("input.txt")?;
  for raw_line_result in raw_lines_from_file {
    if let Some((raw_x, raw_y)) = raw_line_result.unwrap_or_default().split_once(",") {
      let point = Point {
        x: raw_x.parse().expect("Couldn't parse x"),
        y: raw_y.parse().expect("Couldn't parse y"),
      };
      points_hashset.insert(point);
    }
  }

  Ok(points_hashset)
}

/** Read the flip instructions */
fn get_flip_instructions_from_input() -> io::Result<Vec<(Axis, u32)>> {
  let mut flip_instructions: Vec<(Axis, u32)> = vec![];
  let raw_lines_from_file = read_raw_lines_from_file("input.txt")?;
  for raw_line_result in raw_lines_from_file {
    if let Some((raw_axis, raw_value)) = raw_line_result
      .unwrap_or_default()
      .strip_prefix("fold along ")
      .unwrap_or_default()
      .split_once("=")
    {
      let axis = match raw_axis {
        "x" => Axis::X,
        "y" => Axis::Y,
        _ => panic!("Unrecognized axis!"),
      };
      flip_instructions.push((axis, raw_value.parse().expect("Couldn't parse flip axis value!")))
    }
  }

  Ok(flip_instructions)
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_raw_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
