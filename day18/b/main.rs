// use std::collections::HashSet;
// use std::collections::LinkedList;
use std::fs;
// use std::result::Result;
// use std::str;
// use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile { Safe, Trap, }

// #[derive(PartialEq, Eq, Debug)]
type Row = Vec<Tile>;

fn str_to_row(s: &str) -> Row {
    s.chars()
        .map(|c| {
            match c {
                '.' => Tile::Safe,
                '^' => Tile::Trap,
                _ => panic!("bad char in row str: {}", c),
            }
        })
        .collect()
}

fn get_affecting_tiles(row: &Row, idx: usize) -> u8 {
    if idx >= row.len() {
        panic!("get_affecting_tiles: bad idx ({}) >= row length ({})",
            idx, row.len());
    }

    let mut tiles = 0;

    if idx != 0 && row[idx - 1] == Tile::Trap {
        tiles |= 1 << 2;
    }

    if row[idx] == Tile::Trap {
        tiles |= 1 << 1;
    }

    if idx != row.len() - 1 && row[idx + 1] == Tile::Trap {
        tiles |= 1 << 0;
    }

    tiles
}

fn get_next_row_tile(affecting_tiles: u8) -> Tile {
    match affecting_tiles {
        0b110 |
        0b011 |
        0b100 |
        0b001 => Tile::Trap,
        _ => Tile::Safe,
    }
}

fn generate_next_row(row: &Row) -> Row {
    (0..row.len())
        .map(|n| get_affecting_tiles(row, n))
        .map(|tiles| get_next_row_tile(tiles))
        .collect()
}

fn count_tile_type(row: &Row, tile_type: Tile) -> u32 {
    row.iter().filter(|tile| **tile == tile_type).count() as u32
}

fn do_tests() {
    let test_row1 = str_to_row("^^..^.");
    assert_eq!(test_row1, vec![
        Tile::Trap, Tile::Trap, Tile::Safe,
        Tile::Safe, Tile::Trap, Tile::Safe,
    ]);

    let result = get_affecting_tiles(&test_row1, 0);
    assert_eq!(result, 0b011);

    let result = get_affecting_tiles(&test_row1, 2);
    assert_eq!(result, 0b100);

    let result = get_affecting_tiles(&test_row1, 5);
    assert_eq!(result, 0b100);

    assert_eq!(get_next_row_tile(0b110), Tile::Trap);
    assert_eq!(get_next_row_tile(0b011), Tile::Trap);
    assert_eq!(get_next_row_tile(0b100), Tile::Trap);
    assert_eq!(get_next_row_tile(0b001), Tile::Trap);
    assert_eq!(get_next_row_tile(0b000), Tile::Safe);
    assert_eq!(get_next_row_tile(0b111), Tile::Safe);
    assert_eq!(get_next_row_tile(0b010), Tile::Safe);
    assert_eq!(get_next_row_tile(0b101), Tile::Safe);

    let test_row2 = str_to_row(".^^.^.^^^^");
    let result = generate_next_row(&test_row2);
    assert_eq!(result, str_to_row("^^^...^..^"));

    let result = (0..9)
        .fold(test_row2.clone(), |acc, _| generate_next_row(&acc));
    assert_eq!(result, str_to_row("^^.^^^..^^"));

    let result = count_tile_type(&test_row2, Tile::Safe);
    assert_eq!(result, 3);
    let result = count_tile_type(&test_row2, Tile::Trap);
    assert_eq!(result, 7);
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    const ROW_COUNT: usize = 400000;

    do_tests();

    let mut row = str_to_row(input.trim());
    let mut safe_tiles = count_tile_type(&row, Tile::Safe);

    count_tile_type(&row, Tile::Trap);
    (0..(ROW_COUNT - 1)).for_each(|_| {
        row = generate_next_row(&row);
        safe_tiles += count_tile_type(&row, Tile::Safe);
    });

    println!("safe tiles: {}", safe_tiles);
}
