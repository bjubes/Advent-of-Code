use nom::{
    branch::alt, bytes::complete::tag, character::complete, character::complete::anychar,
    multi::separated_list1, IResult,
};
use std::{fs, vec};

fn main() {
    let contents =
        fs::read_to_string("inputs/5.txt").expect("Something went wrong reading the file");
    day5(&contents);
}

#[derive(PartialEq, Debug)]
struct Move {
    count: u32,
    from: u32,
    to: u32,
}

fn day5(contents: &str) {
    let part_1 = false;
    let lines = contents.lines();
    let mut towers: Vec<Vec<char>> = vec![];
    for _ in 0..9 {
        towers.push(vec![]);
    }

    let mut parse_moves = false;
    for line in lines {
        if line.is_empty() {
            parse_moves = true;
            for tower in towers.iter_mut() {
                tower.reverse();
            }
            continue;
        }
        if !parse_moves {
            let (_, rows) = crate_stacks(line).unwrap();
            rows.into_iter().enumerate().for_each(|(i, crate_box)| {
                if let Some(ch) = crate_box {
                    towers[i].push(ch)
                }
            });
        }
        if parse_moves && part_1 {
            let (_, cur_move) = parse_move(line).unwrap();
            for _ in 0..cur_move.count {
                let c = towers[cur_move.from as usize - 1].pop();
                towers[cur_move.to as usize - 1].push(c.unwrap());
            }
        }
        if parse_moves && !part_1 {
            let (_, cur_move) = parse_move(line).unwrap();
            let from = cur_move.from as usize -1;
            let to= cur_move.to as usize -1;
            let count = cur_move.count as usize;
            let len = towers[from].len();

            let mut drained = towers[from].split_off(len-count);
            towers[to].append(&mut drained);
        }
    }
    // dbg!(towers);
    // println!("{}",ans);
    let mut ans = String::from("");
    for tower in &towers {
        ans = format!("{}{}", ans, tower.last().unwrap())
    }
    println!("{}", ans);
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, count) = complete::u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = complete::u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = complete::u32(input)?;
    Ok((input, Move { count, from, to }))
}

fn crate_box(input: &str) -> IResult<&str, Option<char>> {
    let (input, _) = tag("[")(input)?;
    let (input, ch) = anychar(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, Some(ch)))
}

fn no_box(input: &str) -> IResult<&str, Option<char>> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, None))
}

fn crate_stacks(input: &str) -> IResult<&str, Vec<Option<char>>> {
    let (input, stacks) = separated_list1(tag(" "), alt((crate_box, no_box)))(input)?;
    Ok((input, stacks))
}
#[test]
fn test_crate_stack() {
    let s = "move 7 from 3 to 9";
    assert_eq!(
        parse_move(s),
        Ok((
            "",
            Move {
                count: 7,
                from: 3,
                to: 9
            }
        ))
    );
}
#[test]
fn test_crate_stack() {
    let s = "[C] [B] [X]";
    assert_eq!(
        crate_stacks(s),
        Ok(("", vec![Some('C'), Some('B'), Some('X')]))
    );
}
#[test]
fn test_crate_stack_empty() {
    let s = "[C]     [X]";
    assert_eq!(crate_stacks(s), Ok(("", vec![Some('C'), None, Some('X')])));
}

#[test]
fn test_crate() {
    let s = "[C]";
    assert_eq!(crate_box(s), Ok(("", Some('C'))));
}

#[test]
fn test() {
    let contents = "    [D]    
    [N] [C]    
    [Z] [M] [P]
    1   2   3 

    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2";

    day5(contents);
}
