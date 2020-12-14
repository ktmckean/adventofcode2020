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

// #[derive(PartialEq)]
// enum Direction{
//     North,
//     East,
//     South,
//     West,
// }

// enum Direction{
//     North,
//     East,
//     South,
//     West,
// }






fn main() {
    doPartOne();

    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day14\\src\\input.txt");

    let mut ops = Vec::<Vec<&str>>::new();
    let mut op = Vec::<&str>::new();
    for line in &v {
        let lineArr : Vec<&str> = line.split(' ').collect();//::Vec<&str>();

        match & lineArr[0][..3] {
            "mas" => {
                if !&op.is_empty(){
                    ops.push(op.clone());
                    op.clear();
                }
                op.push(lineArr[2])
            },
            "mem" => op.push(line),
            _ => unreachable!(),
        }
    }
    ops.push(op.clone());

    let mut mem = HashMap::<i64,i64>::new();

    let mut mask :&str = "";
    for op in ops{
        mask = op[0];
        for cmd in &op[1..]{
            let parts :Vec<&str>= cmd.split(' ').collect();
            // let mut addrStr  = &parts[0][4 .. &parts[0].len()-1];
            let baseAddr  = &parts[0][4 .. &parts[0].len()-1].parse::<i64>().unwrap();
            let val = parts[2].parse::<i64>().unwrap();
            


            for addr in getFloatingValues(mask, *baseAddr){
                println!("Writing to addr: {}", addr);
                mem.insert(addr, val);
            }
            println!("Next command...");


            // insert the value


            // mem.insert(*addr, (val & getMaskWithXOne(mask)) | getMaskWithXZero(mask) );
        }
    }

    let mut part2Answer = 0;
    for (addr,val) in mem{
        part2Answer += val;
    }


    // let numDigits = getNumBinaryDigits(baseAddr);
    let maskSection = &mask[mask.len() ..];
    // println!("mask, section: {}, {}",mask, maskSection);
    println!("Part 2: {}",part2Answer);

    // let vals = getFloatingValues(mask, 9);
    // println!("26 mod 4: {}",26 % 4);

    assert!(numHasBinaryDigit(26,2));

}



fn getFloatingValues(mask: &str, baseAddr: i64) ->HashSet<i64>{
    let maskSection = &mask[mask.len() - getNumBinaryDigits(baseAddr) as usize ..];

    let mut minAddr = baseAddr;
    let mut factors = Vec::<i64>::new();
    let mut magnitude = 1;
    // println!("Starting with baseAddr {}...",baseAddr);

    for (i,val) in maskSection.chars().rev().enumerate(){
        if val == 'X' {
            factors.push(magnitude);
            // println!("Found X with binary value magnitude: {}", magnitude);

            if numHasBinaryDigit(baseAddr, magnitude){
                minAddr -= magnitude;
                // println!("new minAddr: {}, magnitude: {}", minAddr, magnitude);
            }
        }
        else if val == '1'{
            if !numHasBinaryDigit(baseAddr, magnitude){
                minAddr += magnitude;
                // println!("Adding digit for {}'s place.  MinAddr is now {}",magnitude, minAddr);
            }
        }
        magnitude *= 2;
    }


    println!("Getting variations on {} with factors {:?}",minAddr, factors);
    return getVariations(minAddr, &factors);
}

fn numHasBinaryDigit(num:i64, digitsPlace:i64) -> bool{
    // println!("num({}) mod 2*digits({}): {}",num, digitsPlace, num % (2*digitsPlace));
    return num % (2*digitsPlace) >= digitsPlace;
}



fn getNumBinaryDigits(num: i64) -> i32{
    let mut magnitude = 1;
    let mut value = 0;
    let mut digits = 0;
    while value < num{
        digits += 1;
        value += magnitude;
        magnitude *= 2;
    }
    return digits;
}

// Assume that mask is a binary number except for the X's which are at indices tracked by positions
fn getVariations(minAddr: i64, factors: &Vec::<i64>) -> HashSet::<i64>
{
    let mut set = HashSet::<i64>::new();
    getVariationsRec(minAddr, factors, &mut set);
    return set;
}

fn getVariationsRec(minAddr: i64, factors: &Vec::<i64>, set: &mut HashSet<i64>) -> HashSet::<i64>
{
    println!("factors rec: {:?}",factors);

    let mut variations = HashSet::<i64>::new();
    if factors.len() == 0{
        variations.insert(minAddr);
        return variations;
    }
    if factors.len() == 1{
        variations.insert(minAddr);
        variations.insert(minAddr + factors[0]);
        return variations;
    }


    for (i,factor) in factors.iter().enumerate(){
        // add all variations where this val is 0
        getVariationsRec(minAddr, &factors[i+1..].to_vec(), set);

        // add all variatiosn where this val is 1
        getVariationsRec(minAddr+factor, &factors[i+1..].to_vec(), set);
    }
    println!("returning: {:?}",variations);

    return variations;
}

fn getBinStrAsi64(s: &str) ->i64{
    let mut magnitude = 1;
    let mut value = 0;
    for i in s.chars().rev(){
        match i{
            '1' => value += magnitude,
            '0' => {},
            _ => {
                println!("unexpected value: {}",i);
                unreachable!();
            },
        }
        magnitude *= 2;
    }
    return value;

}

fn doPartOne(){
    
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day14\\src\\input.txt");

    let mut ops = Vec::<Vec<&str>>::new();
    let mut op = Vec::<&str>::new();
    for line in &v {
        let lineArr : Vec<&str> = line.split(' ').collect();//::Vec<&str>();

        match & lineArr[0][..3] {
            "mas" => {
                if !&op.is_empty(){
                    ops.push(op.clone());
                    op.clear();
                }
                op.push(lineArr[2])
            },
            "mem" => op.push(line),
            _ => unreachable!(),
        }
    }
    ops.push(op.clone());
    // println!("op: {:?}", ops);

    let mut mem = HashMap::<i64,i64>::new();

    let mut mask :&str = "";
    for op in ops{
        // println!("op: {:?}", op);
        mask = op[0];
        for cmd in &op[1..]{
            let parts :Vec<&str>= cmd.split(' ').collect();
            let addr  = &parts[0][4 .. &parts[0].len()-1].parse::<i64>().unwrap();
            let val = parts[2].parse::<i64>().unwrap();
            // println!("addr: {}", addr);
            // println!("val: {}", val);

            // insert the value
            mem.insert(*addr, (val & getMaskWithXOne(mask)) | getMaskWithXZero(mask) );
        }
    }

    let mut part1Answer = 0;
    for (addr,val) in mem{
        part1Answer += val;
    }
    println!("Part 1: {}",part1Answer);
}

fn getMaskWithXZero(mask: &str) -> i64{
    let mut magnitude = 1;
    let mut value = 0;
    for i in mask.chars().rev(){
        match i{
            '1' => value += magnitude,
            '0' => {},
            'X' => {},
            _ => {
                println!("unexpected value: {}",i);
                unreachable!();
            },
        }
        magnitude *= 2;
    }
    return value;
}

fn getMaskWithXOne(mask: &str) -> i64{
    let mut magnitude = 1;
    let mut value = 0;
    for i in mask.chars().rev(){
        match i{
            '1' => value += magnitude,
            'X' => value += magnitude,
            '0' => {},
            _ => {
                println!("unexpected value: {} in mask: {}",i, mask);
                unreachable!();
            },
        }
        magnitude *= 2;
    }
    return value;
}





