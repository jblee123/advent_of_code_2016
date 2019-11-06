use std::fs;

#[derive(Clone, Debug, PartialEq, Eq)]
struct KeyPos {
    x: i32,
    y: i32,
}

const KEYPAD_SIZE: usize = 5;
const MAX_KEYPAD_DIM: i32 = KEYPAD_SIZE as i32 - 1;

const KEYPAD: [[u8; KEYPAD_SIZE]; KEYPAD_SIZE] = [
    [b' ', b' ', b'5', b' ', b' ', ],
    [b' ', b'2', b'6', b'A', b' ', ],
    [b'1', b'3', b'7', b'B', b'D', ],
    [b' ', b'4', b'8', b'C', b' ', ],
    [b' ', b' ', b'9', b' ', b' ', ],
];

fn get_keypad_char(pos: &KeyPos) -> Option<u8> {
    if pos.x < 0 || pos.y < 0
        || pos.x > MAX_KEYPAD_DIM || pos.y > MAX_KEYPAD_DIM {

        return None;
    }

    let c = KEYPAD[pos.x as usize][pos.y as usize];
    match c {
        b' ' => None,
        _ => Some(c),
    }
}

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

fn apply_dir(pos: KeyPos, dir: KeypadDir) -> KeyPos {
    if (pos.x == 0 && dir == KeypadDir::Left)
        || (pos.x == MAX_KEYPAD_DIM && dir == KeypadDir::Right)
        || (pos.y == 0 && dir == KeypadDir::Up)
        || (pos.y == MAX_KEYPAD_DIM && dir == KeypadDir::Down) {

        return pos;
    }

    let mut new_pos = pos.clone();
    match dir {
        KeypadDir::Up    => new_pos.y -= 1,
        KeypadDir::Down  => new_pos.y += 1,
        KeypadDir::Left  => new_pos.x -= 1,
        KeypadDir::Right => new_pos.x += 1,
    };

    match get_keypad_char(&new_pos) {
        None => pos,
        Some(_) => new_pos,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = input.lines();

    let mut pos = KeyPos { x: 0, y: 2 };

    print!("keypad num: ");
    for line in lines {
        for dir_byte in line.bytes() {
            let dir = KeypadDir::from_byte(dir_byte);
            pos = apply_dir(pos, dir)
        }

        print!("{}", get_keypad_char(&pos).unwrap() as char);
    }

    println!("")
}
