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

    let mut new_ranges = vec![];
    for ip_range in ip_ranges {
        if new_ranges.is_empty() {
            new_ranges.push(ip_range);
            continue;
        }

        let end = new_ranges.last_mut().unwrap();
        if end.hi == std::u32::MAX {
            break;
        } else if ip_range.lo > end.hi + 1 {
            new_ranges.push(ip_range);
        } else if ip_range.hi > end.hi {
            end.hi = ip_range.hi;
        }
    }

    let ip_ranges = new_ranges;

    let num_ips = (0..(ip_ranges.len() - 1))
        .map(|i| ip_ranges[i + 1].lo - ip_ranges[i].hi - 1)
        .sum::<u32>()
        + ip_ranges.first().unwrap().lo
        + (std::u32::MAX - ip_ranges.last().unwrap().hi);

    println!("num allowed IPs: {}", num_ips);
}
