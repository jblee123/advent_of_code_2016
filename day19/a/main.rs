// use std::collections::HashSet;
// use std::collections::LinkedList;
// use std::fs;
// use std::result::Result;
// use std::str;
// use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Elf {
    id: u32,
    num_presents: u32,
}

fn do_round(cur: &Vec<Elf>) -> Vec<Elf> {
    if cur.len() < 2 {
        return cur.clone();
    }

    let mut next = Vec::with_capacity(cur.len() / 2);

    if cur.len() & 1 == 0 {
        for (i, elf) in cur.iter().enumerate() {
            if i & 1 == 0 {
                next.push(*elf);
                next.last_mut().unwrap().num_presents +=
                    cur[i + 1].num_presents;
            }
        }
    } else {
        for (i, elf) in cur[2..cur.len()].iter().enumerate() {
            if i & 1 == 0 {
                next.push(*elf);
                next.last_mut().unwrap().num_presents +=
                    if i == cur.len() - 3 {
                        cur[0].num_presents + cur[1].num_presents
                    } else {
                        cur[i + 3].num_presents
                    };
            }
        }
    }

    next
}

fn do_tests() {
    let result = do_round(&vec![
        Elf { id: 1, num_presents: 1 },
        Elf { id: 2, num_presents: 1 },
        Elf { id: 3, num_presents: 1 },
        Elf { id: 4, num_presents: 1 },
        Elf { id: 5, num_presents: 1 },
    ]);
    assert_eq!(result, vec![
        Elf { id: 3, num_presents: 2 },
        Elf { id: 5, num_presents: 3 },
    ]);

    let result = do_round(&result);
    assert_eq!(result, vec![
        Elf { id: 3, num_presents: 5 },
    ]);
}

fn main() {
    do_tests();

    const INPUT: u32 = 3014603;

    let mut elves: Vec<Elf> = (1..=INPUT)
        .map(|id| Elf { id: id, num_presents: 1 }).collect();

    while elves.len() > 1 {
        elves = do_round(&elves);
    }

    if elves.len() == 0 {
        panic!("no elves in list!!");
    }

    println!("elf {} has {} presents", elves[0].id, elves[0].num_presents);
}
