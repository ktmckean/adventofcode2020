#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::collections::HashSet;
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
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day6\\src\\input.txt");

    let mut sumOfNumAnswers = 0;
    let mut answeredQs = HashMap::<char,i32>::new();
    let mut groupSize:i32 = 0;

    for line in v{
        if !line.is_empty(){
            groupSize+= 1;
        }

        if answeredQs.is_empty(){
            for c in line.chars(){
                answeredQs.insert(c,1);
            }
        }
        else {
            for c in line.chars(){
                if !answeredQs.contains_key(&c){
                    continue;
                } else{
                *answeredQs.get_mut(&c).unwrap() += 1;
                }
            } 
        }

        if line.is_empty(){
            let mut numqs = 0;
            for c in &answeredQs{
                if c.1 == &groupSize{
                    numqs+=1;
                }
            }
            answeredQs.clear();
            groupSize = 0;
            sumOfNumAnswers += numqs;
        }
    }

    if !answeredQs.is_empty() && groupSize != 0{
        let mut numqs = 0;
        for c in &answeredQs{
            if answeredQs[&c.0] == groupSize{
                numqs+=1;
            }
        }
        answeredQs.clear();
        groupSize = 0;
        sumOfNumAnswers += numqs;
    }

    println!("{}",sumOfNumAnswers);
}

// fn getSharedAnswers(group : Vec::<HashSet<char>>) -> i32{
//     let mut shared = HashMap::<char,i32>::new();
//     for person in group{
//         for c in person{
//             if shared.contains_key(&c){
//                 shared.entry(c).or_insert(0) += 1;
//             } else{
//                 shared[&c] = 1;
//             }
//         }
//     }
//     return 0;
// }

fn doPart1(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day6\\src\\input.txt");

    let mut sumOfNumAnswers = 0;
    let mut answeredQs = HashSet::new();
    for line in v{
        for c in line.chars(){
            answeredQs.insert(c);
        }

        if line.is_empty(){
            sumOfNumAnswers += answeredQs.len();
            answeredQs.clear();
        }
    }

    if !answeredQs.is_empty(){
        sumOfNumAnswers += answeredQs.len();
    }

    println!("{}",sumOfNumAnswers);
}