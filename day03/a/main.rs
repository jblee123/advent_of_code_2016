use std::fs;
use std::str::FromStr;

fn is_legal_triangle(side1: i32, side2: i32, side3: i32) -> bool {
    (side1 + side2 > side3)
        && (side2 + side3 > side1)
        && (side1 + side3 > side2)
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = input.lines();

    let mut num_possible = 0;
    for line in lines {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        assert_eq!(tokens.len(), 3);

        let side1 = i32::from_str(&tokens[0]).unwrap();
        let side2 = i32::from_str(&tokens[1]).unwrap();
        let side3 = i32::from_str(&tokens[2]).unwrap();
        if is_legal_triangle(side1, side2, side3) {
            num_possible += 1;
        }
    }

    println!("num possible triangles: {}", num_possible);
}
