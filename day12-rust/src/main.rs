use std::{
  collections::HashMap,
  fs::File,
  io::{self, BufRead},
  path::Path,
};

fn main() {
  let cave_paths = get_cave_paths_from_input().expect("Error reading input");
  let all_paths = find_all_paths(&cave_paths);
  
  println!("Found {} paths!", all_paths.len());
}

fn find_all_paths(paths_hashmap: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
  let start_cave = "start";
  let mut all_paths: Vec<Vec<String>> = vec![];
  let possible_next_caves = paths_hashmap.get(start_cave).expect("start cave should exist");
  for next_cave in possible_next_caves {
    let current_path = vec![start_cave.to_string()];
    explore_paths(current_path, &mut all_paths, next_cave.to_string(), paths_hashmap);
  }

  all_paths
}

fn explore_paths(
  current_path: Vec<String>,
  all_paths: &mut Vec<Vec<String>>,
  next_cave: String,
  cave_paths_hashmap: &HashMap<String, Vec<String>>,
) {
  if next_cave == "end" {
    let mut final_path = current_path.clone();
    final_path.push(next_cave.to_string());
    all_paths.push(final_path);
    return;
  } else if next_cave == "start" {
    return;
  } else if next_cave.to_lowercase() == next_cave && current_path.contains(&next_cave) {
    return;
  } else {
    let possible_future_caves = cave_paths_hashmap.get(&next_cave).expect("cave should exist");
    for future_cave in possible_future_caves {
      let mut new_path = current_path.clone();
      new_path.push(next_cave.to_string());
      explore_paths(new_path, all_paths, future_cave.to_string(), cave_paths_hashmap);
    }
  }
}

fn get_cave_paths_from_input() -> io::Result<HashMap<String, Vec<String>>> {
  let mut paths_hashmap: HashMap<String, Vec<String>> = HashMap::new();

  for raw_line_result in read_raw_lines_from_file("input.txt")? {
    if let Some((cave_a, cave_b)) = raw_line_result.unwrap_or_default().split_once("-") {
      paths_hashmap.entry(cave_a.to_string()).or_insert(vec![]);
      paths_hashmap.entry(cave_b.to_string()).or_insert(vec![]);

      paths_hashmap
        .get_mut(cave_a)
        .expect("cave_a should be present")
        .push(cave_b.to_string());
      paths_hashmap
        .get_mut(cave_b)
        .expect("cave_b should be present")
        .push(cave_a.to_string());
    }
  }

  Ok(paths_hashmap)
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_raw_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
