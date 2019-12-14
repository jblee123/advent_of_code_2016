use std::cmp;
// use std::collections::HashSet;
// use std::collections::LinkedList;
// use std::fs;
// use std::result::Result;
use std::str;
// use std::str::FromStr;

fn str_to_vec(s: &str) -> Vec<u8> {
    s.chars()
        .map(|c| {
            match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("bad char"),
            }
        })
        .collect()
}

fn vec_to_str(v: &Vec<u8>) -> String {
    let chars = v.iter()
        .map(|num| {
            match num {
                0 => b'0',
                1 => b'1',
                _ => panic!("bad num: {}", num),
            }
        })
        .collect::<Vec<u8>>();

    str::from_utf8(&chars).unwrap().to_string()
}

fn expand_data(data: &mut Vec<u8>, max_len: usize) {
    if data.len() >= max_len {
        return;
    }

    let orig_len = data.len();
    data.push(0);

    let mut i = orig_len - 1;
    while data.len() < max_len {
        data.push(data[i] ^ 1);

        if i == 0 {
            break;
        }

        i -= 1;
    }
}

fn generate_data(data: &Vec<u8>, amount: usize) -> Vec<u8> {
    let mut new_data = Vec::with_capacity(amount);

    let amt_to_copy = cmp::min(amount, data.len());
    new_data.extend_from_slice(&data[0..amt_to_copy]);

    while new_data.len() < amount {
        expand_data(&mut new_data, amount);
    }

    new_data
}

fn checksum_data(data: &Vec<u8>) -> Vec<u8> {
    if data.len() % 2 == 1 {
        panic!("checksum data len is odd");
    }

    let mut chksum = Vec::with_capacity(data.len() / 2);

    let mut i = 0;
    while i < data.len() {
        chksum.push(data[i] ^ data[i + 1] ^ 1);
        i += 2;
    }

    if chksum.len() % 2 == 0 {
        chksum = checksum_data(&chksum);
    }

    chksum
}

fn do_tests() {
    let result = str_to_vec("001101");
    assert_eq!(&result, &vec![0, 0, 1, 1, 0, 1]);

    let result = vec_to_str(&vec![0, 0, 1, 1, 0, 1]);
    assert_eq!(&result, "001101");

    let result = generate_data(&vec![1], 3);
    assert_eq!(&result, &vec![1, 0, 0]);

    let result = generate_data(&vec![1], 2);
    assert_eq!(&result, &vec![1, 0]);

    let result = generate_data(&vec![1], 1);
    assert_eq!(&result, &vec![1]);

    let result = generate_data(&vec![0], 3);
    assert_eq!(&result, &vec![0, 0, 1]);

    let result = generate_data(&str_to_vec("111100001010"), 25);
    assert_eq!(&result, &str_to_vec("1111000010100101011110000"));

    let result = checksum_data(&str_to_vec("110010110100"));
    assert_eq!(&result, &str_to_vec("100"));
}

fn main() {
    const DISK_LEN: usize = 35651584;
    const INPUT: &str = "01000100010010111";

    let data = generate_data(&str_to_vec(INPUT), DISK_LEN);
    let checksum = checksum_data(&data);

    println!("checksum: {}", vec_to_str(&checksum));

    do_tests();
}
