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
    let v = readLines("C:\\Repos\\adventofcode\\2020\\day8\\src\\input.txt");

    let mut linesToChange = Vec::<usize>::new();
    for (i,line) in v.iter().enumerate(){
        let insVal :Vec<&str> = v[i as usize].split(" ").collect();
        if insVal[0] == "nop" || insVal[0] == "jmp"{
            linesToChange.push(i);
        }
    }


    for changed in linesToChange
    {
        let mut acc :i64 = 0;
        let mut i : i64 = 0;
        let mut visited : HashSet<i64> = HashSet::new();
        while i >= 0 && (i as usize) < v.len()  {
            if visited.contains(&i){
                println!("{}", acc);
                break;
            }
            visited.insert(i);

            let insVal :Vec<&str> = v[i as usize].split(" ").collect();
            let val = insVal[1].parse::<i64>().unwrap();

            let mut ins : &str = insVal[0];
            if changed == (i as usize){
                if insVal[0] == "nop"{
                    ins = "jmp";
                } else if insVal[0] == "jmp"{
                    ins = "nop";
                }
            }

            if ins == "acc"{
                acc += val;
                i += 1;
            }
            if ins == "jmp"{
                i += val;
            }
            if ins == "nop"{
                i += 1;
                continue;
            }
        }
        if i < 0{
            println!("i less than 0! {}", i);
            continue;
        }
        if i as usize == v.len(){
            println!("Accumulator: {}", acc);
            break;
        }
    }

}

// fn executeProgram(code :&Vec<(&str,i64)>){
//     let mut acc :i64 = 0;
//     let mut i : i64 = 0;
//     let mut visited : HashSet<i64> = HashSet::new();
//     while i >= 0 && (i as usize) < code.len()  {
//         if visited.contains(&i){
//             println!("{}", acc);
//             break;
//         }
//         visited.insert(i);

//         let insVal :Vec<&str> = v[i as usize].split(" ").collect();
//         let val = insVal[1].parse::<i64>().unwrap();
//         let ins : &str = insVal[0];
        
//         if ins == "acc"{
//             acc += val;
//             i += 1;
//         }
//         if ins == "jmp"{
//             i += val;
//         }
//         if ins == "nop"{
//             i += 1;
//             continue;
//         }
//     }
// }