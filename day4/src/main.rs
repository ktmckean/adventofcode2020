#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::collections::HashMap;

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

static passFields : [&str; 8]= ["byr",
"iyr",
"eyr",
"hgt",
"hcl",
"ecl",
"pid",
"cid"];

fn main() {
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day4\\src\\input.txt");



    let mut valid = 0;
    let mut fields = HashMap::new();
    let mut validPass = true;
    for pass in v.iter()
    {
        if pass.is_empty()
        {
            // // part one //
            // if checkPassValidity(&fields)

            if checkPassValidFields(&fields)
            {
                println!("valid:");
                valid += 1;
            }
            println!("Resetting set...");
            fields.clear();
            fields = HashMap::new();
            continue;
        }
        println!("line is {}",pass);
        for field in pass.split(" ")
        {
            let keyVal = field.split(":").collect::<Vec<&str>>();
            fields.insert(keyVal[0].to_string(), keyVal[1].to_string());
        }
    }
    // if checkPassValidity(&fields)
    if checkPassValidFields(&fields)
    {
        println!("valid:");

        valid += 1;
    }


    println!("{}", valid)

}

fn checkPassValidFields(fields: &HashMap<String,String>) -> bool{
    for field in passFields.iter(){
        if field == &"cid"{
            continue;
        }
        if !fields.contains_key(*field) || fields[*field].is_empty() {
            println!("Invalid!  Pass does not contain field {}", field);
            return false;
        }
        println!("validating {} : {}", field, &fields[*field]);
        if !validateField(field, &fields[*field]){
            return false;
        }

    }

    return true;
}

fn checkPassValidity(fields: &HashMap<String,String>) -> bool{
    // println!("empty!{}",pass);
    let mut validPass = true;
    // pass is done
    for field in passFields.iter() {
            // println!("Checking field: {}", field);
            if !fields.contains_key(*field){
            if *field == "cid"{
                continue;
            }
            // invalid
            // println!("Missing field: {}", field);
            validPass = false;
            break;
        }
    }
    // valid
    if validPass{
        // println!("Adding valid");
    }

    return validPass;
}

static EYE_COLORS: [&str; 7] = ["amb",
 "blu",
 "brn",
 "gry",
 "grn",
 "hzl",
 "oth"];

fn validateField(key :&str, field :&String) -> bool {
    match key as &str{
        "byr" => {
            let val = field.parse::<i32>().unwrap();
            return val >= 1920 && val <=2002;
        },
        "iyr" => {
            let val = field.parse::<i32>().unwrap();
            return val >=2010 && val <=2020;
        },
        "eyr" => {
            let val = field.parse::<i32>().unwrap();
            return val >=2020 && val <=2030;
        },
        "hgt" => {
            let units = &field[field.len()-2..];
            if units == "cm" || units == "in"
            {
                let val = field[..field.len()-2].parse::<i32>().unwrap();
                if units == "cm" && val >= 150 && val <= 193 {return true;}
                if units == "in" && val >= 59 && val <= 76 {return true;}
            }
            return false;
        },
        "hcl" => {
            return validateHgt(field);
        },
        "ecl" => {
            return EYE_COLORS.contains(&&field[..]);
        },
        "pid" => {
            if field.len() == 9 {
                for c in field.chars(){
                    if !c.is_numeric(){
                println!("here0");
                        
                        return false;}
                }
                println!("here1");
                return true;
            }
            println!("here2");

                return false;
        },
        "cid" => { return true;},
        _ => {return false;}
    }
    // return false;
}

fn validateHgt(field:&str) -> bool{
    if field.len() == 7 && field.chars().nth(0).unwrap() == '#' {
        for c in field[1..].chars(){
            if !c.is_numeric() && !c.is_ascii_lowercase(){
                return false;
            }
        }
        return true;
    }
    return false;
}

fn validatebyr(val:i32) -> bool{
    if val <= 1920 && val <=2002 {return true;}
    return false;
}

// fn validatebyr(val:i32) -> bool{
//     if val <= 1920 && val <=2002 {return true;}
//     return false;
// }