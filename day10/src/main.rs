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

    let jolts :Vec::<usize> = getJoltages(&nums);
    println!("Part 1: {}", jolts[1] * jolts[3]);
    assert!(jolts[1]*jolts[3] == 2450);
    // println!("{:?}", jolts);


    // Part 2
    // Conveniently, we have no two-jolt intervals.  We will use this fact.
    let lengths = getSections(&nums);

    // Sanity check:
    //  The # of 1-jolt intervals should equal sum of
    //  ( lengths[i]-1 )
    testOneJoltsMatchLengths(&jolts, &lengths);

    let mut arrangements = 1;
    for length in lengths{
        arrangements *= numVarationsOnOnesLength(length);
    }

    println!("Part 2: {}",arrangements);
    assert!(arrangements == 32396521357312);
}

// Looks for all sections of successive 1-jolt differences, and returns a vector containing
//  the length (inclusive) of all such sections.  All numbers with at least one 1-jolt difference are included
// For example:
//  0,1,4           = length 2 ; section is 0,1
//  0,3,4,5,8       = length 3 ; section is 3,4,5
//  0,1,4,5,6,9,11  = 1x length 2 (0,1), and 1x length 3 (4,5,6)
// The minimum possible length is 2, max is nums.len().
fn getSections(nums: &Vec<i64>) -> Vec::<usize>
{
    let mut lengths = Vec::<usize>::new();
    let mut prevNum = -10;
    let mut startIndex = 0;
    let mut endIndex;
    let mut weAreInOnesSection = false;
    for (i,num) in nums.iter().enumerate(){
        let jolt = num-prevNum;

        if !weAreInOnesSection && jolt == 1 {
            startIndex = i-1;
            weAreInOnesSection = true;
        }
        else if weAreInOnesSection && jolt != 1 {
            weAreInOnesSection = false;
            endIndex = i;
            lengths.push(endIndex - startIndex);
        }
        prevNum = *num;
    }
    return lengths;
}


fn numVarationsOnOnesLength(l: usize) -> i64{
    // We can't omit the ends, because they already have a gap of three to
    //  the next section.  They can't vary, so let's remove them from consideration.
    let i = l-2;
    if i < 1{   
        return 1;   // at minimum, there's 1 "variation"
    }
    else{
        return numVariationsRec(i);
    }
}

// i = internal (and therefore switchable) size of the section.
fn numVariationsRec(i: usize) -> i64{
    if i<4{
        match i{
            1 => return 2,
            2 => return 4,
            3 => return 7,
            _ => unreachable!(),
        }
    }
    // Could memoize this but it already runs instantly.
    else{ 
        return numVariationsRec(i-1)    // For including the first adapter
             + numVariationsRec(i-2)    // For omitting the first and including the second adapter
             + numVariationsRec(i-3);   // For omitting the first two but including the third
    }
}

fn getJoltages(nums: &Vec<i64>) -> Vec::<usize>
{
    let mut jolts : Vec::<usize> = [0,0,0,0].to_vec();

    for i in 1..nums.len(){
        jolts[(nums[i] - nums[i-1]) as usize] += 1;
    }
    return jolts;
}

fn testOneJoltsMatchLengths(jolts: &Vec<usize>, lengths: &Vec<usize>)
{
    let mut nOneJolts:usize = 0;
    for l in lengths{
        nOneJolts += *l -1;
    }

    if nOneJolts != jolts[1]{
        println!("lengths is wrong!");
    }
}