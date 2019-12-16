// use std::collections::HashSet;
// use std::collections::LinkedList;
use std::fs;
// use std::result::Result;
// use std::str;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct SwapPosInstr {
    pos1: u32,
    pos2: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct SwapLtrInstr {
    letter1: u8,
    letter2: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct RotateLeftInstr {
    count: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct RotateRightInstr {
    count: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct RotateLtrInstr {
    letter: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct ReverseInstr {
    pos1: u32,
    pos2: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct MoveInstr {
    pos1: u32,
    pos2: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instr {
    SwapPos(SwapPosInstr),
    SwapLtr(SwapLtrInstr),
    RotateLeft(RotateLeftInstr),
    RotateRight(RotateRightInstr),
    RotateLtr(RotateLtrInstr),
    Reverse(ReverseInstr),
    Move(MoveInstr),
}

fn parse_instruction(instr_str: &str) -> Instr {
    let parts: Vec<&str> = instr_str.split(" ").collect();

    let err = || { panic!("bad instr: {}", instr_str); };

    if parts.is_empty() {
        panic!("NO PARTS!");
    }

    match parts[0] {
        "swap" => match parts[1] {
            "position" => {
                return Instr::SwapPos(
                    SwapPosInstr {
                        pos1: u32::from_str(parts[2]).unwrap(),
                        pos2: u32::from_str(parts[5]).unwrap(),
                    });
            },
            "letter" => {
                return Instr::SwapLtr(
                    SwapLtrInstr {
                        letter1: parts[2].as_bytes()[0],
                        letter2: parts[5].as_bytes()[0],
                    });
            },
            _ => err(),
        },
        "rotate" => match parts[1] {
            "left" => {
                return Instr::RotateLeft(
                    RotateLeftInstr {
                        count: u32::from_str(parts[2]).unwrap(),
                    });
            },
            "right" => {
                return Instr::RotateRight(
                    RotateRightInstr {
                        count: u32::from_str(parts[2]).unwrap(),
                    });
            },
            "based" => {
                return Instr::RotateLtr(
                    RotateLtrInstr {
                        letter: parts[6].as_bytes()[0],
                    });
            },
            _ => err(),
        }
        "reverse" => {
            return Instr::Reverse(
                ReverseInstr {
                    pos1: u32::from_str(parts[2]).unwrap(),
                    pos2: u32::from_str(parts[4]).unwrap(),
                });
        },
        "move" => {
            return Instr::Move(
                MoveInstr {
                    pos1: u32::from_str(parts[2]).unwrap(),
                    pos2: u32::from_str(parts[5]).unwrap(),
                });
        },
        _ => err(),
    }


    panic!("bad instr: {}", instr_str);
}

fn do_swap_pos(s: &str, instr: &SwapPosInstr) -> String {
    let s_bytes = s.as_bytes();
    let mut vec: Vec<u8> = Vec::new();
    vec.extend(s_bytes);
    vec[instr.pos1 as usize] = s_bytes[instr.pos2 as usize];
    vec[instr.pos2 as usize] = s_bytes[instr.pos1 as usize];
    String::from_utf8(vec).unwrap()
}

fn do_swap_ltr(s: &str, instr: &SwapLtrInstr) -> String {
    let s_bytes = s.as_bytes();
    let mut vec: Vec<u8> = Vec::new();
    vec.extend(s_bytes);
    let pos1 = vec.iter().position(|&b| b == instr.letter1).unwrap();
    let pos2 = vec.iter().position(|&b| b == instr.letter2).unwrap();
    vec[pos1 as usize] = s_bytes[pos2 as usize];
    vec[pos2 as usize] = s_bytes[pos1 as usize];
    String::from_utf8(vec).unwrap()
}

fn do_rotate_left(s: &str, instr: &RotateLeftInstr) -> String {
    // P2 FIX: this is the "rotate right" code
    let s_bytes = s.as_bytes();
    let mut vec: Vec<u8> = Vec::new();
    let rot_amt = instr.count as usize % s.len();
    let bdry = s.len() - rot_amt;
    if rot_amt > 0 {
        vec.extend(&s_bytes[bdry..]);
    }
    vec.extend(&s_bytes[..bdry]);
    String::from_utf8(vec).unwrap()
}

fn do_rotate_right(s: &str, instr: &RotateRightInstr) -> String {
    // P2 FIX: this is the "rotate left" code
    let s_bytes = s.as_bytes();
    let mut vec: Vec<u8> = Vec::new();
    let rot_amt = instr.count as usize % s.len();
    vec.extend(&s_bytes[rot_amt..]);
    if rot_amt > 0 {
        vec.extend(&s_bytes[..rot_amt]);
    }
    String::from_utf8(vec).unwrap()
}

fn do_rotate_ltr(s: &str, instr: &RotateLtrInstr) -> String {
    // let idx = s.as_bytes().iter().position(|&b| b == instr.letter).unwrap();
    // let rot_amt = 1 + idx + (if idx >= 4 { 1 } else { 0 });
    // do_rotate_right(s, &RotateRightInstr { count: rot_amt as u32 })

    // P2 FIX
    // For 8-char strings, the following maps the starting and ending positions
    // of the specified letter and the right rotate amount:
    //   0 -> 1 -> 1
    //   1 -> 3 -> 2
    //   2 -> 5 -> 3
    //   3 -> 7 -> 4
    //   4 -> 2 -> 6
    //   5 -> 4 -> 7
    //   6 -> 6 -> 0
    //   7 -> 0 -> 1
    // So now, after we find where the letter is in the string, we can use the
    // above mapping to left-shift back to the original position.
    let idx = s.as_bytes().iter().position(|&b| b == instr.letter).unwrap();
    let left_shift_amt = match idx {
        1 => 1,
        3 => 2,
        5 => 3,
        7 => 4,
        2 => 6,
        4 => 7,
        6 => 0,
        0 => 1,
        _ => panic!("AAAAHHHH! BAD INDEX!"),
    };
    // P2 FIX: And still calling right rotate here because the function
    // currently is rotating left.
    do_rotate_right(s, &RotateRightInstr { count: left_shift_amt as u32 })
}

fn do_reverse(s: &str, instr: &ReverseInstr) -> String {
    let s_bytes = s.as_bytes();
    let mut vec: Vec<u8> = Vec::new();
    vec.extend(s_bytes);

    let mut pos1 = instr.pos1 as usize;
    let mut pos2 = instr.pos2 as usize;
    while pos1 < pos2 {
        vec.swap(pos1, pos2);
        pos1 += 1;
        pos2 -= 1;
    }
    String::from_utf8(vec).unwrap()
}

fn do_move(s: &str, instr: &MoveInstr) -> String {
    let s_bytes = s.as_bytes();
    let mut vec: Vec<u8> = Vec::new();
    vec.extend(s_bytes);

    // P2 FIX: swapped pos1 and pos2
    let removed = vec.remove(instr.pos2 as usize);
    vec.insert(instr.pos1 as usize, removed);

    String::from_utf8(vec).unwrap()
}

fn proc_instr(s: &str, instr: &Instr) -> String {
    match instr {
        Instr::SwapPos(op) => do_swap_pos(s, op),
        Instr::SwapLtr(op) => do_swap_ltr(s, op),
        Instr::RotateLeft(op) => do_rotate_left(s, op),
        Instr::RotateRight(op) => do_rotate_right(s, op),
        Instr::RotateLtr(op) => do_rotate_ltr(s, op),
        Instr::Reverse(op) => do_reverse(s, op),
        Instr::Move(op) => do_move(s, op),
    }
}

// fn do_tests() {
//     let result = parse_instruction(&"swap position 4 with position 0");
//     assert_eq!(result, Instr::SwapPos(SwapPosInstr { pos1: 4, pos2: 0, }));

//     let result = parse_instruction(&"swap letter d with letter b");
//     assert_eq!(result,
//         Instr::SwapLtr(SwapLtrInstr { letter1: b'd', letter2: b'b', }));

//     let result = parse_instruction(&"rotate left 1 step");
//     assert_eq!(result, Instr::RotateLeft(RotateLeftInstr { count: 1, }));

//     let result = parse_instruction(&"rotate right 3 steps");
//     assert_eq!(result, Instr::RotateRight(RotateRightInstr { count: 3, }));

//     let result = parse_instruction(&"rotate based on position of letter b");
//     assert_eq!(result, Instr::RotateLtr(RotateLtrInstr { letter: b'b', }));

//     let result = parse_instruction(&"reverse positions 0 through 4");
//     assert_eq!(result, Instr::Reverse(ReverseInstr { pos1: 0, pos2: 4, }));

//     let result = parse_instruction(&"move position 3 to position 0");
//     assert_eq!(result, Instr::Move(MoveInstr { pos1: 3, pos2: 0, }));

//     let test_text1 = "abcde".to_string();

//     let instr = parse_instruction(&"swap position 4 with position 0");
//     let test_text1 = proc_instr(&test_text1, &instr);
//     assert_eq!(test_text1, "ebcda");

//     let instr = parse_instruction(&"swap letter d with letter b");
//     let test_text1 = proc_instr(&test_text1, &instr);
//     assert_eq!(test_text1, "edcba");

//     let instr = parse_instruction(&"reverse positions 0 through 4");
//     let test_text1 = proc_instr(&test_text1, &instr);
//     assert_eq!(test_text1, "abcde");

//     let instr = parse_instruction(&"rotate left 1 step");
//     let test_text1 = proc_instr(&test_text1, &instr);
//     assert_eq!(test_text1, "bcdea");

//     let instr = parse_instruction(&"move position 1 to position 4");
//     let test_text1 = proc_instr(&test_text1, &instr);
//     assert_eq!(test_text1, "bdeac");

//     let instr = parse_instruction(&"move position 3 to position 0");
//     let test_text1 = proc_instr(&test_text1, &instr);
//     assert_eq!(test_text1, "abdec");

//     let instr = parse_instruction(&"rotate based on position of letter b");
//     let test_text1 = proc_instr(&test_text1, &instr);
//     assert_eq!(test_text1, "ecabd");

//     let instr = parse_instruction(&"rotate based on position of letter d");
//     let test_text1 = proc_instr(&test_text1, &instr);
//     assert_eq!(test_text1, "decab");

//     let instr = parse_instruction(&"rotate right 1 step");
//     let test_text1 = proc_instr(&test_text1, &instr);
//     assert_eq!(test_text1, "bdeca");

//     let instr = parse_instruction(&"rotate right 8 steps");
//     let test_text1 = proc_instr("ebhfcadg", &instr);
//     assert_eq!(test_text1, "ebhfcadg");

//     let instr = parse_instruction(&"rotate left 8 steps");
//     let test_text1 = proc_instr("ebhfcadg", &instr);
//     assert_eq!(test_text1, "ebhfcadg");

// }

fn main() {
    // do_tests();

    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    const START_TEXT: &str = "fbgdceah";

    let result = input.lines().rev().fold(START_TEXT.to_string(), |acc, line| {
        let instr = parse_instruction(line);
        proc_instr(&acc, &instr)
    });

    println!("passwd: {}", result);
}
