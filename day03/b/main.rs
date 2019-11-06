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
    let mut side_num = 0;
    let mut tri1: [i32; 3] = [0; 3];
    let mut tri2: [i32; 3] = [0; 3];
    let mut tri3: [i32; 3] = [0; 3];
    for line in lines {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        assert_eq!(tokens.len(), 3);

        tri1[side_num] = i32::from_str(&tokens[0]).unwrap();
        tri2[side_num] = i32::from_str(&tokens[1]).unwrap();
        tri3[side_num] = i32::from_str(&tokens[2]).unwrap();

        if side_num == 2 {
            if is_legal_triangle(tri1[0], tri1[1], tri1[2]) {
                num_possible += 1;
            }
            if is_legal_triangle(tri2[0], tri2[1], tri2[2]) {
                num_possible += 1;
            }
            if is_legal_triangle(tri3[0], tri3[1], tri3[2]) {
                num_possible += 1;
            }

            side_num = 0;
        } else {
            side_num += 1;
        }
    }

    println!("num possible triangles: {}", num_possible);
}
