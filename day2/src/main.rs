#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() {

    let file = File::open("C:\\Users\\Kerry\\coding\\aoc2020\\day2\\src\\input.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            // Try the work computer's path
            println!("Error: {},\nTrying work path...",error);
            File::open("C:\\Repos\\adventofcode\\2020\\day2\\src\\input.txt").unwrap()
        }
    };
    let reader = BufReader::new(file);

    let mut v : Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        // println!("{}",line);
        v.push(line);
    }

    let mut p1True = 0;
    let mut p2True = 0;

    for line in v{
        let split = line.find('-').unwrap();
        let space = line.find(' ').unwrap();
        let min = &line[0..split].parse::<i32>().unwrap();
        let max = &line[split+1..space].parse::<i32>().unwrap();
        //  println!("{},{}",min, max);
        let c = &line[space+1..space+2];
        let pass = &line[space+4..];

        if checkPassPart1(pass, c, min, max) {
            p1True += 1;
        }



        if checkPassPart2(pass, c.chars().nth(0).unwrap(), *min as usize, *max as usize) {
            p2True += 1;
        }

        // println!("{},{}",numFalse, numTrue);

    }
    println!("Part1: {}", p1True);
    println!("Part2: {}", p2True);


}

fn checkPassPart2(pass : &str, c: char, i1 : usize, i2 : usize) -> bool
{
    let p1 : char = pass.chars().nth(i1-1).unwrap(); // pass[i1..i1+1];
    let p2 : char = pass.chars().nth(i2-1).unwrap(); //[i2..i2+1];

    if (p1 == c || p2 == c) && p1 != p2
    {
        return true;
    }
     false
}

fn checkPassPart1(pass : &str, c: &str, min : &i32, max : &i32) -> bool
{
    let mut count : i32 = 0;
    let mut checkpoint : usize = 0;
    let mut slice = &pass[checkpoint..];
    while count <= *max {
        checkpoint = slice.find(c).unwrap_or(100000);


        // println!("{},{},{},{}", slice, c, count, checkpoint);

        if checkpoint >= 100000{
            if count >= *min{
                return true;
            } else {
                return false;
            }
        } else {
            checkpoint += 1;
            slice = &slice.get(checkpoint..).unwrap();
            count += 1;
        }
    }

    return false;
}