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
            File::open("C:\\Repos\\adventofcode\\2020\\day11\\src\\input.txt").unwrap()
        }    };
    let reader = BufReader::new(file);

    let mut v : Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        v.push(line);
    }
    v
}

fn main() {
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day11\\src\\input.txt");

    testNumAdjacent();
    testNumSeenOcc();


    let mut grid = Vec::<Vec<char>>::new();
    for line in &v{
        grid.push(line.chars().collect::<Vec<char>>());
    }

    updateUntilStatic(&mut grid, &CheckSeatType::Adjacent);
    let occupiedAdj = countTotalOccupied(&grid);
    // println!("{:?}", grid);
    assert!(occupiedAdj!=6940);
    println!("Part 1: {}",occupiedAdj);

    let mut grid = Vec::<Vec<char>>::new();
    for line in v{
        grid.push(line.chars().collect::<Vec<char>>());
    }
    updateUntilStatic(&mut grid, &CheckSeatType::Seen);
    let occupiedSeen = countTotalOccupied(&grid);
    println!("Part 2: {}",occupiedSeen);
}

enum CheckSeatType{
    Adjacent,
    Seen,
}


fn testNumAdjacent(){
    let mut grid = Vec::<Vec<char>>::new();
    grid.push(['#','.','#','L','#','L','#','.','#','#'].to_vec());
    grid.push(['#','L','#','L','#','#','#','#','L','#'].to_vec());

    // println!("{}",getNumAdjacentOcc(&grid,0,0));// == 1);
    assert!(getNumAdjacentOcc(&grid,0,0) == 1);
    assert!(getNumAdjacentOcc(&grid,9,0) == 2);
    assert!(getNumAdjacentOcc(&grid,3,1) == 4);
    assert!(getNumAdjacentOcc(&grid,6,1) == 3);
    assert!(getNumAdjacentOcc(&grid,6,0) == 3);


}

fn testNumSeenOcc(){
    let mut grid = Vec::<Vec<char>>::new();
    grid.push(['#','.','.','#','#','L','#','.','#','#'].to_vec());
    grid.push(['#','L','#','L','#','#','#','#','L','#'].to_vec());

    // println!("{}",getNumSeenOcc(&grid,0,0));// == 1);
    assert!(getNumSeenOcc(&grid,0,0) == 2);
    assert!(getNumSeenOcc(&grid,8,0) == 4);
    assert!(getNumSeenOcc(&grid,3,0) == 4);
    // assert!(GetNumSeenOcc(&grid,6,1) == 3);
    // assert!(GetNumSeenOcc(&grid,6,0) == 3);


}

fn updateUntilStatic(grid: &mut Vec<Vec<char>>, method: &CheckSeatType){
    let mut count = 0;

    loop{
        let updated = updateWithMethod(grid, method);
        if !updated {break;}
        count += 1;
    }

    println!("num times updated: {}",count);
}

fn countTotalOccupied(grid: &Vec<Vec<char>>) -> i32{
    let mut occ = 0;
    for row in grid{
         for seat in row{
             if *seat == '#'{
                 occ += 1;
             }
        }
     }   
     occ
}

fn update(grid: &mut Vec<Vec<char>>) -> bool{
    return updateWithMethod(grid, &CheckSeatType::Adjacent);
}

fn updateSeen(grid: &mut Vec<Vec<char>>) -> bool{
    let mut newGrid = grid.clone();
    let mut changed = false;

    for (y,row) in grid.iter().enumerate(){
        for (x,seat) in row.iter().enumerate(){

            if *seat == '.'{
                continue;
            }
            else //if cell == 'L' || cell == '#'
            {
                let n = getNumSeenOcc(&grid,x as i32,y as i32);

                if *seat == 'L' && n == 0{
                    newGrid[y][x] = '#';
                    changed = true;
                }
                else if *seat == '#' && n >= 5{                 
                    newGrid[y][x] = 'L';
                    changed = true;
                }
            }
        }
    }
    if changed{
        assert!(newGrid!=*grid);
        *grid = newGrid;
    }
    changed
}

fn updateAdj(grid: &mut Vec<Vec<char>>) -> bool{
    let mut newGrid = grid.clone();
    let mut changed = false;

    for (y,row) in grid.iter().enumerate(){
        for (x,seat) in row.iter().enumerate(){

            if *seat == '.'{
                continue;
            }
            else //if cell == 'L' || cell == '#'
            {
                let n = getNumAdjacentOcc(&grid,x as i32,y as i32);

                if *seat == 'L' && n == 0{
                    newGrid[y][x] = '#';
                    changed = true;
                }
                else if *seat == '#' && n >= 4 {
                    newGrid[y][x] = 'L';
                    changed = true;
                }
            }
        }
    }
    if changed{
        assert!(newGrid!=*grid);
        *grid = newGrid;
    }
    return changed;
}
// This actually works, and barely slower, but it's more confusing and slower than
//  just having two separate methods.
fn updateWithMethod(grid: &mut Vec<Vec<char>>, method: &CheckSeatType) -> bool{
    let mut newGrid = grid.clone();
    let mut changed = false;

    for (y,row) in grid.iter().enumerate(){
        for (x,seat) in row.iter().enumerate(){

            if *seat == '.'{
                continue;
            }
            else //if cell == 'L' || cell == '#'
            {
                let n = match &method{
                    CheckSeatType::Adjacent => getNumAdjacentOcc(&grid,x as i32,y as i32),
                    CheckSeatType::Seen => getNumSeenOcc(&grid,x as i32,y as i32),
                };

                if *seat == 'L' && n == 0{
                    newGrid[y][x] = '#';
                    changed = true;
                }
                else if *seat == '#' 
                && n >= match method{
                    CheckSeatType::Adjacent => 4,
                    CheckSeatType::Seen => 5
                } {                 
                    newGrid[y][x] = 'L';
                    changed = true;
                }
            }
        }
    }
    if changed{
        assert!(newGrid!=*grid);
        *grid = newGrid;
    }
    return changed;
}

fn getNumSeenOcc(grid: &Vec<Vec<char>>, x: i32, y: i32) -> i32{
    // Sign multipliers
    let mut lowx :i32 = -1;
    let mut lowy :i32= -1;
    let mut highx:i32 = 1;
    let mut highy:i32 = 1;
    
    if x==0{
        lowx = 0;
    } else if x as usize == grid[0].len() - 1{
        highx = 0;
    }
    
    if y==0{
        lowy = 0;
    } else if y as usize== grid.len() - 1{
        highy = 0;
    }

    let mut numOcc = 0;
    for i in lowx .. highx + 1{
        for j in lowy .. highy + 1{

            if i==0 && j ==0{
                continue;
            }
            if isClosestSeenOccupied(&grid, x ,y,i,j){
                numOcc += 1;
            }

            assert! (x+i >= 0);
            assert! (x+i < grid[0].len() as i32);
            assert! (y+j >= 0);
            assert! (y+j < grid.len() as i32);
        }
    }
    return numOcc;
}

fn isClosestSeenOccupied(grid: &Vec<Vec<char>>, x:i32,y:i32,i:i32,j:i32) -> bool{
    let mut xpos :i32 = x;
    let mut ypos :i32 = y;
    loop{
        xpos += i;
        ypos += j;
        if !(0..grid[0].len()).contains(&(xpos as usize))
        || !(0 .. grid.len() ).contains(&(ypos as usize))
        {
            return false;
        }

        if grid[ypos as usize][xpos as usize] == '#'{
            return true;
        }
        else if grid[ypos as usize][xpos as usize] == 'L'{
            return false;
        }
    }
}



fn getNumAdjacentOcc(grid: &Vec<Vec<char>>, x: i32, y: i32) -> i32{
    let mut lowx :i32 = -1;
    let mut lowy :i32= -1;
    let mut highx:i32 = 1;
    let mut highy:i32 = 1;

    if x==0{
        lowx = 0;
    } else if x as usize == grid[0].len() - 1{
        highx = 0;
    }
    
    if y==0{
        lowy = 0;
    } else if y as usize== grid.len() - 1{
        highy = 0;
    }

    let mut numOcc = 0;
    for i in lowx .. highx + 1{
        for j in lowy .. highy + 1{
            if i==0 && j ==0{
                continue;
            }

            assert! (x+i >= 0);
            assert! (x+i < grid[0].len() as i32);
            assert! (y+j >= 0);
            assert! (y+j < grid.len() as i32);

            if grid[(y+j) as usize][(x+i) as usize] == '#'
            {
                numOcc+=1;
            }
        }
    }
    return numOcc;
}