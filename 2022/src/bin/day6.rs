use std::{fs};

fn main() {
    let contents = fs::read_to_string("inputs/5.txt").expect("Something went wrong reading the file");
    day6_pt2(&contents);
}
fn day6_pt2(contents: &str) {
    for i in 0..contents.len() {
        let end = i +13;
        let window = &contents[i..=end];
        if !has_dup(window.chars().collect()){
            println!("{} {}",i+14,window);
            return
        }
    }
}

fn day6_pt1(contents: &str) {
    for i in 0..contents.len() {
        let end = i +3;
        let window = &contents[i..=end];
        if !has_dup(window.chars().collect()){
            println!("{} {}",i+4,window);
            return
        }
    }
}

fn has_dup(slice: Vec<char>) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }
    false
}


#[test]
fn test() {
   let contents = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    day5_pt1(contents);
}