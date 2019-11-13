use std::fs;
use std::str;
use std::str::FromStr;

fn get_usize(data: &[u8], start: usize, end: usize) -> usize {
    usize::from_str(str::from_utf8(&data[start..end]).unwrap()).unwrap()
}

// returns chunk len, times repeated, and index after
fn parse_decomp_marker(data: &[u8], open_paren: usize) -> (usize, usize, usize) {
    let start_chunk_len = open_paren + 1;
    let (x_idx, _) = data[start_chunk_len..].iter().enumerate()
        .find(|&(_, &b)| b == b'x').unwrap();
    let x_idx = x_idx + start_chunk_len;
    let start_times_repeated = x_idx + 1;

    let (close_paren, _) = data[start_times_repeated..].iter().enumerate()
        .find(|&(_, &b)| b == b')').unwrap();
    let close_paren = close_paren + start_times_repeated;

    let chunk_len = get_usize(data, start_chunk_len, x_idx);
    let times_repeated = get_usize(data, start_times_repeated, close_paren);

    (chunk_len, times_repeated, close_paren + 1)
}

fn get_decompressed_len(data: &[u8], start: usize, end: usize) -> usize {
    let marker_pos = data[start..end].iter().enumerate()
        .find(|&(_, &b)| b == b'(');

    match marker_pos {
        Some((open_paren, _)) => {
            let open_paren = open_paren + start;
            let (chunk_len, times_repeated, new_start) =
                parse_decomp_marker(data, open_paren);

            let chunk_end = new_start + chunk_len;
            let len = (open_paren - start)
                + get_decompressed_len(data, new_start, chunk_end) * times_repeated
                + get_decompressed_len(data, chunk_end, end);

            len
        },
        None => end - start,
    }
}

fn main() {
    let input = fs::read("input.txt")
        .expect("Something went wrong reading the file");

    let input_len = input.len();
    let num_decompressed_bytes = get_decompressed_len(&input, 0, input_len);

    println!("num decompressed bytes: {}", num_decompressed_bytes);
}
