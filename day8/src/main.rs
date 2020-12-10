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
        Err(error) => {
            // Try the work computer's path
            println!("Error: {},\nTrying work path...",error);
            File::open("C:\\Repos\\adventofcode\\2020\\day8\\src\\input.txt").unwrap()
        }    };
    let reader = BufReader::new(file);

    let mut v : Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        v.push(line);
    }
    v
}

fn main() {
    let v = readLines("C:\\Repos\\adventofcode\\2020\\day8\\src\\input.txt");
    let mut code = parseInputToCode(&v);

    doPart1(&code);
    doPart2(&mut code);
}

fn doPart2(code: &mut Vec::<(&str,i64)>){
    let mut linesToChange = Vec::<usize>::new();
    for i in 0..code.len() {
        if swapCmdIfJmpNop(i, &mut code[i]){
            linesToChange.push(i);

            let (finali,acc) = executeProgram(&code);
            if finali < 0{
                println!("Error!  Negative i: {}",i);
                return;
            }
            else if finali as usize == code.len(){
                println!("Part 2 answer: {}",acc);
            }

            // Swap the commands back
            swapCmdIfJmpNop(i, &mut code[i]);
        }
    }
    println!("No swappable instruction found that provides part2 solution.");
}

fn swapCmdIfJmpNop(i: usize, cmd: &mut (&str,i64)) -> bool {
    match cmd.0{
        "nop" => {
            cmd.0 = "jmp";
            return true;
        },
        "jmp" => {
            cmd.0 = "nop";
            return true;
        }
        _ => {},
    }
    return false;

    // if code[i].0 == "nop"{
    //     code[i].0 = "jmp";
    // } 
    // else if code[i].0 == "jmp"{
    //     code[i].0 = "nop";
    // }
}

fn doPart1(code: &Vec::<(&str,i64)>){
    let (i,acc) = executeProgram(&code);
    println!("Part 1 answer: {}",acc);
}

fn executeProgram(code :&Vec<(&str,i64)>) -> (i64,i64){
    let mut acc :i64 = 0;
    let mut i : i64 = 0;
    let mut visited = vec![false; code.len()];
    while i >= 0 && (i as usize) < code.len()  {
        if visited[i as usize]{
            // println!("Final acc: {}", acc);
            break;
        }
        visited[i as usize] = true;

        let ins = code[i as usize].0;
        let val = code[i as usize].1;
        
        if ins == "acc"{
            acc += val;
            i += 1;
        }
        if ins == "jmp"{
            i += val;
        }
        if ins == "nop"{
            i += 1;
        }
    }
    return (i, acc);
}

fn parseInputToCode(v: &Vec<String>) -> Vec::<(&str,i64)>{
    let mut code = Vec::<(&str,i64)>::new();
    
    for line in v.iter(){
        let insVal :Vec<&str> = line.split(" ").collect();
        let val = insVal[1].parse::<i64>().unwrap();
        let ins : &str = insVal[0];
        code.push((ins,val));
    }
    code
}