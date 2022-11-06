use std::{
  collections::{HashMap, HashSet},
  fs::File,
  io::{self, BufRead},
  path::Path,
};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let points_hashmap = get_points_from_input().expect("Couldn't read input!");

  let mut sum_risk_levels: u32 = 0;
  let mut points_traversed: HashSet<Point> = HashSet::new();
  let mut basin_sizes: Vec<u32> = vec![];

  // For each point, check if it's lower than surrounding points.
  for (point, point_height) in &points_hashmap {
    let top_height = points_hashmap
      .get(&Point {
        x: point.x,
        y: point.y - 1,
      })
      .unwrap_or(&10);
    let bottom_height = points_hashmap
      .get(&Point {
        x: point.x,
        y: point.y + 1,
      })
      .unwrap_or(&10);
    let left_height = points_hashmap
      .get(&Point {
        x: point.x - 1,
        y: point.y,
      })
      .unwrap_or(&10);
    let right_height = points_hashmap
      .get(&Point {
        x: point.x + 1,
        y: point.y,
      })
      .unwrap_or(&10);

    let is_low_point = point_height < top_height
      && point_height < bottom_height
      && point_height < left_height
      && point_height < right_height;
    if is_low_point {
      let risk_level = point_height + 1;
      sum_risk_levels += u32::from(risk_level);

      let basin_size = 1 + get_basin_size(point, &points_hashmap, &mut points_traversed);
      basin_sizes.push(basin_size);
    }
  }

  println!("Sum of risk levels: {}", sum_risk_levels);
  basin_sizes.sort();
  let top_basin_sizes = basin_sizes.split_off(basin_sizes.len() - 3);
  println!("Top 3 basins: {:#?}", top_basin_sizes);
}

/** Calculate basin size through path traversal / recursion */
fn get_basin_size(
  point: &Point,
  points_hashmap: &HashMap<Point, u8>,
  points_traversed: &mut HashSet<Point>,
) -> u32 {
  points_traversed.insert(*point);

  let top_point = Point {
    x: point.x,
    y: point.y - 1,
  };
  let top_basin_size = {
    match points_hashmap.get(&top_point) {
      Some(top_height) => {
        if *top_height == 9 || points_traversed.contains(&top_point) {
          0
        } else {
          1 + get_basin_size(&top_point, points_hashmap, points_traversed)
        }
      }
      None => 0,
    }
  };
  let bottom_point = Point {
    x: point.x,
    y: point.y + 1,
  };
  let bottom_basin_size = {
    match points_hashmap.get(&bottom_point) {
      Some(bottom_height) => {
        if *bottom_height == 9 || points_traversed.contains(&bottom_point) {
          0
        } else {
          1 + get_basin_size(&bottom_point, points_hashmap, points_traversed)
        }
      }
      None => 0,
    }
  };
  let left_point = Point {
    x: point.x - 1,
    y: point.y,
  };
  let left_basin_size = {
    match points_hashmap.get(&left_point) {
      Some(left_height) => {
        if *left_height == 9 || points_traversed.contains(&left_point) {
          0
        } else {
          1 + get_basin_size(&left_point, points_hashmap, points_traversed)
        }
      }
      None => 0,
    }
  };
  let right_point = Point {
    x: point.x + 1,
    y: point.y,
  };
  let right_basin_size = {
    match points_hashmap.get(&right_point) {
      Some(right_height) => {
        if *right_height == 9 || points_traversed.contains(&right_point) {
          0
        } else {
          1 + get_basin_size(&right_point, points_hashmap, points_traversed)
        }
      }
      None => 0,
    }
  };

  return top_basin_size + bottom_basin_size + right_basin_size + left_basin_size;
}

/** Generate a hashmap of points and their heights. Point is the key, and height is the value  */
fn get_points_from_input() -> io::Result<HashMap<Point, u8>> {
  let mut points_hashmap: HashMap<Point, u8> = HashMap::new();
  let raw_lines_from_file = read_raw_lines_from_file("input.txt")?;
  for (y, raw_line_result) in raw_lines_from_file.enumerate() {
    for (x, height_char) in raw_line_result.unwrap_or(String::new()).chars().enumerate() {
      let point = Point {
        x: x.try_into().unwrap(),
        y: y.try_into().unwrap(),
      };
      let height =
        u8::from_str_radix(&height_char.to_string(), 10).expect("Couldn't parse height!");
      points_hashmap.insert(point, height);
    }
  }

  Ok(points_hashmap)
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_raw_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
