#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::collections::HashSet;
use std::collections::HashMap;
// use std::collections::LinkedList;
// use id_tree::*;

fn readLines(path :&str) -> std::vec::Vec<String>{
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            // Try the work computer's path
            println!("Error: {},\nTrying work path...",error);
            File::open("C:\\Repos\\adventofcode\\2020\\day25\\src\\input.txt").unwrap()
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
    doPartTwo();
}

fn doPartOne(){
    // let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day24\\src\\test.txt");
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day25\\src\\input.txt");

    let pub1 = v[0].parse::<i64>().unwrap();
    let pub2 = v[1].parse::<i64>().unwrap();

    let ls1 = getLoopSize(pub1);
    // let ls2 = getLoopSize(pub2);

    let key = transform(pub2, ls1);
   
    println!("Part 1: {}",key);
}

fn doPartTwo(){
    // let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day25\\src\\input.txt");

    // let mut tiles = flipTilesOnInput(&v);

    // println!("Day 0: {}", getNumBlack(&tiles));


    // for i in 0..100 {
    //     for t in getTilesToFlip(&tiles) {
    //         let state = tiles.get(&t).unwrap_or(&false);
    //         tiles.insert(t, !state);
    //     }
    //     println!("Day {}: {}", i+1, getNumBlack(&tiles));
    // }

    // // let mut numBlack = 0;
    // // for t in tiles {
    // //     if t.1 == true{
    // //         numBlack += 1;
    // //     }
    // // }


   
    // println!("Part 2: {}",getNumBlack(&tiles));
}

fn getLoopSize(pubKey: i64) -> usize{
    let mut val = 1;
    let mut ls = 0;

    while val != pubKey {
        val *= 7;
        val = val % 20201227;
        ls += 1;
    }
    ls
}

fn transform(subjectNum: i64, loopSize: usize) -> i64 {
    let mut val = 1;
    for _i in 0..loopSize{
        val *= subjectNum;
        val = val % 20201227;
    }
    val
}