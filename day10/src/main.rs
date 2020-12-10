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
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day10\\src\\input.txt");
    let mut nums = Vec::<i64>::new();
    for line in v{
        nums.push(line.parse::<i64>().unwrap());
    }
    nums.push(0);
    nums.sort();
    nums.push(nums[nums.len()-1]+3);
    // println!("{:?}",nums);

    let jolts :Vec::<i64> = getJoltages(&nums);
    println!("{}", jolts[1] * jolts[3]);
    println!("{:?}", jolts);

    let lengths = getOnesLengths(&nums);

    // Sanity check:
    //  The # of 1-jolt intervals should equal sum of
    //  ( lengths[i]-1 )
    //  because 
    let mut nOneJolts:i64 = 0;
    for l in &lengths{
        nOneJolts += *l as i64 -1;
    }
    if nOneJolts != jolts[1]{
        println!("lengths is wrong!");
    }

    let mut arrangements = 1;
    for length in lengths{
        arrangements *= numVarationsOnOnesLength(length);
    }

    println!("{}",arrangements);

    // Outside the ones lengths, there's no variation.  Therefore
    // # possible arrangements = 
    //  product of (
    //      for each ones interval, the number of possibilities it has
    //  )

}

// Looks for all sections of successive 1-jolt differences, and returns a vector containing
//  the length (inclusive) of all such sections.  All numbers with at least one 1-jolt difference are included
// For example:
//  0,1,4           = length 2 ; section is 0,1
//  0,3,4,5,8       = length 3 ; section is 3,4,5
//  0,1,4,5,6,9,11  = 1x length 2 (0,1), and 1x length 3 (4,5,6)
// The minimum possible length is 2, max is nums.len().
fn getOnesLengths(nums: &Vec<i64>) -> Vec::<usize>
{
    let mut lengths = Vec::<usize>::new();
    let mut prevNum = -10;
    let mut startIndex = 0;
    let mut endIndex = 0;
    let mut weAreInOnesSection = false;
    for (i,num) in nums.iter().enumerate(){
        let jolt = num-prevNum;
        if jolt == 1 && !weAreInOnesSection {
            startIndex = i-1;
            weAreInOnesSection = true;
        }
        else if jolt != 1 && weAreInOnesSection {
            weAreInOnesSection = false;
            endIndex = i;
            lengths.push(endIndex - startIndex);
        }
        prevNum = *num;
    }
    return lengths;
}


fn numVarationsOnOnesLength(l: usize) -> i64{
    let i = l-2;    // The internal length is really what we care about
    if i < 1{   
        return 1;   // at minimum, there's 1 variation
    }
    else{
        return numVariationsRec(i);
    }

}

// i = internal (and therefore switchable) size.
fn numVariationsRec(i: usize) -> i64{
    if i<4{
        match i{
            1 => return 2,
            2 => return 4,
            3 => return 7,
            _ => unreachable!(),
        }
    }
    else{
        return numVariationsRec(i-1)
             + numVariationsRec(i-2)
             + numVariationsRec(i-3);
    }
}

fn getJoltages(nums: &Vec<i64>) -> Vec::<i64>
{
    let mut jolts : Vec::<i64> = [0,0,0,0].to_vec();

    for i in 1..nums.len(){
        jolts[(nums[i] - nums[i-1]) as usize] += 1;
    }
    return jolts;
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

