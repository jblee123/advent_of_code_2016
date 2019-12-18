// NOTE: This problem is not solved by code other than printing the initial
// state out to the terminal so you can inspect it and then do some math and
// logic to get the answer. The code below can theoretically solve it, but not
// practically because it's too much work.

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::fs;
// use std::result::Result;
// use std::str;
use std::str::FromStr;
use std::time;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Coord {
    x: u8,
    y: u8,
}

const GOAL_COORD: Coord = Coord { x: 0, y: 0 };

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct Node {
    // pos: Coord,
    // size: u32,
    used: u8,
    avail: u8,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct SimState {
    goal_data_pos: Coord,
    num_moves: u32,
    nodes: Vec<Node>,
}

fn coord_to_idx(coord: Coord, max_x: u8) -> usize {
    (max_x as usize + 1) * (coord.y as usize) + (coord.x as usize)
}

fn get_adjecent_coords(
    pos: Coord, min_x: u8, min_y: u8, max_x: u8, max_y: u8) -> Vec<Coord> {

    let mut adj = Vec::with_capacity(4);

    if pos.y > min_y {
        adj.push(Coord { x: pos.x, y: pos.y - 1 });
    }
    if pos.y < max_y {
        adj.push(Coord { x: pos.x, y: pos.y + 1 });
    }
    if pos.x > min_x {
        adj.push(Coord { x: pos.x - 1, y: pos.y });
    }
    if pos.x < max_x {
        adj.push(Coord { x: pos.x + 1, y: pos.y });
    }

    adj
}

fn parse_node(s: &str) -> (Node, Coord) {
    assert_eq!(s.split_whitespace().count(), 5);

    let u8_from_size = |size_str: &str| {
        let cut_last = &size_str[..(size_str.len() - 1)];
        let val_32 = u32::from_str(cut_last).unwrap();
        let val_32 = std::cmp::min(val_32, 255);
        val_32 as u8
    };

    let u8_from_coord = |coord_str: &str| {
        u8::from_str(&coord_str[1..]).unwrap()
    };

    let mut parts = s.split_whitespace();
    let mut coord_parts = parts.next().unwrap().split("-");
    coord_parts.next();
    let x = u8_from_coord(coord_parts.next().unwrap());
    let y = u8_from_coord(coord_parts.next().unwrap());
    // let size = val_from_size(parts.next().unwrap());
    parts.next(); // skip the size
    let used = u8_from_size(parts.next().unwrap());
    let avail = u8_from_size(parts.next().unwrap());

    let node = Node {
        // pos: Coord { x: x, y: y },
        // size: size,
        used: used,
        avail: avail,
    };

    let coord = Coord { x: x, y: y };

    (node, coord)
}

fn do_tests() {
    let result = get_adjecent_coords(Coord { x: 1, y: 1 }, 0, 0, 2, 2);
    assert_eq!(result, vec![
            Coord { x: 1, y: 0 },
            Coord { x: 1, y: 2 },
            Coord { x: 0, y: 1 },
            Coord { x: 2, y: 1 },
        ]);

    let result = get_adjecent_coords(Coord { x: 0, y: 0 }, 0, 0, 2, 2);
    assert_eq!(result, vec![
            Coord { x: 0, y: 1 },
            Coord { x: 1, y: 0 },
        ]);

    let result = get_adjecent_coords(Coord { x: 1, y: 0 }, 0, 0, 2, 2);
    assert_eq!(result, vec![
            Coord { x: 1, y: 1 },
            Coord { x: 0, y: 0 },
            Coord { x: 2, y: 0 },
        ]);

    let result = get_adjecent_coords(Coord { x: 0, y: 1 }, 0, 0, 2, 2);
    assert_eq!(result, vec![
            Coord { x: 0, y: 0 },
            Coord { x: 0, y: 2 },
            Coord { x: 1, y: 1 },
        ]);

    let result = get_adjecent_coords(Coord { x: 2, y: 2 }, 0, 0, 2, 2);
    assert_eq!(result, vec![
            Coord { x: 2, y: 1 },
            Coord { x: 1, y: 2 },
        ]);

    let result = coord_to_idx(Coord { x: 0, y: 0 }, 2);
    assert_eq!(result, 0);

    let result = coord_to_idx(Coord { x: 1, y: 0 }, 2);
    assert_eq!(result, 1);

    let result = coord_to_idx(Coord { x: 0, y: 1 }, 2);
    assert_eq!(result, 3);
}

fn main() {
    do_tests();

    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut lines = input.lines();
    lines.next();
    lines.next();

    let mut min_x = std::u8::MAX;
    let mut max_x = std::u8::MIN;
    let mut min_y = std::u8::MAX;
    let mut max_y = std::u8::MIN;
    let nodes: HashMap<Coord, Node> = lines
        .map(|line| {
            let (node, coord) = parse_node(line);
            min_x = std::cmp::min(min_x, coord.x);
            max_x = std::cmp::max(max_x, coord.x);
            min_y = std::cmp::min(min_y, coord.y);
            max_y = std::cmp::max(max_y, coord.y);
            (coord, node)
        })
        .collect();

    println!("min/max x: {}, {}", min_x, max_x);
    println!("min/max y: {}, {}", min_y, max_y);

    assert_eq!(min_x, 0);
    assert_eq!(min_y, 0);

    let start_state = {
        let mut sim_state = SimState {
            goal_data_pos: Coord { x: max_x, y: 0 },
            num_moves: 0,
            nodes: Vec::<Node>::with_capacity(nodes.len()),
        };

        (min_y..=max_y).for_each(|y| {
            (min_x..=max_x).for_each(|x| {
                let coord = Coord { x: x, y: y };
                sim_state.nodes.push(*nodes.get(&coord).unwrap());
            });
        });

        sim_state
    };

    // (min_y..=max_y).for_each(|y| {
    //     (min_x..=max_x).for_each(|x| {
    //         let coord = Coord { x: x, y: y };
    //         let idx = coord_to_idx(coord, max_x);
    //         let node = &start_state.nodes[idx];
    //         print!("{}/{}  ", node.used, node.avail);
    //     });

    //     println!("");
    // });

    // println!("{:?}", start_state.nodes);


    let mut visited_states = HashSet::<Vec<Node>>::new();
    let mut state_queue = LinkedList::<SimState>::new();
    state_queue.push_back(start_state);

    let mut num_states_instpected = 0u64;

    let now = time::Instant::now();
    let ans = loop {
        if state_queue.is_empty() {
            break None;
        }

        let cur_state = state_queue.pop_front().unwrap();
        if cur_state.goal_data_pos == GOAL_COORD {
            break Some(cur_state.num_moves);
        }

        (min_x..=max_x).for_each(|x| {
            (min_y..=max_y).for_each(|y| {
                let coord_a = Coord { x: x, y: y };
                let idx_a = coord_to_idx(coord_a, max_x);
                let node_a = &cur_state.nodes[idx_a];
                if node_a.used == 0 {
                    return;
                }

                for coord_b in get_adjecent_coords(
                    coord_a, min_x, min_y, max_x, max_y) {

                    let idx_b = coord_to_idx(coord_b, max_x);
                    let node_b = &cur_state.nodes[idx_b];
                    if node_a.used <= node_b.avail {
                        let mut new_state = cur_state.clone();
                        new_state.nodes[idx_b].used += node_a.used;
                        new_state.nodes[idx_b].avail -= node_a.used;
                        new_state.nodes[idx_a].used -= node_a.used;
                        new_state.nodes[idx_a].avail += node_a.used;

                        if coord_a == cur_state.goal_data_pos {
                            new_state.goal_data_pos = coord_b;
                        }

                        new_state.num_moves += 1;

                        if !visited_states.contains(&new_state.nodes) {
                            visited_states.insert(new_state.nodes.clone());
                            state_queue.push_back(new_state);
                        }
                    }
                }
            });
        });

        num_states_instpected += 1;
        if num_states_instpected % 10000 == 0 {
            let (min, sec) = {
                let sec = now.elapsed().as_secs();
                (sec / 60, sec % 60)
            };

            let mut num_empty = 0;
            (min_x..=max_x).for_each(|x| {
                (min_y..=max_y).for_each(|y| {
                    let coord = Coord { x: x, y: y };
                    let idx = coord_to_idx(coord, max_x);
                    let node = &cur_state.nodes[idx];
                    if node.used == 0 {
                        num_empty += 1;
                    }
                });
            });

            println!(concat!(
                "{} states seen in {}m{}s. move count: {}, num empty: {}, ",
                "queue len: {}, visited len: {}"),
                num_states_instpected, min, sec, cur_state.num_moves, num_empty,
                state_queue.len(), visited_states.len());
        }
    };

    match ans {
        None => println!("no solution"),
        Some(num_moves) => println!("solution in {} moves", num_moves),
    }




    // // let mut max_avail_for_big_drives = 0;
    // let mut min_used = std::u8::MAX;
    // let mut valid_pairs = 0;
    // // let mut empty_nodes = 0;
    // let now = time::Instant::now();
    // (min_x..=max_x).for_each(|x| {
    //     (min_y..=max_y).for_each(|y| {


    //         let coord_a = Coord { x: x, y: y };
    //         let idx_a = coord_to_idx(coord_a, max_x);
    //         let node_a = &start_state.nodes[idx_a];
    //         if node_a.used == 0 {
    //             return;
    //         }
    //         if node_a.used < min_used {
    //             min_used = node_a.used;
    //         }

    //         for coord_b in get_adjecent_coords(
    //             coord_a, min_x, min_y, max_x, max_y) {

    //             let idx_b = coord_to_idx(coord_b, max_x);
    //             let node_b = &start_state.nodes[idx_b];
    //             if node_a.used <= node_b.avail {
    //                 valid_pairs += 1;
    //             }
    //         }






    //         // let coord_a = Coord { x: x_a, y: y_a };
    //         // let node_a = nodes.get(&coord_a).unwrap();
    //         // if node_a.used == 0 {
    //         //     empty_nodes += 1;
    //         // }


    //         // let node_a = &nodes[x as usize][y as usize];
    //         // // if node_a.size > 500 && node_a.avail > max_avail_for_big_drives {
    //         // //     max_avail_for_big_drives = node_a.avail;
    //         // // }
    //         // if node_a.used == 0 {
    //         //     return;
    //         // }
    //         // if node_a.used < min_used {
    //         //     min_used = node_a.used;
    //         // }

    //         // for adj_coord in get_adjecent_coords(
    //         //     Coord { x: x, y: y }, min_x, min_y, max_x, max_y) {

    //         //     let node_b = &nodes[adj_coord.x as usize][adj_coord.y as usize];
    //         //     if node_a.used <= node_b.avail {
    //         //         valid_pairs += 1;
    //         //     }
    //         // }




    //         // (min_x..=max_x).for_each(|x_b| {
    //         //     (min_y..=max_y).for_each(|y_b| {
    //         //         let coord_a = Coord { x: x_a, y: y_a };
    //         //         let coord_b = Coord { x: x_b, y: y_b };
    //         //         if coord_a == coord_b {
    //         //             return;
    //         //         }

    //         //         let node_a = &nodes[x_a as usize][y_a as usize];
    //         //         let node_b = &nodes[x_b as usize][y_b as usize];

    //         //         if node_a.used == 0 {
    //         //             return;
    //         //         }

    //         //         if node_a.used <= node_b.avail
    //         //             && adjacent(node_a.pos, node_b.pos) {

    //         //             valid_pairs += 1;
    //         //         }
    //         //     });
    //         // });
    //     });
    // });

    // assert_eq!(valid_pairs, 4);

    // // println!("max_avail_for_big_drives: {}", max_avail_for_big_drives);
    // println!("min_used: {}", min_used);
    // println!("elapsed: {}", now.elapsed().as_secs_f32());

    // println!("valid pairs: {}", valid_pairs);
    // // println!("empty nodes: {}", empty_nodes);
    // println!("Node size: {}", std::mem::size_of::<Node>());










    // println!("\033[1;31mNode size: {}\033[0m;", std::mem::size_of::<Node>());

    // // println!("This should be white.");
    // // println!("\x1b[1;31mThis should be red.\x1b[0m");
    // // println!("This should be white.");

    // // println!("\x1B[6n\n");

    // // let mut num = 0;
    // // loop {
    // //     use std::io::Write;

    // //     print!("\x1b[6;3HHello {} ", num);
    // //     num += 1;
    // //     std::io::stdout().flush().unwrap();
    // //     std::thread::sleep(std::time::Duration::from_secs(1));
    // // }
}
