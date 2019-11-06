use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;

struct Room {
    name: String,
    sector: i32,
    checksum: String,
}

fn parse_line(line: &str) -> Room {
    let mut parts = line.split(|c| c == '[' || c == ']');
    let name_and_sector = parts.next().unwrap();
    let checksum = parts.next().unwrap();

    let mut front_parts = name_and_sector.rsplitn(2, "-");
    let sector_str = front_parts.next().unwrap();
    let name = front_parts.next().unwrap();
    let sector = sector_str.parse::<i32>().unwrap();

    Room {
        name: name.to_string(),
        sector: sector,
        checksum: checksum.to_string(),
    }
}

fn calc_checksum(room: &Room) -> String {
    struct CharAndCount {
        c: char,
        cnt: i32,
    }

    let mut char_map = HashMap::new();

    for c in room.name.chars() {
        if c == '-' {
            continue;
        }

        let mut key_exists = false;
        if let Some(cnt) = char_map.get_mut(&c) {
            *cnt += 1;
            key_exists = true;
        }

        if !key_exists {
            char_map.insert(c, 1);
        }
    }

    let mut char_vec = Vec::<CharAndCount>::with_capacity(char_map.len());
    for (c, cnt) in char_map {
        char_vec.push(CharAndCount{ c: c, cnt: cnt, });
    }

    char_vec.sort_by(|a, b| match a.cnt == b.cnt {
        false => b.cnt.cmp(&a.cnt),
        true => a.c.cmp(&b.c),
    });

    let checksum = char_vec.into_iter().map(|char_and_cnt| char_and_cnt.c)
        .take(5).collect::<Vec<char>>();
    String::from_iter(checksum)
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut sectors_sum = 0;

    let lines = input.lines();
    for line in lines {
        let room  = parse_line(line);
        let checksum = calc_checksum(&room);
        if checksum == room.checksum {
            sectors_sum += room.sector;
        }
    }

    println!("sector sum: {}", sectors_sum);
}
