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
            File::open("C:\\Repos\\adventofcode\\2020\\day7\\src\\input.txt").unwrap()
        }    };
    let reader = BufReader::new(file);

    let mut v : Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        v.push(line);
    }
    v
}

struct Bag{
    color : String,
    children : Vec<(Bag,i32)>
}

impl Bag{
    fn fromTupleVec(color: &String, children: Vec<(Bag,i32)>) -> Bag{
        return Bag {color:color, children:children};   
    }

    fn fromStringVec(color: &String, children: &Vec<&str>) -> Bag{
        let mut tupleVec = Vec<(Bag,i32)>::new();
        for child in children{
            let num :i32 = bagNums[..1].parse::<i32>().unwrap();
            let color :&str= &bagNums[2..];
            tupleVec.push(color.to_string(), Vec<(Bag,i32)>::new());
        }
        return Bag {color:color, children:tupleVec};
    }

}

fn main() {
    doPart1();

    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day7\\src\\input.txt");
    // doPart2();
 

}

fn doPart1(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day7\\src\\input.txt");

    let mut bags : HashMap::<String, HashSet<String>> = HashMap::new();

    for line in &v{
        let mut colors = line.replace(" bags contain ","\n");
        colors = colors.replace(" bag, ",",");
        colors = colors.replace(" bags, ",",");
        colors = colors.replace(" bag.","");
        colors = colors.replace(" bags.","");
        colors = colors.replace("no other","");
        // println!("{}",colors);



        let bagsPair :Vec<&str>= colors.split("\n").collect();
        let bigBag:&str = &bagsPair[0];
        let littleBags:Vec<&str> = bagsPair[1].split(",").collect();

        if littleBags[0].is_empty(){
            println!("Skipped! {}",bigBag);
            continue;
        }

        for bagNums in &littleBags {
            let bag :&str= &bagNums[2..];
            // println!("{}",bag);

            if bags.contains_key(bag){
                // we're good
            } else {
                bags.insert(bag.to_string(),HashSet::<String>::new());
            }
            bags.get_mut(bag).unwrap().insert(bigBag.to_string());
        }

    }

    let myBagType = "shiny gold";
    let mut topBags = HashSet::<String>::new();
    for bag in &bags[myBagType]{
        addAllParentBagsPart1(&bag, &bags, &mut topBags);
    }

    println!("Part 2: {}", bagsCount);
}

fn doPart2Old(){
    let mut bags : HashMap::<String, HashMap<String, i32>> = HashMap::new();
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day7\\src\\input.txt");

    for line in &v{
        let mut colors = line.replace(" bags contain ","\n");
        colors = colors.replace(" bag, ",",");
        colors = colors.replace(" bags, ",",");
        colors = colors.replace(" bag.","");
        colors = colors.replace(" bags.","");
        colors = colors.replace("no other","");
        // println!("{}",colors);

        let bagsPair :Vec<&str>= colors.split("\n").collect();


        let bigBag:&str = &bagsPair[0];
        let littleBags:Vec<&str> = bagsPair[1].split(",").collect();

        if littleBags[0].is_empty(){
<<<<<<< Updated upstream
            // println!("Skipped! {}",bigBag);
=======
            // println!("No children! {}",bigBag);
>>>>>>> Stashed changes
            continue;
        }
        for bagNum in littleBags{
            let num :i32 = bagNums[..1].parse::<i32>().unwrap();
            let bag :&str= &bagNums[2..];
        }

        if !bags.contains_key(bigBag){
            bags.insert(bigBag.to_string(), HashMap::new());
        }


        for bagNums in &littleBags {
            // println!("{}",&bagNums[..1]);
            let num :i32 = bagNums[..1].parse::<i32>().unwrap();
            let bag :&str= &bagNums[2..];

            if bags[bigBag].contains_key(bag){
                // we're good
                continue;
            } else {
                bags.get_mut(bigBag).unwrap().insert(bag.to_string(), num);
            }
        }

    }

    let myBagType = "shiny gold";
<<<<<<< Updated upstream
    let mut topBags = HashSet::<String>::new();
    for bag in &bags[myBagType]{
        addAllParentBagsPart1(&bag, &bags, &mut topBags);
    }

    println!("Part1: {}", topBags.len());
=======
    let bagsCount : i64 = addAllChildBags(myBagType, &bags) - 1;    // -1 because we don't count our top bag.
    // let mut topBags = HashSet::<String>::new();
    // for bag in &bags[myBagType]{
    //     println!("{}",bag.0);
    //     addAllChildBags(&bag, &bags, &mut bagsCount);
    // }
>>>>>>> Stashed changes

    println!("{}", bagsCount);
}

fn addAllChildBags(bag: &str, rules :&HashMap::<String,HashMap<String,i32>>) -> i64{
    let mut count:i64 = 1;  // 1 for the current bag.
    if !rules[bag].is_empty(){
        for child in &rules[bag]{
            count += *child.1 as i64 * addAllChildBags(&child.0, rules);
        }
    // types.insert(bag.to_string());
    // if !rules.contains_key(bag){
    //     println!("Bag not found in rules: {}",bag);
    // } else{
    //     for parent in &rules[bag]{
    //         addAllParentBags(&parent, rules, types);
    //     }
    }
    return count;
}

fn addAllParentBagsPart1(bag: &str, rules :&HashMap::<String,HashSet<String>>, types: &mut HashSet<String>){
    types.insert(bag.to_string());
    if !rules.contains_key(bag){
        // println!("Bag not found in rules: {}",bag);
    } else{
        for parent in &rules[bag]{
            addAllParentBagsPart1(&parent, rules, types);
        }
    }
}