use std::{fs, collections::HashSet};

fn main() {
    let contents = fs::read_to_string("inputs/3.txt").expect("Something went wrong reading the file");
    day3_pt2(&contents);
}

fn day3_pt2(contents: &str) {
    let mut iter = contents.lines();
    let mut points = 0;
    while let (Some(a), Some(b), Some(c)) = (iter.next(), iter.next(), iter.next()) {
        let set_a = HashSet::<char>::from_iter(a.chars());
        let set_b = HashSet::<char>::from_iter(b.chars());
        let set_c = HashSet::<char>::from_iter(c.chars());
        let mut common = set_a.intersection(&set_b).collect::<HashSet<_>>();
        common.retain(|e| set_c.contains(e));
        let common = *(common.iter().next().unwrap());
        if common.is_uppercase() {
            points += (*common as i32) - 64 + 26;
        } else {
            points += *common as i32 - 96;
        }
        
    }
    println!("{}",points);
}

fn day3_pt1(contents: &str) {
    let mut iter = contents.lines();
    let mut points = 0;
    while let Some(line) = iter.next() {
        let len = line.len() /2;
        let first = &line[..len];
        let last = &line[len..];
        points += calc_points(first, last)
        
    }
    println!("{}",points);
}

fn calc_points (first: &str, last: &str) -> i32 {
    for char1 in first.chars() {
        for char2 in last.chars() {
            if char1 == char2 {
                if char1.is_uppercase() {
                    return 26 + (char1 as i32 - 64);
                } else {
                    return char1 as i32 - 96;
                }
            }
        }
    }
    0
}

#[test]
fn test() {
   let contents = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    day3_pt1(contents);

}