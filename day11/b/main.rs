use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::fmt;
use std::fs;

const NUM_FLOORS: usize = 4;

#[derive(Clone, Debug)]
enum Item {
    Chip(u32),
    Generator(u32),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Item::Chip(num) => write!(f, "C{}", num),
            Item::Generator(num) => write!(f, "G{}", num),
        }
    }
}

type Floor = Vec<Item>;
// struct FloorDisp<'a>(&'a Floor);

// impl<'a> fmt::Display for FloorDisp<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.write_str(&self.0.iter()
//             .map(|item| format!("{}", item))
//             .collect::<Vec<String>>()
//             .join(", "))
//     }
// }

#[derive(Clone)]
struct Facility {
    elev_floor: usize,
    floors: [Floor; NUM_FLOORS],
}

impl fmt::Display for Facility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..NUM_FLOORS {
            let floor = NUM_FLOORS - i - 1;
            let on_cur_floor = floor == self.elev_floor;
            write!(f, "{}", if on_cur_floor { "E" } else { " " })?;
            for item in self.floors[floor].iter() {
                write!(f, " {}", item)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct State {
    cur_move: u32,
    facility: Facility,
}

fn find_nth(s: &str, target: char, num_to_skip: u32) -> Option<usize> {
    let mut num_to_skip_mut = num_to_skip;
    s.find(|c| {
        if c == target {
            if num_to_skip_mut == 0 {
                return true;
            } else {
                num_to_skip_mut -= 1;
            }
        }

        false
    })
}

fn get_item_type_num(
    type_str: &str, type_map: &mut HashMap<String, u32>) -> u32 {

    if let Some(val) = type_map.get(type_str) {
        return *val;
    }

    let new_val = (type_map.len() + 1) as u32;
    type_map.insert(type_str.to_string(), new_val);
    new_val
}

fn sort_floor_items(floor: &mut Floor) {
    floor.sort_by(|a, b| {
        let (a_type_num, a_type_val) = match a {
            Item::Chip(val) => (0, val),
            Item::Generator(val) => (1, val),
        };

        let (b_type_num, b_type_val) = match b {
            Item::Chip(val) => (0, val),
            Item::Generator(val) => (1, val),
        };

        if a_type_val == b_type_val {
            a_type_num.cmp(&b_type_num)
        } else {
            a_type_val.cmp(&b_type_val)
        }
    });
}

fn line_to_floor(line: &str, type_map: &mut HashMap<String, u32>) -> Floor {
    let mut floor = Floor::new();

    let fourth_space_idx = find_nth(line, ' ', 3).unwrap();
    let line = &line[fourth_space_idx..(line.len() - 1)];

    if line.starts_with(" nothing") {
        return floor
    }

    // println!("");

    let item_strs = line.split(" a ")
        .map(|s| s.trim().trim_end_matches(" and").trim_end_matches(","))
        .filter(|s| *s != "")
        .collect::<Vec<&str>>();
    for item_str in item_strs {
        // println!("  '{}'", item_str);
        let parts = item_str.split(" ").collect::<Vec<&str>>();
        let item = match parts[1] {
            "generator" => {
                let type_num = get_item_type_num(&parts[0], type_map);
                Item::Generator(type_num)
            },
            "microchip" =>  {
                let type_str = parts[0].trim_end_matches("-compatible");
                let type_num = get_item_type_num(&type_str, type_map);
                Item::Chip(type_num)
            },
            bad_str => panic!("bad item type: {}", bad_str),
        };
        floor.push(item);
    }

    sort_floor_items(&mut floor);

    // println!("{}", FloorDisp(&floor));

    floor
}

fn get_init_state(input: &str, type_map: &mut HashMap<String, u32>) -> State {
    
    let mut facility = Facility {
        elev_floor: 0,
        floors: Default::default(),
    };
    for (i, line) in input.lines().enumerate() {
        facility.floors[i] = line_to_floor(&line, type_map);
    }

    State {
        cur_move: 0,
        facility: facility,
    }
}

fn get_floor_mutations(floor: &Floor) -> Vec<(Vec<Item>, Floor)> {
    let mut results: Vec<(Vec<Item>, Floor)> = vec![];
    for i in 0..floor.len() {
        results.push(
            (
                vec![floor[i].clone()],
                {
                    let mut new_floor = floor.clone();
                    new_floor.remove(i);
                    new_floor
                }
            )
        );
    }

    for i in 0..(floor.len() - 1) {
        for j in (i + 1)..floor.len() {
            results.push(
                (
                    vec![floor[i].clone(), floor[j].clone()],
                    {
                        let mut new_floor = floor.clone();
                        new_floor.remove(j);
                        new_floor.remove(i);
                        new_floor
                    }
                )
            );
        }
    }

    results
}

fn is_legal_facility(facility: &Facility) -> bool {
    for floor in facility.floors.iter() {
        let generators = floor.iter()
            .filter_map(|item| {
                if let Item::Generator(num) = item { Some(*num) } else { None }
            })
            .collect::<Vec<u32>>();
        let chips = floor.iter()
            .filter_map(|item| {
                if let Item::Chip(num) = item { Some(*num) } else { None }
            })
            .collect::<Vec<u32>>();

        for chip_num in chips {
            if !generators.is_empty() && !generators.contains(&chip_num) {
                return false;
            }
        }
    }

    true
}

fn apply_facility_mutation(
    facility: &Facility, new_floor_idx: usize, items_to_move: &Vec<Item>,
    new_floor: &Floor, visited: &HashSet<String>)
    -> Option<Facility> {

    if new_floor_idx >= NUM_FLOORS {
        return None;
    }

    let mut new_facility = facility.clone();
    new_facility.elev_floor = new_floor_idx;
    new_facility.floors[facility.elev_floor] = new_floor.clone();
    new_facility.floors[new_floor_idx].append(&mut items_to_move.clone());

    sort_floor_items(&mut new_facility.floors[new_floor_idx]);

    let fac_str = format!("{}", new_facility);
    if visited.contains(&fac_str) {
        // println!("already visited...");
        return None;
    }

    if !is_legal_facility(&new_facility) {
        // println!("not legal facility...");
        return None;
    }

    Some(new_facility)
}

fn generate_next_states(state: &State, visited: &mut HashSet<String>)
    -> Vec<State> {

    let mut next_states = vec![];

    let mutations = get_floor_mutations(
        &state.facility.floors[state.facility.elev_floor]);

    for (to_move, resulting_floor) in mutations.iter() {
        // println!("{:?}, {:?}", to_move, resulting_floor);

        if state.facility.elev_floor > 0 {
            if let Some(new_facility) = apply_facility_mutation(
                &state.facility, state.facility.elev_floor - 1, &to_move,
                &resulting_floor, visited) {

                next_states.push(
                    State {
                        cur_move: state.cur_move + 1,
                        facility: new_facility,
                    });
            }
        }

        if state.facility.elev_floor < NUM_FLOORS - 1 {
            if let Some(new_facility) = apply_facility_mutation(
                &state.facility, state.facility.elev_floor + 1, &to_move,
                &resulting_floor, visited) {

                next_states.push(
                    State {
                        cur_move: state.cur_move + 1,
                        facility: new_facility,
                    });
            }
        }
    }
    
    next_states
}

fn is_final_state(facility: &Facility) -> bool {
    facility.floors[0..(NUM_FLOORS - 1)].iter()
        .all(|floor| floor.is_empty())
}

fn print_state(state: &State, label: &str) {
    if label.len() > 0 {
        println!("{}", label);
    }
    println!("move: {}", state.cur_move);
    println!("{}", state.facility);
}

fn process_state(state: &State, states: &mut LinkedList<State>,
    visited: &mut HashSet<String>) -> bool {
    
    // print_state(&state, "generating next states for:");
    let next_states = generate_next_states(&state, visited);
    // println!("generated {} new states", next_states.len());
    for next_state in next_states {
        if is_final_state(&next_state.facility) {
            print_state(&next_state, "found final state:");
            return true;
        }
        states.push_back(next_state);
        let next_state = states.back().unwrap();
        visited.insert(format!("{}", next_state.facility));
        // print_state(&next_state, "");
    }

    false
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    // assert_eq!(find_nth("a b cd e f", ' ', 2), Some(6));

    let mut type_map = HashMap::new();
    // let mut states = vec![get_init_state(&input, &mut type_map)];
    let mut states = LinkedList::new();
    states.push_back(get_init_state(&input, &mut type_map));

    // print!("{}", states.front().unwrap().facility);

    // print!("{}", format!("{}", states[0].facility));

    let mut visited = HashSet::<String>::new();
    let mut i: u64 = 0;
    loop {
        match states.pop_front() {
            Some(front_state) => {
                if process_state(&front_state, &mut states, &mut visited) {
                    break;
                }
            },
            None => {
                println!("No solution.");
                break;
            }
        }

        if i % 1000 == 0 {
            println!("i: {}, len states: {}, front move count: {}",
                i, states.len(), states.front().unwrap().cur_move);
        }
        i += 1;
    }



    // line_to_floor(concat!(
    //     "The first floor contains a thulium generator, a thulium-compatible ",
    //     "microchip, a plutonium generator, and a strontium generator."),
    //     &mut type_map
    // );
    // line_to_floor(concat!(
    //     "The second floor contains a plutonium-compatible microchip and a ",
    //     "strontium-compatible microchip."),
    //     &mut type_map);
    // line_to_floor("The fourth floor contains nothing relevant.", &mut type_map);

}
