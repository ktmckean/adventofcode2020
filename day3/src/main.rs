#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};

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
        // println!("{}",line);
        v.push(line);
    }
    v
}



fn main() {
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day3\\src\\input.txt");


    let width = v[0].len();

    let xincrememnt = [1,3,5,7,1];
    let yincrememnt = [1,1,1,1,2];

    let mut treesVec : Vec<i64> = Vec::new();

    for inc in xincrememnt.iter().enumerate()
    {
        let mut x = 0;
        let mut trees = 0;
        println!("{}", inc.0);
        let mut skip = true;

        for line in &v{
            if inc.0 == 4{
                skip = !skip;
                if skip{
                    continue;
                }
            }


            if line.chars().nth(x % width).unwrap() == '#'
            {
                trees += 1;
                // println!("{}", "woo!");
            }
            x += inc.1;//xincrememnt[inc];

        }
        println!("{}",trees);
        treesVec.push(trees);
    }
    let mut product = 1;
    for num in treesVec{
        product *= num;
    }
    println!("{}",product)

}

