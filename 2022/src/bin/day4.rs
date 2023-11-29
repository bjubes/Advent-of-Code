use std::{fs, collections::HashSet};

fn main() {
    let contents = fs::read_to_string("inputs/4.txt").expect("Something went wrong reading the file");
    day4_pt2(&contents);
}

fn day4_pt1(contents: &str) {
    let mut iter = contents.lines();
    let mut ans = 0;
    while let Some(line) = iter.next() {
        let nums:Vec<i32>=line.split(&['-',',']).map(|x| x.parse::<i32>().unwrap()).collect();
       if (nums[0] <= nums[2] && nums[1] >= nums[3]) || (nums[0] > nums[2] && nums[1] < nums[3]) {
        ans+=1;
       }
    }
    println!("{}",ans);
}

fn day4_pt2(contents: &str) {
    let mut iter = contents.lines();
    let mut ans = 0;
    while let Some(line) = iter.next() {
        let nums: Vec<i32> = line
            .split(&['-', ','])
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        if !((nums[1] < nums[2]) || (nums[0] > nums[3]  )){
            ans += 1;
        } 
    }
    println!("{}", ans);
}