use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::LinkedList;
use std::fs;
// use std::result::Result;
// use std::str;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Node {
    pos: Coord,
    size: u32,
    used: u32,
    avail: u32,
}

fn parse_node(s: &str) -> Node {
    assert_eq!(s.split_whitespace().count(), 5);

    let val_from_size = |size_str: &str| {
        u32::from_str(&size_str[..(size_str.len() - 1)]).unwrap()
    };

    let mut parts = s.split_whitespace();
    let mut coord_parts = parts.next().unwrap().split("-");
    coord_parts.next();
    let x = u32::from_str(&coord_parts.next().unwrap()[1..]).unwrap();
    let y = u32::from_str(&coord_parts.next().unwrap()[1..]).unwrap();
    let size = val_from_size(parts.next().unwrap());
    let used = val_from_size(parts.next().unwrap());
    let avail = val_from_size(parts.next().unwrap());

    Node {
        pos: Coord { x: x, y: y },
        size: size,
        used: used,
        avail: avail,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut lines = input.lines();
    lines.next();
    lines.next();

    let mut min_x = std::u32::MAX;
    let mut max_x = std::u32::MIN;
    let mut min_y = std::u32::MAX;
    let mut max_y = std::u32::MIN;
    let nodes: HashMap<Coord, Node> = lines
        .map(|line| {
            let node = parse_node(line);
            min_x = std::cmp::min(min_x, node.pos.x);
            max_x = std::cmp::max(max_x, node.pos.x);
            min_y = std::cmp::min(min_y, node.pos.y);
            max_y = std::cmp::max(max_y, node.pos.y);
            (node.pos, node)
        })
        .collect();

    println!("min/max x: {}, {}", min_x, max_x);
    println!("min/max y: {}, {}", min_y, max_y);

    let mut valid_pairs = 0;
    (min_x..=max_x).for_each(|x_a| {
        (min_y..=max_y).for_each(|y_a| {
            (min_x..=max_x).for_each(|x_b| {
                (min_y..=max_y).for_each(|y_b| {
                    let coord_a = Coord { x: x_a, y: y_a };
                    let coord_b = Coord { x: x_b, y: y_b };
                    if coord_a == coord_b {
                        return;
                    }

                    let node_a = nodes.get(&coord_a).unwrap();
                    let node_b = nodes.get(&coord_b).unwrap();

                    if node_a.used == 0 {
                        return;
                    }

                    if node_a.used <= node_b.avail {
                        valid_pairs += 1;
                    }
                });
            });
        });
    });

    println!("valid pairs: {}", valid_pairs);
}
