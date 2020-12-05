#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::collections::HashSet;

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
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day5\\src\\input.txt");

    let mut seats = HashSet::<i32>::new();
    let mut maxid = 0;
    for code in v{
        let row = getRow(&code);
        let col = getCol(&code);
        let id = getId(row,col);
        if id > maxid{
            maxid = id;
        }
        seats.insert(getId(row,col));
    }
    for row in 0..128{
        for col in 0..8{
            if !seats.contains(&getId(row,col))
            && seats.contains(&getId(row,col+1))
            && seats.contains(&getId(row,col-1))
            {
                println!("{},{}",row,col);
                println!("{}",8*row+col)
            }
        }
    }
}

fn getId(row:i32, col:i32) -> i32{
    return 8*row+col;
}


fn getCol(id : &str) -> i32
{
    let chars = id.chars().rev();
    let mut row = 0;
    let mut magnitude = 1;
    for c in chars
    {
        if c != 'L' && c != 'R' {continue;}
        if c == 'R'{
            row += magnitude;
        }
        magnitude *= 2;

    }
    return row;
}

fn getRow(id : &str) -> i32
{
    let chars = id.chars().rev();
    let mut row = 0;
    let mut magnitude = 1;
    for c in chars
    {
        if c != 'B' && c != 'F' {continue;}
        if c == 'B'{
            row += magnitude;
        }
        magnitude *= 2;
    }
    return row;
}