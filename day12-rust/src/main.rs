use std::{
  collections::{HashMap},
  fs::File,
  io::{self, BufRead},
  path::Path,
};

fn main() {
  println!("Hello, world!");
  let cave_paths = get_cave_paths_from_input().expect("Error reading input");
  find_all_paths(&cave_paths);
  println!("Done!");
}

fn find_all_paths(paths_hashmap: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
  let start_cave = "start";
  let current_path = vec![start_cave.to_string()];
  let mut all_paths: Vec<Vec<String>> = vec![];
  let possible_caves = paths_hashmap.get(start_cave).expect("start cave should exist");
  explore_paths(current_path, &mut all_paths, possible_caves, paths_hashmap);
  
  all_paths
}

fn explore_paths(
  current_path: Vec<String>,
  all_paths: &mut Vec<Vec<String>>,
  possible_caves: &Vec<String>,
  cave_paths_hashmap: &HashMap<String, Vec<String>>,
) {
  for cave in possible_caves {
    if cave == "end" {
      all_paths.push(current_path);
      return;
    } else if cave == "start" {
      return;
    } else if cave.to_lowercase() == *cave && current_path.contains(cave) {
      return;
    } else {
      let next_possible_caves = cave_paths_hashmap.get(cave).expect("cave should exist");
      for cave in next_possible_caves {
        let mut new_path = current_path.clone();
        new_path.push(cave.to_string());
        explore_paths(
          new_path,
          all_paths,
          next_possible_caves,
          cave_paths_hashmap,
        );
      }
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
