use std::fs;

#[derive(PartialEq, Eq, Debug)]
struct Aba {
    a: u8,
    b: u8,
}

fn get_abas_from_str(s: &str) -> Vec<Aba> {
    let mut abas: Vec<Aba> = vec![];
    let bytes = s.as_bytes();
    for i in 0..bytes.len() - 2 {
        if bytes[i] != bytes[i + 1]
            && bytes[i] == bytes[i + 2] {

            abas.push( Aba{ a: bytes[i], b: bytes[i + 1]});
        }
    }

    abas
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = input.lines();

    let mut num_supports_ssl = 0;

    for line in lines {
        let parts = line.split(|c| c == '[' || c == ']').collect::<Vec<&str>>();

        let outside_abas = parts.iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, p)| get_abas_from_str(p))
            .flatten()
            .collect::<Vec<Aba>>();

        let inside_parts = parts.iter()
            .enumerate()
            .filter(|&(i, _)| i % 2 == 1)
            .map(|(_, p)| get_abas_from_str(p))
            .flatten();

        let inside_abas = inside_parts
            .map(|aba| Aba { a: aba.b, b: aba.a })
            .collect::<Vec<Aba>>();

        for op in &outside_abas {
            if inside_abas.iter().any(|p| *p == *op) {
                num_supports_ssl += 1;
                break;
            }
        }
    }

    println!("num with SSL: {}", num_supports_ssl);
}
