use std::fs;

fn main() {
    let contents = fs::read_to_string("inputs/2.txt").expect("Something went wrong reading the file");
    // let contents = "A X";

    let mut iter = contents.lines();
    let mut points = 0;
    while let Some(line) = iter.next() {
        let mut split = line.split(" ");
        points += get_points_pt_2((split.next().unwrap(),split.next().unwrap()));
    }
    println!("{}",points);
}

fn get_points_pt_2(hand: (&str,&str) )-> i32 {
    match hand {
        ("A","X") => 3,
        ("B","X") => 1,
        ("C","X") => 2,
        ("A","Y") => 3 + 1,
        ("B","Y") => 3 + 2,
        ("C","Y") => 3 + 3,
        ("A","Z") => 6 + 2,
        ("B","Z") => 6 + 3,
        ("C","Z") => 6 + 1,
        (_,_)=>0
    }
}

fn get_points_pt_1(hand: (&str,&str) )-> i32 {
    match hand {
        ("A","X") => 1+3,
        ("B","X") => 1,
        ("C","X") => 1 + 6,
        ("A","Y") => 2 + 6,
        ("B","Y") => 2 + 3,
        ("C","Y") => 2,
        ("A","Z") => 3,
        ("B","Z") => 3 + 6,
        ("C","Z") => 3 + 3,
        (_,_)=>0
    }
}