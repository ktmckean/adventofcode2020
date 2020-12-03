#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;

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
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day3\\src\\input.txt");

    let xincrememnt = [1,3,5,7,1];
    let yincrememnt = [1,1,1,1,2];
    let mut treesVec : Vec<i64> = Vec::new();

    for inc in xincrememnt.iter().enumerate()
    {
        treesVec.push(
            getNumTrees(
                &v,
                xincrememnt[inc.0],
                yincrememnt[inc.0]
        ));
    }

    let mut product = 1;
    for num in treesVec{
        product *= num;
    }
    println!("{}",product)

}

fn getNumTrees(v : &Vec<String>, xinc: i32, yinc: i32) -> i64
{
    let mut trees = 0;
    let width = v[0].len();

    let mut x = 0;
    let mut y = -1;
    for line in v{
        y += 1;
        if y % yinc != 0
        {
            continue;
        }

        if line.chars().nth(x % width).unwrap() == '#'
        {
            trees += 1;
        }
        x += xinc as usize;
    }
    trees
}