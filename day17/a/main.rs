// use std::collections::HashSet;
use std::collections::LinkedList;
// use std::fs;
// use std::result::Result;
use std::str;
// use std::str::FromStr;

const MAZE_WIDTH: u32 = 4;
const MAZE_HEIGHT: u32 = 4;

// const NUM_HEX_CHARS: u32 = 16;

const MD5_CHUNK_LEN: usize = 512 / 8;
const MD5_MSG_SIZE_LEN: usize = 8;
const TO_BITS: usize = 8;
const MD5_LEN_BYTES: usize = 16;
// const MD5_LEN_HEX: usize = MD5_LEN_BYTES * 2;

#[derive(PartialEq, Eq, Debug)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn to_byte(&self) -> u8 {
        match *self {
            Dir::Up => b'U',
            Dir::Down => b'D',
            Dir::Left => b'L',
            Dir::Right => b'R',
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Coord {
    x: u32,
    y: u32,
}

#[derive(PartialEq, Eq, Debug)]
struct Move {
    dir: Dir,
    result_pos: Coord,
}

#[derive(PartialEq, Eq, Debug)]
struct Trail {
    path: Vec<u8>,
    end: Coord,
}

struct Md5ByteIterator<'a> {
    bytes: &'a[u8],
    count: usize,
    size_bytes: [u8; 8],
    num_padding_bytes: usize,
}

impl Md5ByteIterator<'_> {
    fn from_bytes(bytes: &'_[u8]) -> Md5ByteIterator<'_> {
        let num_padding_bytes =
            (MD5_CHUNK_LEN - ((bytes.len() + MD5_MSG_SIZE_LEN + 1) % MD5_CHUNK_LEN)) + 1;

        Md5ByteIterator {
            bytes: bytes,
            count: 0,
            size_bytes: (bytes.len() * TO_BITS).to_le_bytes(),
            num_padding_bytes: num_padding_bytes,
        }
    }

    fn num_chunks(&self) -> usize {
        (self.bytes.len() + self.num_padding_bytes + MD5_MSG_SIZE_LEN) / MD5_CHUNK_LEN
    }
}

impl Iterator for Md5ByteIterator<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        static PADDING: [u8; 64] = [
            0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let padding_bounds = self.bytes.len() + self.num_padding_bytes;
        let msg_len_bounds = padding_bounds + MD5_MSG_SIZE_LEN;

        let result = {
            if self.count < self.bytes.len() {
                Some(self.bytes[self.count])
            } else if self.count < padding_bounds {
                Some(PADDING[self.count - self.bytes.len()])
            } else if self.count < msg_len_bounds {
                Some(self.size_bytes[self.count - padding_bounds])
            } else {
                None
            }
        };

        if result.is_some() {
            self.count += 1;
        }

        result
    }
}

fn calc_md5(bytes: &[u8]) -> [u8; MD5_LEN_BYTES] {
    fn left_rotate(x: u32, c: u32) -> u32 {
        (x << c) | (x >> (32 - c))
    }

    // s specifies the per-round shift amounts
    static S: [u32; 64] = [
        7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
        5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
        4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
        6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
    ];

    static K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];

    // Initialize variables:
    let mut a0: u32 = 0x67452301;  // A
    let mut b0: u32 = 0xefcdab89;  // B
    let mut c0: u32 = 0x98badcfe;  // C
    let mut d0: u32 = 0x10325476;  // D

    let mut iter = Md5ByteIterator::from_bytes(bytes);

    // Process the message in successive 512-bit chunks:
    // for each 512-bit chunk of padded message
    let mut m: [u32; 16] = [0; 16];
    for _ in 0..iter.num_chunks() {
        // break chunk into sixteen 32-bit words m[j], 0 ≤ j ≤ 15
        for i in 0..16 {
            m[i] = (iter.next().unwrap() as u32)
                | (iter.next().unwrap() as u32) << 8
                | (iter.next().unwrap() as u32) << 16
                | (iter.next().unwrap() as u32) << 24;
        }

        // Initialize hash value for this chunk:
        let mut a: u32 = a0;
        let mut b: u32 = b0;
        let mut c: u32 = c0;
        let mut d: u32 = d0;

        // Main loop:
        for i in 0..64 {
            let (mut f, g): (u32, usize);
            if i <= 15 {
                f = (b & c) | ((!b) & d);
                g = i;
            } else if i <= 31 {
                f = (d & b) | ((!d) & c);
                g = (5 * i + 1) % 16;
            } else if i <= 47 {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            } else {
                f = c ^ (b | (!d));
                g = (7 * i) % 16;
            }

            // Be wary of the below definitions of a,b,c,d
            f = f.wrapping_add(a)
                .wrapping_add(K[i])
                .wrapping_add(m[g]);  // M[g] must be a 32-bits block
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(left_rotate(f, S[i]));
        }

        // Add this chunk's hash to result so far:
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    // var char digest[16] := a0 append b0 append c0 append d0
    // (Output is in little-endian)
    [
        ((a0 >>  0) & 0xff) as u8,
        ((a0 >>  8) & 0xff) as u8,
        ((a0 >> 16) & 0xff) as u8,
        ((a0 >> 24) & 0xff) as u8,

        ((b0 >>  0) & 0xff) as u8,
        ((b0 >>  8) & 0xff) as u8,
        ((b0 >> 16) & 0xff) as u8,
        ((b0 >> 24) & 0xff) as u8,

        ((c0 >>  0) & 0xff) as u8,
        ((c0 >>  8) & 0xff) as u8,
        ((c0 >> 16) & 0xff) as u8,
        ((c0 >> 24) & 0xff) as u8,

        ((d0 >>  0) & 0xff) as u8,
        ((d0 >>  8) & 0xff) as u8,
        ((d0 >> 16) & 0xff) as u8,
        ((d0 >> 24) & 0xff) as u8,
    ]
}

fn md5_bytes_as_str(md5: &[u8; MD5_LEN_BYTES]) -> String {
    format!("{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}\
             {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        md5[00], md5[01], md5[02], md5[03], md5[04], md5[05], md5[06], md5[07],
        md5[08], md5[09], md5[10], md5[11], md5[12], md5[13], md5[14], md5[15])
}

fn hex_char_to_digit(hex_char: u8) -> u8 {
    match hex_char {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'a' => 10,
        b'b' => 11,
        b'c' => 12,
        b'd' => 13,
        b'e' => 14,
        b'f' => 15,
        _ => panic!("bad hex char: {}", hex_char),
    }
}

fn find_valid_moves(passwd: &[u8], path: &[u8], coord: Coord) -> Vec<Move> {
    let is_open = |val: u8| -> bool {
        let val = hex_char_to_digit(val);
        if val <= 0x0a {
            return false;
        }
        if val <= 0x0f {
            return true;
        }
        panic!("bad is_open value: {}", val);
    };

    // println!("finding valid moves for:");
    // println!("  path: {}", str::from_utf8(&path).unwrap());
    // println!("  coord: {:?}", coord);

    let mut to_hash = Vec::<u8>::with_capacity(passwd.len() + path.len());
    to_hash.extend_from_slice(passwd);
    to_hash.extend_from_slice(path);

    let md5_str = md5_bytes_as_str(&calc_md5(&to_hash));
    let lock_chars = &md5_str.as_bytes()[0..4];

    let mut moves = vec![];

    // up
    if coord.y > 0 && is_open(lock_chars[0]) {
        moves.push(
            Move {
                dir: Dir::Up,
                result_pos: Coord { x: coord.x, y: coord.y - 1 }
            });
    }

    // down
    if coord.y < (MAZE_HEIGHT - 1) && is_open(lock_chars[1]) {
        moves.push(
            Move {
                dir: Dir::Down,
                result_pos: Coord { x: coord.x, y: coord.y + 1 }
            });
    }

    // left
    if coord.x > 0 && is_open(lock_chars[2]) {
        moves.push(
            Move {
                dir: Dir::Left,
                result_pos: Coord { x: coord.x - 1, y: coord.y }
            });
    }

    // right
    if coord.x < (MAZE_WIDTH - 1) && is_open(lock_chars[3]) {
        moves.push(
            Move {
                dir: Dir::Right,
                result_pos: Coord { x: coord.x + 1, y: coord.y }
            });
    }

    moves
}

fn search_maze(passwd: &str) -> Option<String> {
    const START_POS: Coord = Coord { x: 0, y: 0 };
    const END_POS: Coord = Coord { x: 3, y: 3 };

    let mut trails = LinkedList::new();
    trails.push_back(
        Trail {
            path: vec![],
            end: START_POS,
        });

    loop {
        if trails.is_empty() {
            return None
        }

        let cur_trail = trails.pop_front().unwrap();
        if cur_trail.end == END_POS {
            return Some(str::from_utf8(&cur_trail.path).unwrap().to_string());
        }

        let next_moves = find_valid_moves(
            passwd.as_bytes(), &cur_trail.path, cur_trail.end);
        // println!("next_moves: {:?}", next_moves);

        for next_move in next_moves {
            let mut next_path = cur_trail.path.clone();
            next_path.push(next_move.dir.to_byte());
            trails.push_back(
                Trail {
                    path: next_path,
                    end: next_move.result_pos,
                });
        }
    }
}

fn do_tests() {
    let test_md5 = calc_md5("abcdefghijklmnopqrstuvwxyz".as_bytes());
    assert!(md5_bytes_as_str(&test_md5) == "c3fcd3d76192e4007dfb496cca67e13b");

    let result = find_valid_moves(
        "hijkl".as_bytes(), "".as_bytes(), Coord { x: 0, y: 0 });

    let down = Move {
        dir: Dir::Down,
        result_pos: Coord { x: 0, y: 1 }
    };
    assert_eq!(result, vec![down]);

    let result = find_valid_moves(
        "hijkl".as_bytes(), "D".as_bytes(), Coord { x: 0, y: 1 });

    let up = Move {
        dir: Dir::Up,
        result_pos: Coord { x: 0, y: 0 }
    };
    let right = Move {
        dir: Dir::Right,
        result_pos: Coord { x: 1, y: 1 }
    };
    assert_eq!(result, vec![up, right]);

    let result = search_maze("hijkl");
    assert_eq!(result, None);

    let result = search_maze("ihgpwlah");
    assert_eq!(result, Some("DDRRRD".to_string()));

    let result = search_maze("kglvqrro");
    assert_eq!(result, Some("DDUDRLRRUDRD".to_string()));

    let result = search_maze("ulqzkmiv");
    assert_eq!(result, Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string()));
}

fn main() {
    const INPUT: &str = "pslxynzg";

    do_tests();

    match search_maze(INPUT) {
        None => println!("no solution"),
        Some(path) => println!("path: {}", path),
    };
}
