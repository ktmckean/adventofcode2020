#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
// use std::collections::HashSet;
use std::collections::HashMap;

fn readLines(path :&str) -> std::vec::Vec<String>{
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
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
    // doPartOne();

    let _v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day15\\src\\input.txt");

    let turns = [0,1,4,13,15,12,16].to_vec();
    // let turns :Vec<usize> = [0,3,6].to_vec();
    // let turns = [1,3,2].to_vec();

    let mut spoken = HashMap::<usize, (usize, usize)>::new();
    
    let mut i = 1;
    for num in &turns{
        addAndUpdate(&mut spoken, *num, i);
        i += 1;
    }



    let mut lastTurn = turns[turns.len()-1];


    // while i < 2021  {
    //     let mut turn = 0;

    //     if spoken[&lastTurn].1 != 0{
    //         turn = spoken[&lastTurn].0 - spoken[&lastTurn].1;
    //         // println!("Found previous turn of {}: changing turn to {}", lastTurn, turn);
    //     }
    //     addAndUpdate(&mut spoken, turn, i);

    //     lastTurn = turn;
    //     i+=1;
    // }

    // assert!(2018 != lastTurn);
    // assert!(1665 == lastTurn);
    // println!("Part 1: {}",lastTurn);


    let mut percentDone = 5;
    while i < 30000001  {
        let mut turn = 0;

        if spoken[&lastTurn].1 != 0{
            turn = spoken[&lastTurn].0 - spoken[&lastTurn].1;
            // println!("Found previous turn of {}: changing turn to {}", lastTurn, turn);
        }
        addAndUpdate(&mut spoken, turn, i);

        lastTurn = turn;
        if i % 1500000 == 0{
            println!("{}% done...", percentDone);
            percentDone += 5;
        }
        i+=1;
    }

    println!("Part 2: {}",lastTurn);
}

fn addAndUpdate(map :&mut HashMap<usize, (usize, usize)>, key: usize, turn: usize){
    let mut lastTimeSpoken = 0;
    if map.contains_key(&key){
        lastTimeSpoken = map[&key].0;
    }

    // println!("Adding {}: ({}, {})", key, turn, lastTimeSpoken);
    map.insert(key, (turn, lastTimeSpoken));
}

