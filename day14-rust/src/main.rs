use std::{
  collections::HashMap,
  fs::File,
  io::{self, BufRead},
  path::Path,
};

fn main() {
  let (mut current_pairings, mut element_counts, pair_insertion_rules) =
    get_template_and_pairings_from_input().expect("Couldn't read input!");
  for _ in 0..40 {
    pair_insertion_step(&mut current_pairings, &mut element_counts, &pair_insertion_rules);
  }
  let ((least_element, least_count), (most_element, most_count)) = find_least_and_most_common_element(element_counts);

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

fn find_least_and_most_common_element(element_counts: HashMap<char, u64>) -> ((char, u64), (char, u64)) {
  let mut counts_vector: Vec<(char, u64)> = element_counts.into_iter().collect();
  counts_vector.sort_by(|(_, a_count), (_, b_count)| a_count.cmp(b_count));
  let (least_common_char, least_common_count) = counts_vector.first().expect("first element should exist");
  let (most_common_char, most_common_count) = counts_vector.last().expect("last element should exist");
  (
    (*least_common_char, *least_common_count),
    (*most_common_char, *most_common_count),
  )
}

fn pair_insertion_step(
  current_pairings: &mut HashMap<String, u64>,
  element_counts: &mut HashMap<char, u64>,
  pair_insertion_rules: &HashMap<String, char>,
) {
  // Update the element counts
  for (pair, num_pairs) in current_pairings.iter() {
    let new_element = pair_insertion_rules.get(pair).expect("pair should be in rules");
    *element_counts.entry(*new_element).or_insert(0) += *num_pairs;
  }

  // Update the current pairings
  let old_pairings = current_pairings.clone();
  for (old_pair, num_pairs) in old_pairings {
    let new_element = pair_insertion_rules.get(&old_pair).expect("pair should be in rules");
    let mut new_pair_a = old_pair.get(0..1).expect("1st char of pair should exist").to_string();
    new_pair_a.push(*new_element);
    let mut new_pair_b = old_pair.get(1..2).expect("2nd char of pair should exist").to_string();
    new_pair_b.insert(0, *new_element);

    // Increase the count of the new pairs by num_pairs
    *current_pairings.entry(new_pair_a).or_insert(0) += num_pairs;
    *current_pairings.entry(new_pair_b).or_insert(0) += num_pairs;

    // Decrease the count of the old pair by num_pairs
    *current_pairings
      .get_mut(&old_pair)
      .expect("old pair should already exist") -= num_pairs;
  }
}

/** Get the template and pair insertion rules from input */
fn get_template_and_pairings_from_input(
) -> io::Result<(HashMap<String, u64>, HashMap<char, u64>, HashMap<String, char>)> {
  let mut current_pairings: HashMap<String, u64> = HashMap::new();
  let mut element_counts: HashMap<char, u64> = HashMap::new();
  let mut pair_insertion_rules: HashMap<String, char> = HashMap::new();

  let raw_lines_from_file = read_raw_lines_from_file("input.txt")?;
  for (line_num, raw_line_result) in raw_lines_from_file.enumerate() {
    if line_num == 0 {
      let template = raw_line_result.expect("Couldn't read template!");
      for i in 0..template.len() - 1 {
        let pair = template.get(i..i + 2).expect("Indexing error while reading template");
        *current_pairings.entry(pair.to_string()).or_insert(0) += 1;
      }
      for c in template.chars() {
        *element_counts.entry(c).or_insert(0) += 1;
      }
    } else if let Some((pair, insert_element)) = raw_line_result.unwrap_or_default().split_once(" -> ") {
      pair_insertion_rules.insert(pair.to_string(), insert_element.chars().next().unwrap());
    }
  }

  Ok((current_pairings, element_counts, pair_insertion_rules))
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_raw_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
