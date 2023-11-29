use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/1.txt").expect("Something went wrong reading the file");
    let mut iter = contents.lines();

    let mut top_3 = vec![0,0,0];
    let mut curr = 0;
    while let Some(line) = iter.next() {
        if let Ok(num) = line.parse::<i32>() {
            curr += num;
            let lowest = min(&top_3);
            if curr > lowest{
                let pos = top_3.iter().position(|&x| x == lowest);
                match pos {
                    Some(x) => top_3[x] = curr,
                    None => println!("{:?} {:?}", top_3, curr),
                }
            }
        } else {
            curr = 0;
        }
    }
    println!("{:?}", top_3);
    let sum = top_3.iter().sum::<i32>();
    println!("{:?}", sum);
}


fn min(v: &Vec<i32>)  -> i32 {
    let mut min = std::i32::MAX;
    for i in 0..v.len() {
        if v[i] < min {
            min = v[i];
        }
    }
    min
}