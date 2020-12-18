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
        Err(error) => {
            // Try the work computer's path
            println!("Error: {},\nTrying work path...",error);
            File::open("C:\\Repos\\adventofcode\\2020\\day17\\src\\input.txt").unwrap()
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
    doPartOne();

    // let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day18\\src\\input.txt");

 
    // println!("part 2: {}", numActive);

}

fn doPartOne(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day18\\src\\input.txt");

    let expressions = Vec::<&str>::new();
    
    // let mut sum = 0;
    // for line in &v{
    //     sum += eval1(&line);
    // }
    // println!("part1: {}", sum);

    let mut sum2 = 0;
    for line in &v{
        sum2 += eval2(&line);
    }
    println!("part2: {}", sum2);
}

fn eval2(exp: &str) -> i64{
    let mut val = 0;
    let mut nextOp = 'N';
    let mut skipUntil = 0;
    let mut multStack = Vec::<i64>::new();
    let mut pendingMult = 1;

    for (nth,c) in exp.chars().enumerate(){
        if skipUntil != 0 {
            if nth <= skipUntil{
                continue;
            }
            skipUntil = 0;
        }
        println!("{}, {}, {}, {}, {}", val, nextOp, c, nth, skipUntil);

        match c {
            ' ' => continue,
            '*' => nextOp = c,
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' =>{
                if nextOp == 'N'{
                    // multStack.push(c.to_digit(10).unwrap() as i64)
                    val = c.to_digit(10).unwrap() as i64;
                }
                else if nextOp == '*'{
                    multStack.push(c.to_digit(10).unwrap() as i64)
                }
            },
            '+' => {
                let (newVal, endSkip) = evalPlusGroup(&exp, nth-2, val);
                println!("plus group finished with value: {}", newVal);
                val = newVal;
                skipUntil = endSkip;
            },
            ')' => {
                while !multStack.is_empty(){
                    val *= multStack.pop().unwrap();
                }
            }
            '(' => {
                let (newVal,endGroup) = evalGroup2(&exp, nth);
                val = applyOp(val, newVal, nextOp);
                skipUntil = nth+1+endGroup;
            },
            _ => unreachable!(),
        };
    }
    while !multStack.is_empty(){
        println!("popping stack: {} --> val becomes {}", multStack.last().unwrap(), val * multStack.last().unwrap());
        val *= multStack.pop().unwrap();
    }
    return val;
}

// Starts with some number before a plus, with index pointing to what is to be added
fn evalPlusGroup(exp: &str, i: usize, startVal: i64) -> (i64,usize){
    println!("Evaluating plus group: {}",&exp[i..]);
    let mut skipUntil = 0;
    let mut val = startVal;
    
    for (nth,c) in exp[i..].chars().enumerate() {
        if skipUntil != 0 {
            if nth <= skipUntil{
                continue;
            }
            skipUntil = 0;
        }
        println!("{}, {}, _, {}, {} ", val, c, nth, skipUntil);

        match c{
            ' '|')'|'+' => continue,
            '*' => return (val, i+nth-1),
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' =>
            {
                println!("Adding {}", c);
                val += c.to_digit(10).unwrap() as i64
            },
            '(' => {
                let (newVal,endGroup) = evalGroup2(&exp, i+nth);
                val = applyOp(val, newVal, '+');
                skipUntil += nth+1+endGroup;
            },
            _ => unreachable!(),
        };
    }

    return (val, exp.len()-1);
}

fn eval1(exp: &str) -> i64{
    let mut val = 0;
    let mut nextOp = 'N';
    let mut skipUntil = 0;

    for (nth,c) in exp.chars().enumerate(){
        // println!("{}, {}, {}, {}, {}", val, nextOp, c, nth, skipUntil);
        if skipUntil != 0 {
            if nth <= skipUntil{
                continue;
            }
            skipUntil = 0;
        }

        match c{
            ' '|')' => continue,
            '*'|'+' => nextOp = c,
            '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' =>{
                val = applyOp(
                    val
                    , c.to_digit(10).unwrap() as i64
                    , nextOp);
            },
            '(' => {
                let (newVal,endGroup) = evalGroup(&exp, nth);
                val = applyOp(val, newVal, nextOp);
                skipUntil = nth+1+endGroup;
            },
            _ => unreachable!(),
        };
    }
    return val;
}

fn applyOp (val: i64, newVal: i64, op: char) -> i64{
    match op{
        '*' => return val * newVal,
        '+' => return val + newVal,
        'N' => return newVal,
        _ => unreachable!(),
    };
}

fn evalGroup2(exp: &str, mut i: usize) -> (i64, usize)
{
    let group: &str;
    let mut nest = 1;
    let groupSize:usize;
    let mut endGroup = 0;
    
    // println!("scanning {}",&exp[i+1..]);
    for (i,c) in exp[i+1..].chars().enumerate() {
        match c {
            '(' => nest += 1,
            ')' => nest -= 1,
            _ => continue,
        }
        if nest == 0 {
            endGroup = i;
            break;
        }
    }
    assert!(endGroup != 0);
    group = &exp[i+1 .. i+1+endGroup];
    println!("Group: {}", group);
    println!("Group evaluated to {}",eval2(&group));

    return (eval2(&group), endGroup);
} 

fn evalGroup(exp: &str, i: usize) -> (i64, usize)
{
    let group: &str;
    let mut nest = 1;
    let groupSize:usize;
    let mut endGroup = 0;
    
    println!("scanning {}",&exp[i+1..]);
    for (i,c) in exp[i+1..].chars().enumerate() {
        match c {
            '(' => nest += 1,
            ')' => nest -= 1,
            _ => continue,
        }
        if nest == 0 {
            endGroup = i;
            break;
        }
    }
    assert!(endGroup != 0);
    group = &exp[i+1 .. i+1+endGroup];
    // println!("Group: {}", group);
    // println!("Group evaluated to {}",eval1(&group));

    return (eval1(&group), endGroup);
} 
