#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() {

    let file = File::open("C:\\Users\\Kerry\\coding\\aoc2020\\day2\\src\\input.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let reader = BufReader::new(file);

    let mut v : Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        // println!("{}",line);
        v.push(line);
    }

    let mut numTrue = 0;
    let mut numFalse = 0;

    for line in v{
        let split = line.find('-').unwrap();
        let space = line.find(' ').unwrap();
        let min = &line[0..split].parse::<usize>().unwrap();
        let max = &line[split+1..space].parse::<usize>().unwrap();
        //  println!("{},{}",min, max);
        let c = &line[space+1..space+2];
        let pass = &line[space+4..];

        // if checkPassValid(pass, c, min, max) {
        if checkPassPart2(pass, c.chars().nth(0).unwrap(), *min, *max) {
            numTrue += 1;
        } else {
            numFalse += 1;
        }

        // println!("{},{}",numFalse, numTrue);

    }
    println!("{},{}",numFalse, numTrue);


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

fn checkPassValid(pass : &str, c: &str, min : &i32, max : &i32) -> bool
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