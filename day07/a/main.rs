use std::fs;

fn find_abba(s: &str) -> bool {
    let bytes = s.as_bytes();
    for i in 0..bytes.len() - 3 {
        if bytes[i] != bytes[i + 1]
            && bytes[i] == bytes[i + 3]
            && bytes[i + 1] == bytes[i + 2] {

            return true;
        }
    }

    false
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = input.lines();

    let mut num_supports_tls = 0;

    for line in lines {
        let parts = line.split(|c| c == '[' || c == ']');

        let mut found_abba_out = false;
        let mut found_abba_in = false;
        for (i, part) in parts.enumerate() {
            match i % 2 {
                0 => found_abba_out |= find_abba(part),
                1 => found_abba_in |= find_abba(part),
                _ => panic!("can't get here!"),
            }
        }

        if found_abba_out && !found_abba_in {
            num_supports_tls += 1;
        }
    }

    println!("num with TLS: {}", num_supports_tls);
}
