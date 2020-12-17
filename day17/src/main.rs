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


// struct ptnd{
//     pub ndim : usize;
//     pub coords : Vec<i32>;
// }


fn main() {
    // doPartOne();

    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day17\\src\\input.txt");

    let numTurns = 6;
    // let nDim = 4;
    let gridSize = numTurns*2 + v.len();

    // let mut grid = Vec::<Vec<Vec<bool>>>::new();
    let mut grid: Vec<Vec<Vec<Vec<i32>>>> = vec![vec![vec![vec![0 ; gridSize]; gridSize]; gridSize]; gridSize];
    // for yzplane in &grid{
    //     for mut zcol in yzplane{
    //         zcol = vec![false; gridSize];
    //     }
    // }

    // println!("{},{},{}",grid.len(), grid[0].len(), grid[0][0].len());

    for (i,line) in v.iter().enumerate(){
        for (j,c) in line.chars().enumerate(){
            if c == '#'{
                grid[gridSize/2][gridSize/2][i+numTurns][j+numTurns] = 1;
            }
        }
    }

    // printGrid(&grid);
    for _i in 0..numTurns{
        println("update {} ...", i);
        update(&mut grid);
    }


    let mut numActive = 0;
    for x in 0..grid.len(){
        for y in 0..grid.len(){
            for z in 0..grid.len() {
                for w in 0..grid.len(){
                    if grid[x][y][z][w] != 0{
                        numActive += 1;
                    }
                }
            }
        }
    }
    println!("part 2: {}", numActive);

}

// fn doPartOne(){
//     let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day17\\src\\input.txt");

//     let numTurns = 6;

//     let gridSize = numTurns*2 + v.len();

//     // let mut grid = Vec::<Vec<Vec<bool>>>::new();
//     let mut grid = vec![vec![vec![0 ; gridSize]; gridSize]; gridSize];
//     // for yzplane in &grid{
//     //     for mut zcol in yzplane{
//     //         zcol = vec![false; gridSize];
//     //     }
//     // }

//     println!("{},{},{}",grid.len(), grid[0].len(), grid[0][0].len());

//     for (i,line) in v.iter().enumerate(){
//         for (j,c) in line.chars().enumerate(){
//             if c == '#'{
//                 grid[gridSize/2][i+numTurns][j+numTurns] = 1;
//             }
//         }
//     }

//     // printGrid(&grid);
//     for _i in 0..numTurns{
//         update(&mut grid);
//     }


//     let mut numActive = 0;
//     for x in 0..grid.len(){
//         for y in 0..grid.len(){
//             for z in 0..grid.len() {
//                 if grid[x][y][z] != 0{
//                     numActive += 1;
//                 }
//             }
//         }
//     }
//     println!("part1: {}", numActive);
// }

fn update(grid: &mut  Vec<Vec<Vec<Vec<i32>>>>){
    let gridSize = grid.len();
    let mut updated = grid.clone();//vec![vec![vec![0 ; gridSize]; gridSize]; gridSize];
    
    for x in 0..grid.len(){
        for y in 0..grid.len(){
            for z in 0..grid.len() {
                for w in 0..grid.len(){
                    let numActiveAdj = getNumAdjacent(grid, x as i64,y as i64,z as i64, w as i64);
                    // println!("numadj {}", numActiveAdj);

                    if grid[x][y][z][w] != 0{
                        if !(numActiveAdj == 2 || numActiveAdj == 3){
                            updated[x][y][z][w] = 0;
                        }
                    }
                    else if numActiveAdj == 3{
                        updated[x][y][z][w] = 1;
                    }
                }
            }
        }
    }
    *grid = updated.clone();

    // printGrid(grid);



}

// fn printGrid(grid: &Vec::<Vec<Vec<Vec<i32>>>>){
//     // assert!(grid != updated);
    
//     for yz in grid{
//         // print!("[");
//         for y in yz{
//             for z in y{
//                 print!("{:?}", z);
//             }
//             print!("]\n");
//         }
//         print!("]\n");

//     }
//     println!("break");
// }


fn getNumAdjacent(grid: &Vec<Vec<Vec<Vec<i32>>>>, x: i64, y: i64, z: i64, w: i64) -> i32{
    let mut lowx :i64 = -1;
    let mut lowy :i64= -1;
    let mut highx:i64 = 1;
    let mut highy:i64 = 1;
    let mut lowz :i64= -1;
    let mut highz:i64 = 1;
    let mut loww :i64= -1;
    let mut highw:i64 = 1;

    if x==0{
        lowx = 0;
    } else if x as usize == grid.len() - 1{
        highx = 0;
    }
    
    if y==0{
        lowy = 0;
    } else if y as usize== grid[0].len() - 1{
        highy = 0;
    }

    if z==0{
        lowz = 0;
    } else if z as usize== grid[0][0].len() - 1{
        highz = 0;
    }
    if w==0{
        loww = 0;
    } else if w as usize== grid[0][0][0].len() - 1{
        highw = 0;
    }


    let mut numActive:i32 = 0;
    for i in lowx .. highx + 1{
        for j in lowy .. highy + 1{
            for k in lowz .. highz + 1{
                for l in loww .. highw + 1{
                    if i==0 && j ==0 && k == 0 && l==0{
                        continue;
                    }

                    assert! (x+i >= 0);
                    assert! (x+i < grid.len() as i64);
                    assert! (y+j >= 0);
                    assert! (y+j < grid[0].len() as i64);
                    assert! (z+k >= 0);
                    assert! (z+k < grid[0][0].len() as i64);
                    assert! (w+l >= 0);
                    assert! (w+l < grid[0][0].len() as i64);

                    if grid[(x+i) as usize][(y+j) as usize][(z+k) as usize][(w+l) as usize] != 0
                    {
                        numActive+=1;
                    }
                }
            }
        }
    }
    return numActive;
}


    // let mut fields = Vec::<(i64,i64,i64,i64)>::new();
    // for line in &v[..20]{
    //     let parts: Vec<&str> = line.split(" ").collect();
    //     let mut field = (1,1,1,1);

    //     let r1 : Vec<&str> = parts[parts.len()-3].split("-").collect();
    //     field.0 = r1[0].parse::<i64>().unwrap();
    //     field.1 = r1[1].parse::<i64>().unwrap();
        
    //     let r2 : Vec<&str> = parts[parts.len()-1].split("-").collect();
    //     field.2 = r2[0].parse::<i64>().unwrap();
    //     field.3 = r2[1].parse::<i64>().unwrap();

    //     println!("{:?}",field);
    //     fields.push(field);
    // }


    // let mut sumInvalid = 0;
    // // let mut numInvalid = 0;
    // let mut tickets = Vec::<Vec<i64>>::new();
    // for ticket in &v[25..]{
    //     let mut nums = Vec::<i64>::new();
    //     // println!("{}",ticket);
    //     let mut num : i64;
    //     let mut validTic = true;

    //     for n in ticket.split(","){
    //         // println!("{}",n);
    //         // nums.push(n.parse::<i64>().unwrap());
    //         num = n.parse::<i64>().unwrap();
    //         nums.push(num);

    //         let mut validNum = false;
    //         for field in &fields{
    //             // println!("{:?}",field);
                
    //             if isNumInField(&num, &field){
    //                 validNum = true;
    //                 break;
    //             }
    //         }
    //         if !validNum{
    //             sumInvalid += num;
    //             validTic = false;
    //         } 
    //     }

    //     if validTic{
    //         tickets.push(nums.clone());
    //     }
    // }

    // assert!(2316340 != sumInvalid);
    // assert!(599514 != sumInvalid);
    // println!("part1: sumInvalid tickets: {}",sumInvalid);

    // let mut myTicket =Vec::<i64>::new();
    // for n in v[22].split(","){
    //     myTicket.push(n.parse::<i64>().unwrap());
    // }
    // tickets.push(myTicket.clone());


    // let mut validFields = Vec::<Vec<usize>>::new();
    // for _num in &tickets[0]{
    //     let valid : Vec<usize> = (0..20).collect();
    //     validFields.push(valid);
    // }

    // for ticket in &tickets{
    //     for (i,num) in ticket.iter().enumerate() {
    //         let mut invalids = Vec::<usize>::new();
    //         for (j,fieldNum) in validFields[i].iter().enumerate() {
    //             if !isNumInField(&num, &fields[*fieldNum]){
    //                 invalids.push(j);
    //             }
    //         }

    //         let mut numRemoved = 0;
    //         for inv in invalids{
    //             // println!("removing {}, with {} removed prior",inv, numRemoved);
    //             validFields[i].remove(inv-numRemoved);
    //             numRemoved += 1;
    //         }
    //     }
    // }



    // // nth value here = the column of ticket numbers corresponding to the nth field
    // let mut ticketFields = vec![0; 20];

    // let mut keepGoing = true;
    // while keepGoing{
    //     keepGoing = false;
    //     for list in &validFields{
    //         println!("{:?}",list);
    //     }


    //     let mut assignedField = 200;
    //     for (i,col) in validFields.iter().enumerate(){
    //         if col.len() == 0{
    //             continue;
    //         }
    //         if col.len() == 1 {
    //             keepGoing = true;
    //             assignedField = col[0];
    //             println!("assigned {}!",col[0]);
    //             // ticketFields[i] = col[0];    // ticketFields index = ticket column index // DEBUG ONLY
    //             ticketFields[col[0]] = i;       // ticketFields index = fields index, value is ticket column
    //             // validFields.remove(i);
    //             // println!("{:?}",validFields);
    //             break;
    //         }
    //     }

    //     for list in &mut validFields{
    //         list.retain(|&x| x!=assignedField);
    //     }
    //     // println!("{:?}",ticketFields);

    // }

    // println!("part2: ticketFields {:?}",&ticketFields);

    // // Get myTicket's product
    // let mut product :i64 = 1;
    // for (i,fieldNum) in ticketFields[..6].iter().enumerate() {
    //     // println!("MyTickets[{}] = {}",fieldNum,myTicket[*fieldNum]);
    //     product *= myTicket[*fieldNum]
    // }

    // assert!(1846746207799 > product);
    // assert!(1288736562277 < product);

    // println!("part2 answer: {}",product);
// }

