use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    const NUM_COLS: usize = 8;
    const NUM_LETTER: usize = 26;
    let mut chars = [[0u8; NUM_LETTER]; NUM_COLS];

    let lines = input.lines();

    for line in lines {
        let mut col_idx = 0;
        for c in line.as_bytes() {
            let char_idx = *c - ('a' as u8);
            chars[col_idx][char_idx as usize] += 1;
            col_idx += 1;
        }
    }

    let mut decoded_chars = [0u8; NUM_COLS];
    let mut decoded_char_idx = 0;
    for col in &chars {
        let mut col_char_idx = 0;
        let mut min_char_idx = 0;
        let mut min_char_cnt = 255;
        for char_cnt in col {
            if *char_cnt > 0 && *char_cnt < min_char_cnt {
                min_char_idx = col_char_idx;
                min_char_cnt = *char_cnt;
            }
            col_char_idx += 1;
        }
        decoded_chars[decoded_char_idx] = ('a' as u8) + min_char_idx;
        decoded_char_idx += 1;
    }

    println!("msg: {}", std::str::from_utf8(&decoded_chars).unwrap());
}
