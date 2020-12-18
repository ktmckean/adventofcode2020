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
            File::open("C:\\Repos\\adventofcode\\2020\\day18\\src\\input.txt").unwrap()
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

    let mut sum = 0;
    for line in &v{
        sum += eval(&line);
    }
    println!("part1: {}", sum);

    let mut sum2 = 0;
    for line in &v {
        let s = drawParentheses(&line);
        sum2 += eval(&s);
    }
    println!("part2: {}", sum2);

}

fn eval(exp: &str) -> i64{
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

fn drawParentheses(exp: &str) -> String
{
    let mut lastIndex:usize = 2;
    let mut expStr = String::from(exp);
    // println!("Drawing...");
    // println!("{}", expStr);

    loop {
        let nextPlus = expStr[lastIndex ..].find('+');

        if nextPlus.is_none(){
            break;
        }
        else{
            makePlusGroup(&mut expStr, lastIndex + nextPlus.unwrap());
            lastIndex += nextPlus.unwrap() + 2; // +2 because we added a '(' and want to start after it
            // println!("{}", expStr);
        }
    }
    return expStr;
}

fn makePlusGroup(exp: &mut String, i:usize){
    let start = getLeftGroupStart(exp, i);
    exp.insert(start, '(');

    let end = getRightGroupEnd(exp, i+1);
    exp.insert(end, ')');
}

fn getRightGroupEnd(exp:&str, i:usize) -> usize
{
    // println!("getting right from: {}", exp.chars().nth(i+2).unwrap());
    if  exp.chars().nth(i+2).unwrap() != '(' {
        return i+3;
    }

    else{
        let mut nest = 0;
        let mut j = i+2;
        while j < exp.len(){
            match exp.chars().nth(j).unwrap(){
                '(' => nest += 1,
                ')' => nest -= 1,
                _ => {},
            }
            if nest == 0{
                return j;
            }
            j += 1;
        }
        return 0;
    }
}

fn getLeftGroupStart(exp:&str, i:usize) -> usize{
    if  exp.chars().nth(i-2).unwrap() != ')' {
        return i-2;
    }

    else{
        let mut nest = 1;
        let mut j = i-2;
        while j > 0{
            j -= 1;
            match exp.chars().nth(j).unwrap(){
                ')' => nest += 1,
                '(' => nest -= 1,
                _ => {},
            }
            if nest == 0{
                return j;
            }
        }
        return 0;
    }
}

fn evalGroup(exp: &str, i: usize) -> (i64, usize)
{
    let group: &str;
    let mut nest = 1;
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
    // println!("Group: {}", group);
    // println!("Group evaluated to {}",eval1(&group));

    return (eval(&group), endGroup);
}
