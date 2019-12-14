// use std::collections::HashSet;
// use std::collections::LinkedList;
use std::fs;
// use std::result::Result;
// use std::str;
use std::str::FromStr;

struct Disc {
    disc_num: u32,
    num_positions: u32,
    state: u32,
    state_mask: u32,
}

impl Disc {
    pub fn from_str(s: &str) -> Disc {
        let tokens = s.split(" ").collect::<Vec<&str>>();

        let disc_num = u32::from_str(&tokens[1][1..]).unwrap();
        let num_positions = u32::from_str(tokens[3]).unwrap();

        let time0_pos_period_idx = tokens[11].len() - 1;
        let time0_pos_str = &tokens[11][..time0_pos_period_idx];
        let time0_pos = u32::from_str(time0_pos_str).unwrap();

        let time0_state = 1 << time0_pos;
        let state_mask = (u32::max_value() << num_positions) ^ u32::max_value();

        Disc {
            disc_num: disc_num,
            num_positions: num_positions,
            state: time0_state,
            state_mask: state_mask,
        }
    }

    pub fn inc(&mut self) {
        self.state = (self.state << 1) & self.state_mask;
        if self.state == 0 {
            self.state = 1;
        }
    }

    pub fn inc_times(&mut self, cnt: u32) {
        for _ in 0..cnt {
            self.inc();
        }
    }
}

fn parse_input(input: &str) -> Vec<Disc> {
    input.lines().map(|line| Disc::from_str(line)).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut test_disc = Disc::from_str(
        "Disc #3 has 5 positions; at time=0, it is at position 4.");
    assert_eq!(test_disc.disc_num, 3);
    assert_eq!(test_disc.num_positions, 5);
    assert_eq!(test_disc.state, 0b00000000000000000000000000010000);
    assert_eq!(test_disc.state_mask, 0b00000000000000000000000000011111);

    test_disc.inc();
    assert_eq!(test_disc.state, 0b00000000000000000000000000000001);
    test_disc.inc();
    assert_eq!(test_disc.state, 0b00000000000000000000000000000010);

    let mut discs = parse_input(&input);
    println!("{} discs", discs.len());

    // Fast-forward each disc to account for fall time.
    discs.iter_mut().for_each(|disc| disc.inc_times(disc.disc_num));

    let mut time = 0;
    while discs.iter().any(|disc| disc.state != 1) {
        discs.iter_mut().for_each(|disc| disc.inc());
        time += 1;
    }

    println!("drop time: {}", time);
}
