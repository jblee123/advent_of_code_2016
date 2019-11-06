use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct KeyPos {
    x: i32,
    y: i32,
}

const KEYPAD_SIZE: usize = 3;
const MAX_KEYPAD_DIM: i32 = KEYPAD_SIZE as i32 - 1;

const KEYPAD_NUMS: [[i32; KEYPAD_SIZE]; KEYPAD_SIZE] = [
    [ 1, 4, 7 ],
    [ 2, 5, 8 ],
    [ 3, 6, 9 ]
];

#[derive(PartialEq, Eq)]
enum KeypadDir { Up, Down, Left, Right }

impl KeypadDir {
    fn from_byte(b: u8) -> KeypadDir {
        match b {
            b'U' => KeypadDir::Up,
            b'D' => KeypadDir::Down,
            b'L' => KeypadDir::Left,
            b'R' => KeypadDir::Right,
            _ => panic!("bad dir byte: {} ({})", b, b as char),
        }
    }
}

fn apply_dir(mut pos: KeyPos, dir: KeypadDir) -> KeyPos {
    if (pos.x == 0 && dir == KeypadDir::Left)
        || (pos.x == MAX_KEYPAD_DIM && dir == KeypadDir::Right)
        || (pos.y == 0 && dir == KeypadDir::Up)
        || (pos.y == MAX_KEYPAD_DIM && dir == KeypadDir::Down) {

        return pos;
    }

    match dir {
        KeypadDir::Up    => pos.y -= 1,
        KeypadDir::Down  => pos.y += 1,
        KeypadDir::Left  => pos.x -= 1,
        KeypadDir::Right => pos.x += 1,
    };

    pos
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = input.lines();

    let mut pos = KeyPos { x: 1, y: 1 };

    print!("keypad num: ");
    for line in lines {
        for dir_byte in line.bytes() {
            let dir = KeypadDir::from_byte(dir_byte);
            pos = apply_dir(pos, dir)
        }

        print!("{}", KEYPAD_NUMS[pos.x as usize][pos.y as usize]);
    }
    println!("");
}
