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
    // doPartOne();

    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day16\\src\\input.txt");

    let mut fields = Vec::<(i64,i64,i64,i64)>::new();
    for line in &v[..20]{
        let parts: Vec<&str> = line.split(" ").collect();
        let mut field = (1,1,1,1);

        let r1 : Vec<&str> = parts[parts.len()-3].split("-").collect();
        field.0 = r1[0].parse::<i64>().unwrap();
        field.1 = r1[1].parse::<i64>().unwrap();
        
        let r2 : Vec<&str> = parts[parts.len()-1].split("-").collect();
        field.2 = r2[0].parse::<i64>().unwrap();
        field.3 = r2[1].parse::<i64>().unwrap();

        println!("{:?}",field);
        fields.push(field);
    }


    let mut sumInvalid = 0;
    // let mut numInvalid = 0;
    let mut tickets = Vec::<Vec<i64>>::new();
    for ticket in &v[25..]{
        let mut nums = Vec::<i64>::new();
        // println!("{}",ticket);
        let mut num : i64;
        let mut validTic = true;

        for n in ticket.split(","){
            // println!("{}",n);
            // nums.push(n.parse::<i64>().unwrap());
            num = n.parse::<i64>().unwrap();
            nums.push(num);

            let mut validNum = false;
            for field in &fields{
                // println!("{:?}",field);
                
                if isNumInField(&num, &field){
                    validNum = true;
                    break;
                }
            }
            if !validNum{
                sumInvalid += num;
                validTic = false;
            } 
        }

        if validTic{
            tickets.push(nums.clone());
        }
    }

    assert!(2316340 != sumInvalid);
    assert!(599514 != sumInvalid);
    println!("part1: sumInvalid tickets: {}",sumInvalid);

    let mut myTicket =Vec::<i64>::new();
    for n in v[22].split(","){
        myTicket.push(n.parse::<i64>().unwrap());
    }
    tickets.push(myTicket.clone());


    let mut validFields = Vec::<Vec<usize>>::new();
    for _num in &tickets[0]{
        let valid : Vec<usize> = (0..20).collect();
        validFields.push(valid);
    }

    for ticket in &tickets{
        for (i,num) in ticket.iter().enumerate() {
            let mut invalids = Vec::<usize>::new();
            for (j,fieldNum) in validFields[i].iter().enumerate() {
                if !isNumInField(&num, &fields[*fieldNum]){
                    invalids.push(j);
                }
            }

            let mut numRemoved = 0;
            for inv in invalids{
                // println!("removing {}, with {} removed prior",inv, numRemoved);
                validFields[i].remove(inv-numRemoved);
                numRemoved += 1;
            }
        }
    }



    // nth value here = the column of ticket numbers corresponding to the nth field
    let mut ticketFields = vec![0; 20];

    let mut keepGoing = true;
    while keepGoing{
        keepGoing = false;
        for list in &validFields{
            println!("{:?}",list);
        }


        let mut assignedField = 200;
        for (i,col) in validFields.iter().enumerate(){
            if col.len() == 0{
                continue;
            }
            if col.len() == 1 {
                keepGoing = true;
                assignedField = col[0];
                println!("assigned {}!",col[0]);
                // ticketFields[i] = col[0];    // ticketFields index = ticket column index // DEBUG ONLY
                ticketFields[col[0]] = i;       // ticketFields index = fields index, value is ticket column
                // validFields.remove(i);
                // println!("{:?}",validFields);
                break;
            }
        }

        for list in &mut validFields{
            list.retain(|&x| x!=assignedField);
        }
        // println!("{:?}",ticketFields);

    }

    println!("part2: ticketFields {:?}",&ticketFields);

    // Get myTicket's product
    let mut product :i64 = 1;
    for (i,fieldNum) in ticketFields[..6].iter().enumerate() {
        // println!("MyTickets[{}] = {}",fieldNum,myTicket[*fieldNum]);
        product *= myTicket[*fieldNum]
    }

    assert!(1846746207799 > product);
    assert!(1288736562277 < product);

    println!("part2 answer: {}",product);
}

// fn isTicketValid(ticket: &Vec<i64>, fields: &Vec<(i64,i64,i64,i64)>) -> bool{
//     for num in ticket{
//         for field in fields{
//             if !isNumInField(num,&field){
//                 return false;
//             }
//         }
//     }
//     return true;
// }

fn isNumInField(num: &i64, field: &(i64,i64,i64,i64)) -> bool{
    if num >= &field.0 && num <= &field.1 {
        // println!("{} in range1 {} - {}",num, field.0, field.1);
        return true;
    }
    if num >= &field.2 && num <= &field.3{
        // println!("{} in range2 {} - {}",num, field.2, field.3);
        return true;
    }
    // println!("{} not in range {} - {} or {} - {}!",num, field.0, field.1, field.2, field.3);

    return false;
}