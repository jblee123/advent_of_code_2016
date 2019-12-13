use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs;
use std::result::Result;
use std::str::FromStr;

enum MazeSpace {
    Wall,
    Open(u32),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: u32,
    y: u32,
}

const START_COORD: Coord = Coord { x: 1, y: 1 };
const END_COORD: Coord = Coord { x: 31, y: 39 };

struct Maze {
    spaces: HashMap<Coord, MazeSpace>,
    origins: LinkedList<Coord>,
    wall_param: u32,
}

impl Maze {
    pub fn new(start_coord: Coord, wall_param: u32) -> Maze {
        let mut new_maze = Maze {
            spaces: HashMap::new(),
            origins: LinkedList::new(),
            wall_param: wall_param,
        };
        new_maze.spaces.insert(
            start_coord,
            MazeSpace::Open(0));
        new_maze.origins.push_back(start_coord);
        new_maze
    }

    fn offset_pos(pos: Coord, x_offset: i32, y_offset: i32) -> Option<Coord> {
        if x_offset < 0 && (x_offset.abs() as u32) > pos.x {
            return None;
        }

        if y_offset < 0 && (y_offset.abs() as u32) > pos.y {
            return None;
        }

        Some(
            Coord {
                x: pos.x.wrapping_add(x_offset as u32),
                y: pos.y.wrapping_add(y_offset as u32),
            })
    }

    fn is_wall(pos: Coord, wall_param: u32) -> bool {
        let mut val = (pos.x * pos.x) + (3 * pos.x) + (2 * pos.x * pos.y) + pos.y
            + (pos.y * pos.y)
            + wall_param;

        let mut num_ones = 0;
        while val != 0 {
            if val & 1 == 1 {
                num_ones += 1;
            }

            val = val >> 1;
        }

        (num_ones & 1) == 1
    }

    fn process_offset(&mut self, pos: Coord, pos_dist: u32,
        x_offset: i32, y_offset: i32) {

        let new_pos = match Maze::offset_pos(pos, x_offset, y_offset) {
            None => return,
            Some(new_pos) => new_pos,
        };

        if self.spaces.contains_key(&new_pos) {
            return;
        }

        if Maze::is_wall(new_pos, self.wall_param) {
            self.spaces.insert(new_pos, MazeSpace::Wall);
        } else {
            self.spaces.insert(new_pos, MazeSpace::Open(pos_dist + 1));
            self.origins.push_back(new_pos);
        }
    }

    pub fn dist_to(&mut self, end_coord: Coord) -> Result<u32, &str> {
        loop {
            if let Some(space) = self.spaces.get(&end_coord) {
                match space {
                    MazeSpace::Wall => return Err("no solution: wall"),
                    MazeSpace::Open(dist) => return Ok(*dist),
                };
            }

            if self.origins.is_empty() {
                return Err("no solution: unreachable");
            }

            let pos = self.origins.pop_front().unwrap();
            let pos_dist = match *self.spaces.get(&pos).unwrap() {
                MazeSpace::Wall => panic!("next origin was a wall"),
                MazeSpace::Open(dist) => dist,
            };

            self.process_offset(pos, pos_dist,  0, -1);
            self.process_offset(pos, pos_dist,  0,  1);
            self.process_offset(pos, pos_dist, -1,  0);
            self.process_offset(pos, pos_dist,  1,  0);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let wall_param = u32::from_str(input.trim()).unwrap();
    println!("input: {}", wall_param);

    let mut maze = Maze::new(START_COORD, wall_param);
    let dist = maze.dist_to(END_COORD).unwrap();
    println!("dist: {}", dist);
}
