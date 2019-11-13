use std::fs;
use std::str;
use std::str::FromStr;

fn get_decompressed_len(data: &[u8], start_idx: usize)
        -> (usize, usize) {

    let (mut x_idx, _) = data[(start_idx + 1)..].iter().enumerate()
        .find(|&(_, &b)| b == b'x').unwrap();
    x_idx += start_idx + 1;

    let (mut end_idx, _) = data[(x_idx + 1)..].iter().enumerate()
        .find(|&(_, &b)| b == b')').unwrap();
    end_idx += x_idx + 1;

    let data_len = usize::from_str(
        str::from_utf8(&data[(start_idx + 1)..x_idx]).unwrap()).unwrap();

    let times_repeated = usize::from_str(
        str::from_utf8(&data[(x_idx + 1)..end_idx]).unwrap()).unwrap();

    let consumed_len = (end_idx - start_idx + 1) + data_len;
    let decompressed_len = data_len * times_repeated;

    (consumed_len, decompressed_len)
}

fn main() {
    let input = fs::read("input.txt")
        .expect("Something went wrong reading the file");

    let input_len = input.len();
    let mut cur: usize = 0;
    let mut num_decompressed_bytes = 0;
    loop {
        let (num_consumed, decompr_chunk_len) = match input[cur] {
            b'(' => get_decompressed_len(&input, cur),
            _ => (1, 1),
        };

        cur += num_consumed;
        num_decompressed_bytes += decompr_chunk_len;

        if cur >= input_len {
            break;
        }
    }

    println!("num decompressed bytes: {}", num_decompressed_bytes);
}
