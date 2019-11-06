use std::fs;
use std::str::FromStr;

enum TurnDir { R, L, }

#[derive(Debug)]
enum MapDir { N, E, S, W, }

#[derive(Debug)]
struct Point2D {
    x: i32,
    y: i32,
}

struct Instr {
    turn: TurnDir,
    dist: i32,
}

impl Instr {
    pub fn from_str(instr_str: &str) -> Instr {

        let first_letter = instr_str.as_bytes()[0];
        let turn = match first_letter {
            b'R' => TurnDir::R,
            b'L' => TurnDir::L,
            _ => panic!("invalid instr letter: {}", first_letter as char),
        };

        let dist = i32::from_str(&instr_str[1..]).unwrap();

        Instr {
            turn: turn,
            dist: dist,
        }
    }
}

struct Agent {
    pos: Point2D,
    dir: MapDir,
}

impl Agent {
    fn turn(&mut self, turn_dir: &TurnDir) {
        let new_dir = match turn_dir {
            TurnDir::R => 
                match self.dir {
                    MapDir::N => MapDir::E,
                    MapDir::E => MapDir::S,
                    MapDir::S => MapDir::W,
                    MapDir::W => MapDir::N,
                },
            TurnDir::L => 
                match self.dir {
                    MapDir::N => MapDir::W,
                    MapDir::E => MapDir::N,
                    MapDir::S => MapDir::E,
                    MapDir::W => MapDir::S,
                },
        };

        self.dir = new_dir;
    }

    fn go_fwd(&mut self, dist: i32) {
        match self.dir {
            MapDir::N => { self.pos.y += dist; }
            MapDir::E => { self.pos.x += dist; }
            MapDir::S => { self.pos.y -= dist; }
            MapDir::W => { self.pos.x -= dist; }
        }
    }

    fn apply(&mut self, instr: &Instr) {
        self.turn(&instr.turn);
        self.go_fwd(instr.dist);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut agent = Agent {
        pos: Point2D { x: 0, y: 0, },
        dir: MapDir::N,
    };

    for instr_str in input.as_str().split(", ") {
        let instr = Instr::from_str(instr_str);
        agent.apply(&instr);
    }

    println!("end pos: {:?}", agent.pos);
    println!("dist: {}",agent.pos.x.abs() + agent.pos.y.abs());
}
