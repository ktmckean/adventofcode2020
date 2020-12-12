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

#[derive(PartialEq)]
enum Direction{
    North,
    East,
    South,
    West,
}

// enum Direction{
//     North,
//     East,
//     South,
//     West,
// }

fn updateDirection(facing: Direction, d:char, deg:i32) -> Direction{
    let mut returnDirection = facing;
    let mut tempDeg = deg % 360;
    loop{
        if tempDeg <=0 {
            return returnDirection;
        }
        returnDirection = quarterTurn(returnDirection, d, tempDeg);
        tempDeg -= 90;
    }
}

fn quarterTurn(facing: Direction, d:char, deg:i32) -> Direction{  
    if d == 'R'{
        return match facing{
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    else if d=='L'{
        return match facing{
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    else {return facing;}
    // return facing;
}

fn rotateWaypoint(x: &mut i32, y: &mut i32, dir: char, deg: i32){
    let mut rhDeg = deg;
    if dir == 'R'{
        rhDeg = -1*deg;
    }

    if rhDeg%180 == 0 {
        *x *= -1;
        *y *= -1;
        return;
    }
    else if rhDeg%360 == 90 || rhDeg == -270{
        let temp = *x;
        *x = -1* *y;
        *y = temp;
    }
    else if rhDeg%360 == 270 || rhDeg == -90{
        let temp = *x;
        *x = *y;
        *y = -1* temp;
    }
}

fn testUpdateDir(){
    // for quarterTurns in 0..5{
    assert!(updateDirection(Direction::North, 'L', 0) == Direction::North);
    assert!(updateDirection(Direction::North, 'L', 90) == Direction::West);
    assert!(updateDirection(Direction::North, 'L', 180) == Direction::South);
    assert!(updateDirection(Direction::North, 'L', 270) == Direction::East);
    assert!(updateDirection(Direction::North, 'L', 360) == Direction::North);
}

fn testRotateWaypoint()
{
    let mut x = 1;
    let mut y = 0;
    rotateWaypoint(&mut x, &mut y, 'L', 90);
    assert!(x==0 && y == 1);
    rotateWaypoint(&mut x, &mut y, 'R', 90);
    assert!(x==1 && y == 0);
}

fn testModuloSign(){
    assert!(90%360 == 90);
    assert!(-90%360 != 270);
    assert!(-90%360 == -90);
    assert!(90%-360 == 90);
    assert!(-90%-360 == -90);
}




fn main() {
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day12\\src\\input.txt");

    testUpdateDir();
    testRotateWaypoint();
    testModuloSign();

    let mut x = 0;
    let mut y = 0;

    let mut facing = Direction::East;
    // println!("{},{}",x,y);
    for line in &v{
        let dir: char = line.chars().nth(0).unwrap();
        let dist = line[1..].parse::<i32>().unwrap();

        match dir {
            'N' => y += dist,
            'E' => x += dist,
            'S' => y -= dist,
            'W' => x -= dist,
            'L' => facing = updateDirection(facing, dir, dist),
            'R' => facing = updateDirection(facing, dir, dist),
            'F' => match facing {
                Direction::North => y+=dist,
                Direction::East => x+=dist,
                Direction::South => y-=dist,
                Direction::West => x-=dist,
            },

            _ => unreachable!()
        }
        // println!("{},{}",x,y);
    }

    let manhattanDist = x.abs()+y.abs();
    assert!(manhattanDist != 1344);
    println!("Part 1: {}",manhattanDist);

    x = 10;
    y = 1;
    let mut sx = 0;
    let mut sy = 0;
    // println!("{},{},{},{}",sx,sy,x,y);

    for line in v{
        let dir: char = line.chars().nth(0).unwrap();
        let dist = line[1..].parse::<i32>().unwrap();

        match dir {
            'N' => y += dist,
            'E' => x += dist,
            'S' => y -= dist,
            'W' => x -= dist,
            'L' => rotateWaypoint(&mut x, &mut y, dir, dist),
            'R' => rotateWaypoint(&mut x, &mut y, dir, dist),
            'F' => {
                sx += x*dist;
                sy += y*dist;
            },
            _ => unreachable!()
        }
        // println!("{},{},{},{}",sx,sy,x,y);
    }
    let manhattanDist2 = sx.abs()+sy.abs();
    assert!(manhattanDist2 != 27580);
    println!("Part 2: {}",manhattanDist2);
    
}

