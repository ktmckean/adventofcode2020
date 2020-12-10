use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    let file = File::open("C:\\Users\\Kerry\\coding\\aoc2020\\day1\\src\\input.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            // Try the work computer's path
            println!("Error: {},\nTrying work path...",error);
            File::open("C:\\Repos\\adventofcode\\2020\\day1\\src\\input.txt").unwrap()
        }
    };
    let reader = BufReader::new(file);

    let mut v : Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        // println!("{}",line);
        v.push(line.parse::<i32>().unwrap());
    }

    doPart1(&v);
    doPart2(&v);

}

fn doPart1(v: &Vec<i32>)
{
    for num in v
    {
        for num2 in v
        {
            if num+num2 == 2020
            {
                println!("Part1: {}", num*num2);
                return;
            }
        }
    }
}


fn doPart2(v: &Vec<i32>)
{
    for num in v
    {
        for num2 in v
        {
        for num3 in v
        {
            if num+num2+num3 == 2020
            {
                println!("Part2: {}", num*num2*num3);
                return;
            }
        }
        }
    }
}
