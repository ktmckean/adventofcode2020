#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use id_tree::*;

fn readLines(path :&str) -> std::vec::Vec<String>{
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            // Try the work computer's path
            println!("Error: {},\nTrying work path...",error);
            File::open("C:\\Repos\\adventofcode\\2020\\day19\\src\\input.txt").unwrap()
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


fn main() {
    doPartOne();
    doPartTwo();

    // let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day18\\src\\input.txt");

 
    // println!("part 2: {}", numActive);

}

static mut END_RULES: [(usize, char); 2] = [(0,'a'),(0,'a')];



fn doPartTwo(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day19\\src\\input2.txt");

    let rulesVec = scanAllRules(&v);

    // let mut numMatches = countNumValidRules(&v,&rulesVec);
    // println!("Part 2: {}", numMatches);
}

fn doPartOne(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day19\\src\\input.txt");

    let rulesVec = scanAllRules(&v);

    let mut numMatches = countNumValidRules(&v,&rulesVec);
    println!("Part 1: {}", numMatches);
}

fn countNumValidRules(v: &std::vec::Vec<String>, rulesVec: &Vec<Vec<Vec<usize>>>) -> usize{
    let numRules = rulesVec.len();
    let mut numMatches = 0;
    for line in &v[numRules+1 ..] {
        // println!("{}",line);

        // let mut j = 0;
        // let matched = nextRuleMetValid(line, &mut j, 0, &rulesVec);
        let matched = checkRule0(line,&rulesVec);
        // let matched = checkRule(line, &mut j, &rulesVec, 0);
        // println!("{}", matched);
        if matched{
            numMatches += 1;
        }
    }
    return numMatches;
}


fn scanAllRules(v: &std::vec::Vec<String>) -> Vec<Vec<Vec<usize>>>{
    let numRules = getNumRules(&v);
    let mut rulesVec = Vec::<Vec<Vec<usize>>>::with_capacity(numRules);
    rulesVec.resize(numRules, Vec::<Vec<usize>>::new());

    let mut scannedEndRules = Vec::<(usize, char)>::new();


    for line in &v[.. numRules]{
        let parts = line.split(": ").collect::<Vec<&str>>();


        let ruleNum = parts[0].parse::<usize>().unwrap();

        if parts[1].contains("\""){
            let charVal = parts[1].chars().nth(parts[1].len() - 2).unwrap();
            scannedEndRules.push((ruleNum, charVal));

            rulesVec[ruleNum] = Vec::<Vec<usize>>::new();   // We will have to check around this placeholder
        }
        else {
            let mut ruleSets = Vec::<Vec<usize>>::new();

            for ruleSet in parts[1].split("|").collect::<Vec<&str>>(){
                let mut rules = Vec::<usize>::new();

                for rule in ruleSet.split(" ").collect::<Vec<&str>>(){
                    let parseRule = rule.parse::<usize>();
                    if !parseRule.is_err(){
                        rules.push(parseRule.unwrap());                
                    }
                }
                ruleSets.push(rules)
            }
            rulesVec[ruleNum] = ruleSets;
        }
    }

    set_END_RULES(&scannedEndRules);
    return rulesVec
}

fn set_END_RULES(newRules: &Vec<(usize, char)>)
{
    unsafe{
        assert!(newRules.len() == END_RULES.len());

        for (i,val) in newRules.iter().enumerate(){
            assert!(i<END_RULES.len());
            END_RULES[i] = *val;
        }
    }
}

fn getNumRules(v: &Vec<String>) -> usize{
    for (i,line) in v.iter().enumerate(){
        if line.is_empty(){
            return i;
        }
    }
    return v.len();
}


// fn testNextRuleValid(){
//     // assert!(nextRuleMetValid("ab", 0, ))
// }

fn nextRuleMetValid(s: &str, i: &mut usize, thisRuleIdx: usize, rules: &Vec<Vec<Vec<usize>>>) -> bool
{
    // println!("checking rule {} : {:?}", thisRuleIdx, &rules[thisRuleIdx]);
    if *i >= s.len() {return false;}
    if isEndRule(&thisRuleIdx){
        // *i += 1;
        return s.chars().nth(*i).unwrap() == getRuleChar(&thisRuleIdx);
    }



    
    // let thisRule = rules[ruleNum];
    // RuleSet is a vector of vector of ruleNums
    let mut setValid = true;
    for ruleSet in &rules[thisRuleIdx]
    {
        setValid = true;
        // Rule Set is a vector of ruleNums
        for (j,ruleNum) in ruleSet.iter().enumerate(){
            println!("{}, {}, checking ruleNum {:?}", *i+j, s.chars().nth(*i+j).unwrap(), ruleNum);

            if isEndRule(&ruleNum){
                if s.chars().nth(*i+j).unwrap() == getRuleChar(&ruleNum){
                    // *i += 1;
                    continue;
                }
                else{
                    // *i += 1;
                    setValid = false;
                    // println!("Nope!");
                    break;
                }
            }
            else{
                // println!("about to check ruleNum {}", ruleNum);
                *i += 1;
                let nextValid = nextRuleMetValid(s, i, *ruleNum, rules);

                if !nextValid{
                    *i -= 1;
                    setValid = false;
                    break;
                }
                else{
                    // *i += 1;
                    // println!("success! i is now {}", *i);
                }
            }
        }
        if setValid{
            // println!("valid ruleset update pre:  {}", *i);
            *i += ruleSet.len();
            // println!("valid ruleset update post: {}", *i);
            // println!("ruleset {}, {:?} was vaild!", thisRuleIdx, ruleSet);
            return true;
        }
    }
    // println!("rule {}, {:?} was not valid", thisRuleIdx, rules[thisRuleIdx]);
    return false;
}

//  Assumes ruleNum is not end rule
fn checkRuleSet(s: &str, i: &mut usize, rules: &Vec<Vec<Vec<usize>>>, ruleNum: usize) -> bool
{
    assert!(!isEndRule(&ruleNum));
    let startChar = *i;

    for ruleSet in &rules[ruleNum]{
        let mut isSetValid = true;
        *i = startChar;

        for subRuleNum in ruleSet{
            if !checkRule(s, i, rules, *subRuleNum){
                isSetValid = false;
                break;
            }
        }

        if isSetValid{
            return true;
        }
    }
    return false;
}



fn checkRule(s: &str, i: &mut usize, rules: &Vec<Vec<Vec<usize>>>, ruleNum: usize) -> bool
{
    // println!("checking rule {} : {:?}", ruleNum, &rules[ruleNum]);
    if *i >= s.len(){
        return false;
    }

    if isEndRule(&ruleNum){
        if s.chars().nth(*i).unwrap() == getRuleChar(&ruleNum){
            *i += 1;
            return true;
        }
        else{
            return false;
        }
    }
    else{
        return checkRuleSet(s, i, rules, ruleNum);
    }
}

fn checkRule0(s: &str, rules: &Vec<Vec<Vec<usize>>>) -> bool
{
    // println!("checking rule 0 : {:?}", &rules[0]);

    let mut i = 0;
    let ruleValid = checkRule(s, &mut i, rules, 0);

    return ruleValid && i==s.len();
}

fn isEndRule(num: &usize) -> bool {
    unsafe {
        for pair in END_RULES.iter(){
            if *num == pair.0{
                return true;
            }
        }
    }
    return false;
    // return *num == 77 || *num==91;
}

// fn isEndRule(num: &usize) -> bool {
//     return *num == 4 || *num==5;
// }

fn getRuleChar(num: &usize) -> char{
    unsafe {
        for pair in END_RULES.iter(){
            if *num == pair.0{
                return pair.1
            }
        }
        unreachable!();
    }
    // return match *num{
    //     91 => 'b',
    //     77 => 'a',
    //     _ => unreachable!(),
    // };
}

// fn getRuleChar(num: &usize) -> char{
//     return match *num{
//         5 => 'b',
//         4 => 'a',
//         _ => unreachable!(),
//     };
// }

// fn assembleRule(rules : {
//     let mut rules = HashMap::<Rule, Rule>::new();

//     addRule(&mut rules, 0);
// }

// fn addRule(&mut rules: HashMap<Rule,Rule>, ruleNum: usize){

// }
