// use std::collections::HashSet;
// use std::collections::LinkedList;
use std::fs;
// use std::result::Result;
// use std::str;
use std::str::FromStr;

struct IpRange {
    lo: u32,
    hi: u32,
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut ip_ranges: Vec<IpRange> = input.lines()
        .map(|line| {
            let mut parts = line.trim().split("-");
            IpRange {
                lo: u32::from_str(parts.next().unwrap()).unwrap(),
                hi: u32::from_str(parts.next().unwrap()).unwrap(),
            }
        })
        .collect();

    ip_ranges.sort_by(|a, b| {
        if a.lo != b.lo {
            a.lo.partial_cmp(&b.lo).unwrap()
        } else {
            a.hi.partial_cmp(&b.hi).unwrap()
        }
    });

    let mut result = None;

    let mut candidate = 0;
    for ip_range in ip_ranges {
        if candidate < ip_range.lo {
            result = Some(candidate);
            break;
        }

        if ip_range.hi >= candidate {
            candidate = ip_range.hi.wrapping_add(1);
            if candidate == 0 {
                break;
            }
        }
    }

    match result {
        None => println!("no IP found"),
        Some(ip) => println!("got IP: {}", ip),
    }
}
