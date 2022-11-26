use std::{
  collections::HashMap,
  fs::File,
  io::{self, BufRead},
  path::Path,
};

fn main() {
  let (mut polymer, pair_insertion_rules) = get_template_and_pairings_from_input().expect("Couldn't read input!");
  for _ in 0..10 {
    pair_insertion_step(&mut polymer, &pair_insertion_rules);
  }
  let ((least_element, least_count), (most_element, most_count)) = find_least_and_most_common_element(&polymer);

  println!(
    "Most common element: '{}' - appeared {} times",
    most_element, most_count
  );
  println!(
    "Least common element: '{}' - appeared {} times",
    least_element, least_count
  );
  println!();
  println!("Most count - least count = {}", most_count - least_count);
}

fn find_least_and_most_common_element(polymer: &str) -> ((char, u32), (char, u32)) {
  let mut element_counts: HashMap<char, u32> = HashMap::new();
  for element in polymer.chars() {
    *element_counts.entry(element).or_insert(0) += 1;
  }

  let mut counts_vector: Vec<(char, u32)> = element_counts.into_iter().collect();
  counts_vector.sort_by(|(_, a_count), (_, b_count)| a_count.cmp(b_count));
  let (least_common_char, least_common_count) = counts_vector.first().expect("first element should exist");
  let (most_common_char, most_common_count) = counts_vector.last().expect("last element should exist");

  (
    (*least_common_char, *least_common_count),
    (*most_common_char, *most_common_count),
  )
}

fn pair_insertion_step(polymer: &mut String, pair_insertion_rules: &HashMap<String, char>) {
  let mut elements_to_insert: Vec<&char> = vec![];
  for i in 0..polymer.len() - 1 {
    let pair = polymer.get(i..i + 2).expect("Indexing error while reading polymer");
    elements_to_insert.push(
      pair_insertion_rules
        .get(pair)
        .expect(&format!("Insertion rule '{}' should exist", pair)),
    );
  }

  let mut insert_index: usize = 1;
  for element in elements_to_insert {
    polymer.insert(insert_index, element.to_owned());
    insert_index += 2;
  }
}

/** Get the template and pair insertion rules from input */
fn get_template_and_pairings_from_input() -> io::Result<(String, HashMap<String, char>)> {
  let mut template = String::new();
  let mut pairings: HashMap<String, char> = HashMap::new();

  let raw_lines_from_file = read_raw_lines_from_file("input.txt")?;
  for (line_num, raw_line_result) in raw_lines_from_file.enumerate() {
    if line_num == 0 {
      template = raw_line_result.expect("Couldn't read template!");
    } else if let Some((pair, insert_element)) = raw_line_result.unwrap_or_default().split_once(" -> ") {
      pairings.insert(pair.to_string(), insert_element.chars().next().unwrap());
    }
  }

  Ok((template, pairings))
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_raw_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
