use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {

    let file = File::open("C:\\Users\\Kerry\\coding\\aoc2020\\day1\\src\\input.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let reader = BufReader::new(file);

    let mut v : Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        // println!("{}",line);
        v.push(line.parse::<i32>().unwrap());
    }

    for num in &v
    {
        for num2 in &v
        {
        for num3 in &v
        {
            if num+num2+num3 == 2020
            {
                println!("{}", num*num2*num3);
            }
        }
        }
    }


    // for line in file.iter()
    // {
    //     // v.push_back(line);
    //     println!("{:#?}", line);
    // }


}
