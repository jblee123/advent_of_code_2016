// use std::collections::HashSet;
// use std::collections::LinkedList;
// use std::fs;
// use std::result::Result;
// use std::str;
// use std::str::FromStr;

type Elf = u32;

fn do_round(cur: &Vec<Elf>) -> Vec<Elf> {
    if cur.len() < 2 {
        return cur.clone();
    }

    let steal_idx = cur.len() / 2;
    let mut alive = vec![true; cur.len()];

    let mut steal_state = if cur.len() & 1 == 0 { 1 } else { 2 };

    let mut next_start_idx = 0;
    for i in steal_idx..cur.len() {
        let will_steal = steal_state != 0;
        if will_steal {
            next_start_idx += 1;
        }
        alive[i] = !will_steal;
        steal_state = (steal_state + 1) % 3;
    }

    let mut next = Vec::with_capacity(cur.len() - next_start_idx);
    (0..cur.len()).for_each(|i| {
        let new_idx = (next_start_idx + i) % cur.len();
        if alive[new_idx] {
            next.push(cur[new_idx])
        }
    });

    next
}

fn do_tests() {
    let result = do_round(&vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25]);
    assert_eq!(result,
        vec![10, 11, 12, 14, 17, 20, 23, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let result = do_round(&result);
    assert_eq!(result, vec![23, 1, 4, 7, 10, 11, 12, 14, 17, 20]);
}

fn main() {
    do_tests();

    const INPUT: u32 = 3014603;

    let mut elves: Vec<Elf> = (1..=INPUT).map(|id| id).collect();

    while elves.len() > 1 {
        elves = do_round(&elves);
    }

    if elves.len() == 0 {
        panic!("no elves in list!!");
    }

    println!("elf {} has all the presents", elves[0]);
}
