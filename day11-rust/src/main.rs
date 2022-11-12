use std::{
  cell::RefCell,
  collections::HashMap,
  fs::File,
  io::{self, BufRead},
  path::Path,
};

#[derive(PartialEq, Debug)]
enum Status {
  Charging,
  Flashed,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Cell {
  x: i8,
  y: i8,
}

#[derive(Debug)]
struct Octopus {
  status: Status,
  level: u8,
}

fn main() {
  let octopus_cells_step1 = get_octopus_cells_from_input().expect("Error reading input!");
  let mut total_flashes = 0_u32;
  for _ in 0..100 {
    let (flashes, _) = simulate_step(&octopus_cells_step1);
    total_flashes += flashes;
  }
  println!("Total flashes after 100 steps: {}", total_flashes);

  let octopus_cells_step2 = get_octopus_cells_from_input().expect("Error reading input!");
  let mut step = 0_u32;
  loop {
    step += 1;
    let (_, all_flashed) = simulate_step(&octopus_cells_step2);
    if all_flashed {
      break;
    }
  }
  println!("First step with synchonized flash: {}", step);
}

/**
Simulate one step: Increase each octopus' energy level by 1, and then check for flashes.
Returns total number of flashes this step, and whether all octopuses flashed on this step
*/
fn simulate_step(octopus_cells: &HashMap<Cell, RefCell<Octopus>>) -> (u32, bool) {
  let mut step_flashes = 0_u32;
  for (_, octopus) in octopus_cells {
    octopus.borrow_mut().level += 1;
  }
  loop {
    let mut loop_flashes = 0_u32;
    for (cell, octopus) in octopus_cells {
      if octopus.borrow().level > 9 {
        flash_octopus(&mut octopus.borrow_mut());
        for adjacent_cell in get_adjacent_cells(&cell) {
          if let Some(adjacent_octopus) = octopus_cells.get(&adjacent_cell) {
            if adjacent_octopus.borrow().status != Status::Flashed {
              adjacent_octopus.borrow_mut().level += 1;
            }
          }
        }
        loop_flashes += 1;
        step_flashes += 1;
      }
    }
    if loop_flashes == 0 {
      break;
    }
  }

  // check if all octopuses have flashed
  let any_charging = octopus_cells
    .values()
    .any(|octopus| octopus.borrow().status == Status::Charging);

  // reset octopus statuses
  for (_, octopus) in octopus_cells {
    octopus.borrow_mut().status = Status::Charging;
  }

  (step_flashes, !any_charging)
}

fn flash_octopus(octopus: &mut Octopus) {
  octopus.level = 0;
  octopus.status = Status::Flashed;
}

fn get_adjacent_cells(cell: &Cell) -> [Cell; 8] {
  let Cell { x, y } = cell;
  [
    Cell { x: *x, y: y - 1 },    // top
    Cell { x: x + 1, y: y - 1 }, // top right
    Cell { x: x - 1, y: y - 1 }, // top left
    Cell { x: x - 1, y: *y },    // left
    Cell { x: x + 1, y: *y },    // right
    Cell { x: *x, y: y + 1 },    // bottom
    Cell { x: x + 1, y: y + 1 }, // bottom right
    Cell { x: x - 1, y: y + 1 }, // bottom left
  ]
}

/** From input file, generate a hashmap of cells -> octopus along with their current levels/status */
fn get_octopus_cells_from_input() -> io::Result<HashMap<Cell, RefCell<Octopus>>> {
  let mut hashmap: HashMap<Cell, RefCell<Octopus>> = HashMap::new();
  let raw_lines_from_file = read_raw_lines_from_file("input.txt")?;
  for (y, raw_line_result) in raw_lines_from_file.enumerate() {
    for (x, level_char) in raw_line_result.unwrap_or(String::new()).chars().enumerate() {
      let cell = Cell {
        x: x.try_into().unwrap(),
        y: y.try_into().unwrap(),
      };
      let level = u8::from_str_radix(&level_char.to_string(), 10).expect("Couldn't parse energy level!");
      hashmap.insert(
        cell,
        RefCell::new(Octopus {
          status: Status::Charging,
          level,
        }),
      );
    }
  }

  Ok(hashmap)
}

/** Reads lines from file. Returns an iterator over the lines, or throws an error */
fn read_raw_lines_from_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
