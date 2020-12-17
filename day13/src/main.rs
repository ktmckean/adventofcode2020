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
            File::open("C:\\Repos\\adventofcode\\2020\\day13\\src\\input.txt").unwrap()
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

// #[derive(PartialEq)]
// enum Direction{
//     North,
//     East,
//     South,
//     West,
// }

// enum Direction{
//     North,
//     East,
//     South,
//     West,
// }






fn main() {
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day13\\src\\input.txt");
    println!("l1: {}",v [0]);

    doPart1();
    
    // let schedule = "17,x,13,19";
    // let schedule = "67,7,59,61";
    // let schedule = "67,x,7,59,61";
    // let schedule = "67,7,x,59,61";
    // let schedule = "1789,37,47,1889";
    let schedule = &v[1];

    let busStrs: Vec<&str>  = schedule.split(',').collect();
    let mut buses = Vec::<(i64,i64)>::new();

    for (i,s) in busStrs.iter().enumerate(){
        if !s.contains('x'){
            // println!("{}",s);
            buses.push((i as i64,s.parse::<i64>().unwrap()));
        }
    }

    for bus in &buses{
        assert!(isPrime(bus.1));
    }


    let mut found = Vec::<bool>::new();
    for bus in &buses{
        found.push(false);
    }
    found[0] = true;
    let mut remaining = found.len() -1;

    let mut t = buses[0].1;
    let mut increment:i64 = buses[0].1;
    loop{
        for (i,bus) in buses.iter().enumerate(){
            if found[i]{
                continue;
            }

            if (t + bus.0) % bus.1 == 0 {
                increment *= bus.1;
                found[i] = true;
                remaining -= 1;
            }
        }
        if remaining <= 0 { 
            break;
        }    
        t += increment;
    }

    println!("part 2: {}",t);
}

fn isPrime(a: i64) -> bool{
    let mut f = 2;
    while f < a {
        if a % f == 0{
            return false;
        }
        f += 1;
    }
    return true;
}





fn  doPart1(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day13\\src\\input.txt");
    let departureTime = v[0].parse::<i64>().unwrap();
    // println!("got here");

    let busStrs: Vec<&str>  = v[1].split(',').collect();
    let mut buses = Vec::<i64>::new();

    for s in busStrs{
        if !s.contains('x'){
            // println!("{}",s);
            buses.push(s.parse::<i64>().unwrap());
        }
    }
    
    let mut minBus:i64 = buses[0];
    let mut waitTime:i64 =(departureTime/minBus + 1)*minBus - departureTime;
    for bus in &buses{
        // println!("factor: {}", departureTime / minBus);
        let nextTime = (departureTime/bus + 1)*bus;
        // assert!(nextTime > departureTime);
        
        // println!("{},{}",nextTime, waitTime);

        if nextTime - departureTime < waitTime{
            minBus = *bus;
            waitTime = nextTime - departureTime;
        }


        // println!("{}, ",bus % departureTime);
        // if (bus % departureTime) < waitTime{
        //     minBus = bus;
        //     waitTime = bus % departureTime;
        // }
    }
    assert!(524353728 != minBus * waitTime);
    println!("Part 1: {}", minBus* waitTime );
}

