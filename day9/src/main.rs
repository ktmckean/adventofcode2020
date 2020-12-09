#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
// use std::collections::HashSet;
// use std::collections::HashMap;

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
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day9\\src\\input.txt");
    let mut nums = Vec::<i64>::new();
    for line in v{
        nums.push(line.parse::<i64>().unwrap());
    }

    // let preambleLen = 5;
    let preambleLen = 25;
    let p1 = doPart1(preambleLen, &nums);
    // let invDex:usize = p1.0;
    let invNum:i64 = p1.1;

    doPart2(invNum, &nums);
}

fn doPart1(preambleLen: usize, nums: &Vec<i64>) -> (usize, i64){
    let mut invDex:usize = 0;
    let mut invNum:i64 = 0;
    for (i,n) in nums.iter().enumerate(){
        if i < preambleLen{
            continue;
        }
        if !isPreambleSum(i, preambleLen, &nums){
            println!("Part 1 answer: {}", n);
            invNum = *n;
            invDex = i;
            break;
        }
        else{
            // println!("nope!");
        }
    }
    (invDex, invNum)
}

fn doPart2(invNum: i64, nums: &Vec<i64>) {
    let (s, e) = findRangeContainingSum(invNum, &nums);
    
    let mut min = std::i64::MAX;
    let mut max = 0;
    for n in &nums[s..e]{
        if n < &min{
            min = *n;
        }
        if n > &max{
            max = *n;
        }
    } 
    println!("final sum:{}", min+max);
}

fn findRangeContainingSum(target: i64, nums: &Vec<i64>) -> (usize,usize){
    let (mut s,mut e): (usize,usize) = (0,1);
    loop {
        let mut sum = 0;
        for j in s .. e {
            sum += nums[j];
        }
        if sum < target{
            e += 1;
        }
        else if sum > target {
            s += 1;
        }
        else{ // if sum == invNum
            break;
        }
    }
    (s,e)
}

// requires that i < preambleLen
fn isPreambleSum(i: usize, preambleLen: usize, nums: &Vec<i64>) -> bool{
    // println!("ind:{}", i);

    for a in 1..preambleLen+1{
        for b in a+1..preambleLen+1{
            if nums[i]==nums[i-a] + nums[i-b]{
                return true;
            }
        }
    }
    return false;
}

