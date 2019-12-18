use std::collections::HashSet;
use std::collections::LinkedList;
use std::fs;
// use std::result::Result;
// use std::str;
// use std::str::FromStr;

type Map = Vec<Vec<u8>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: u8,
    y: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct WorldState {
    pos: Coord,
    tagged: u16,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct SimState {
    world_state: WorldState,
    num_steps: u32,
}

fn parse_map(input_str: &str) -> (Map, Coord, u32) {

     let lines: Vec<Vec<u8>> = input_str.lines()
        .map(|line| line.trim().bytes().collect::<Vec<u8>>())
        .collect();

    let width = lines[0].len();
    let height = lines.len();

    let mut map = vec![vec![0u8; height]; width];
    let mut num_chkpts = 0;

    let mut start_pos = Coord { x: 0, y: 0 };
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            map[x][y] = *c;
            match *c {
                 b'0' => {
                    start_pos = Coord { x: x as u8, y: y as u8 };
                    // map[x][y] = b'.';
                },
                b'1'..=b'9' => { num_chkpts += 1; },
                _ => {}
            }
        }
    }


    (map, start_pos, num_chkpts)
}

fn get_next_positions(pos: Coord, map: &Map) -> Vec<Coord> {
    let width = map.len() as i32;
    let height = map[0].len() as i32;

    let pos_x = pos.x as i32;
    let pos_y = pos.y as i32;

    let test_coord = |x: i32, y: i32| -> bool {
        if x < 0 || y < 0 || x >= width || y >= height {
            false
        } else {
            map[x as usize][y as usize] != b'#'
        }
    };

    let to_coord = |x: i32, y: i32| -> Coord {
        Coord { x: x as u8, y: y as u8 }
    };

    let mut next_positions = vec![];

    if test_coord(pos_x, pos_y - 1) {
        next_positions.push(to_coord(pos_x, pos_y - 1));
    }

    if test_coord(pos_x, pos_y + 1) {
        next_positions.push(to_coord(pos_x, pos_y + 1));
    }

    if test_coord(pos_x - 1, pos_y) {
        next_positions.push(to_coord(pos_x - 1, pos_y));
    }

    if test_coord(pos_x + 1, pos_y) {
        next_positions.push(to_coord(pos_x + 1, pos_y));
    }

    next_positions
}

fn get_chkpt(pos: Coord, map: &Map) -> Option<u16> {
    let c = map[pos.x as usize][pos.y as usize];
    match c {
        b'0'..=b'9' => Some((c - b'0') as u16),
        _ => None
    }
}

fn do_tests() {
    let sample_input = concat!(
        "###########\n",
        "#0.1.....2#\n",
        "#.#######.#\n",
        "#4.......3#\n",
        "###########\n",);
    let (sample_map, start_pos, num_chkpts) = parse_map(sample_input);
    assert_eq!(sample_map, vec![
        vec![b'#', b'#', b'#', b'#', b'#'],
        vec![b'#', b'0', b'.', b'4', b'#'],
        vec![b'#', b'.', b'#', b'.', b'#'],
        vec![b'#', b'1', b'#', b'.', b'#'],
        vec![b'#', b'.', b'#', b'.', b'#'],
        vec![b'#', b'.', b'#', b'.', b'#'],
        vec![b'#', b'.', b'#', b'.', b'#'],
        vec![b'#', b'.', b'#', b'.', b'#'],
        vec![b'#', b'.', b'#', b'.', b'#'],
        vec![b'#', b'2', b'.', b'3', b'#'],
        vec![b'#', b'#', b'#', b'#', b'#'],
    ]);
    assert_eq!(start_pos, Coord { x: 1, y: 1 });
    assert_eq!(num_chkpts, 4);

    let result = get_next_positions(Coord { x: 1, y: 1}, &sample_map);
    assert_eq!(result, vec![
        Coord { x: 1, y: 2 }, Coord { x: 2, y: 1 }
    ]);

    let result = get_next_positions(Coord { x: 9, y: 3}, &sample_map);
    assert_eq!(result, vec![
        Coord { x: 9, y: 2 }, Coord { x: 8, y: 3 }
    ]);

    let result = get_chkpt(Coord { x: 1, y: 1 }, &sample_map);
    assert_eq!(result, Some(0));

    let result = get_chkpt(Coord { x: 2, y: 1 }, &sample_map);
    assert_eq!(result, None);

    let result = get_chkpt(Coord { x: 3, y: 1 }, &sample_map);
    assert_eq!(result, Some(1));
}

fn main() {
    do_tests();

    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let (map, start_pos, num_chkpts) = parse_map(&input);

    const TARGET_TAGGED: u16 = std::u16::MAX;
    const TARGET_TAGGED_EXCEPT_START: u16 = std::u16::MAX << 1;

    let mut visited = HashSet::<WorldState>::new();
    let mut states = LinkedList::<SimState>::new();

    states.push_back(
        SimState {
            world_state: WorldState {
                pos: start_pos,
                tagged: (std::u16::MAX << (num_chkpts + 1)),
            },
            num_steps: 0,
        });

    let result = loop {
        if states.is_empty() {
            break None;
        }

        let cur = states.pop_front().unwrap();
        if cur.world_state.tagged == TARGET_TAGGED {
            break Some(cur.num_steps);
        }

        for next_pos in get_next_positions(cur.world_state.pos, &map) {

            let new_tagged = match get_chkpt(next_pos, &map) {
                Some(chkpt) => {
                    let seen_all_chkpts =
                        cur.world_state.tagged == TARGET_TAGGED_EXCEPT_START;
                    if (chkpt == 0 && seen_all_chkpts) || (chkpt > 0) {
                        cur.world_state.tagged | (1 << chkpt)
                    } else {
                        cur.world_state.tagged
                    }
                },
                None => cur.world_state.tagged,
            };

            let new_ws = WorldState {
                pos: next_pos,
                tagged: new_tagged,
            };

            if !visited.contains(&new_ws) {
                visited.insert(new_ws.clone());
                states.push_back(
                    SimState {
                        world_state: new_ws,
                        num_steps: cur.num_steps + 1,
                    });
            }
        }
    };

    match result {
        None => println!("no solution"),
        Some(num_steps) => println!("path len: {}", num_steps),
    };
}
