#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::collections::HashSet;
// use std::collections::HashMap;
use std::collections::LinkedList;
// use id_tree::*;

fn readLines(path :&str) -> std::vec::Vec<String>{
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            // Try the work computer's path
            println!("Error: {},\nTrying work path...",error);
            File::open("C:\\Repos\\adventofcode\\2020\\day23\\src\\input.txt").unwrap()
        }
    };
    let reader = BufReader::new(file);

    let mut v : Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        v.push(line);
    }
    v
}



fn main() {
    // let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day18\\src\\input.txt");
    doPartOne();
    // doPartTwo();
}

fn doPartOne(){
    // let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day23\\src\\test.txt");
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day23\\src\\input.txt");

    let input = &v[0];

    let mut cups = Vec::<u32>::new();
    for c in input.chars() {
        cups.push(c.to_digit(10).unwrap());
    }

    println!("{:?}",cups);

    // println!("{}", -1 % 5);
    let mut curInd = 0;
    for i in 0 .. 100 {
        curInd = takeTurn(&mut cups, curInd);
    }
    println!("{:?}",cups);

    let oneInd = getCupIndex(&cups, &1) +1 %cups.len();
    let mut sol = String::new();
    for c in &cups[oneInd ..] {
        sol.push_str(&c.to_string());
    }
    for c in &cups[.. oneInd] {
        sol.push_str(&c.to_string());
    }
    println!("Part 1: {}",sol);

}

fn takeTurn(cups :&mut Vec<u32>, curIndex: usize) -> usize{
    let curCup = cups[curIndex];

    print!("{:?}",cups);

    let mut removed = removeNextThree(cups, curIndex);

    print!(" {} --> ",curCup);
    print!("{:?}",removed);

    let destCup = getDestCup(curCup, &removed);
    print!(" {}\n",destCup);


    let dci = getCupIndex(&cups, &destCup);

    for c in removed.iter().rev(){
        cups.insert(dci+1, *c);
    }
    return (getCupIndex(&cups, &curCup) + 1) % cups.len();
}

fn removeNextThree(cups : &mut Vec<u32>, mut curIndex: usize) -> Vec<u32> {
    let mut removed = Vec::<u32>::new();
    removed.push(cups.remove((curIndex + 1) % cups.len()));
    if curIndex >= cups.len() { curIndex -= 1;  }
    removed.push(cups.remove((curIndex + 1) % cups.len()));
    if curIndex >= cups.len() { curIndex -= 1;  }
    removed.push(cups.remove((curIndex + 1) % cups.len()));

    return removed;
}

fn getCupIndex(cups: &Vec<u32>, cup: &u32) -> usize{
    for (i,c) in cups.iter().enumerate(){
        if c == cup{
            return i;
        }
    }
    unreachable!();
}

fn getDestCup(curCup: u32, removed: &Vec<u32>) -> u32{
    let mut destCup = curCup - 1;
    if destCup < 1 {
        destCup = 9;
    }
    while removed.contains(&destCup){
        // println!("{}",destCup);
        destCup -= 1;
        if destCup < 1 {
            destCup = 9;
        }
    }
    return destCup;
}