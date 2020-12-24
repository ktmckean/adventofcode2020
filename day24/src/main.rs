#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::collections::HashSet;
use std::collections::HashMap;
// use std::collections::LinkedList;
// use id_tree::*;

fn readLines(path :&str) -> std::vec::Vec<String>{
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            // Try the work computer's path
            println!("Error: {},\nTrying work path...",error);
            File::open("C:\\Repos\\adventofcode\\2020\\day24\\src\\input.txt").unwrap()
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
    // let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day18\\src\\input.txt");
    doPartOne();
    doPartTwo();
}

fn doPartOne(){
    // let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day24\\src\\test.txt");
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day24\\src\\input.txt");

    let mut tiles = flipTilesOnInput(&v);

    let mut numBlack = 0;
    for t in tiles {
        if t.1 == true{
            numBlack += 1;
        }
    }

   
    println!("Part 1: {}",numBlack);
}

fn doPartTwo(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day24\\src\\input.txt");

    let mut tiles = flipTilesOnInput(&v);

    println!("Day 0: {}", getNumBlack(&tiles));


    for i in 0..100 {
        for t in getTilesToFlip(&tiles) {
            let state = tiles.get(&t).unwrap_or(&false);
            tiles.insert(t, !state);
        }
        println!("Day {}: {}", i+1, getNumBlack(&tiles));
    }

    // let mut numBlack = 0;
    // for t in tiles {
    //     if t.1 == true{
    //         numBlack += 1;
    //     }
    // }


   
    println!("Part 2: {}",getNumBlack(&tiles));
}

fn getNumBlack(tiles: &HashMap<(i32,i32), bool>) -> usize{
    let mut numBlack = 0;
    for t in tiles {
        if *t.1 == true{
            numBlack += 1;
        }
    }
    numBlack
}


fn getTilesToFlip(tiles: &HashMap<(i32,i32), bool>) -> HashSet<(i32,i32)>
{
    let blacks = getBlackTiles(&tiles);
    let mut toFlip = HashSet::<(i32,i32)>::new();

    for b in blacks{
        let mut numAdj = 0;
        let adj = getAdjacentTiles(b);
        for t in adj{
            // Check for adjacent blacks
            if *tiles.get(&t).unwrap_or(&false) {
                numAdj += 1;
            }
            else {
                // Check whites for black neighbors
                if doWeInsertWhite(&tiles, t){
                    toFlip.insert(t);
                }
            }
        }
        if numAdj == 0 || numAdj >2 {
            toFlip.insert(b);
        }
    }
    return toFlip;
}

fn doWeInsertWhite(tiles: &HashMap<(i32,i32), bool>, t:(i32,i32)) -> bool
{
    assert!(false == *tiles.get(&t).unwrap_or(&false));
    let whiteNeighbors = getAdjacentTiles(t);
    let mut numAdjToWhite = 0;

    for wn in whiteNeighbors{
        if *tiles.get(&wn).unwrap_or(&false){
            numAdjToWhite += 1;
        }
    }
    if numAdjToWhite == 2{
        return true;
    }
    return false;
}


fn getBlackTiles(tiles: &HashMap<(i32,i32), bool>) -> Vec<(i32,i32)>{
    let mut black = Vec::<(i32,i32)>::new();
    for t in tiles{
        if *t.1{
            black.push(*t.0);
        }
    }
    return black;
}


fn getAdjacentTiles(coord: (i32,i32)) -> Vec<(i32,i32)>
{
    let mut v = Vec::<(i32,i32)>::new();

    v.push((coord.0+2, coord.1));
    v.push((coord.0-2, coord.1));
    v.push((coord.0+1, coord.1+2));
    v.push((coord.0-1, coord.1+2));
    v.push((coord.0+1, coord.1-2));
    v.push((coord.0-1, coord.1-2));
    return v;
}

fn getTilesWithin2(coord: (i32,i32)) -> HashSet<(i32,i32)>
{
    let mut v = HashSet::<(i32,i32)>::new();
    for t in getAdjacentTiles(coord){
        for t2 in getAdjacentTiles(t){
            v.insert(t2);
        }
    }

    return v;
}


fn flipTilesOnInput(v: &Vec<String>) -> HashMap<(i32,i32), bool> {
    let mut tiles = HashMap::<(i32,i32), bool>::new();

    for line in v {
        let tile = parseDirections(&line);
        if tiles.contains_key(&tile){
            *tiles.get_mut(&tile).unwrap() = !tiles[&tile];
            // println!("flipping");
        }
        else {
            // println!("adding");
            tiles.insert(tile, true);
        }
    }
    return tiles;
}

fn parseDirections(dir: &str) -> (i32,i32)
{
    let mut coord: (i32,i32) = (0,0);
    let mut waiting = false;
    for c in dir.chars() {
        if !waiting{
            match c {
                'e' => coord.0 += 2,
                'w' => coord.0 -= 2,
                's' => {
                    coord.1 -= 2;
                    waiting = true;
                },
                'n' => {
                    coord.1 += 2;
                    waiting = true;
                },
                _ => unreachable!(),
            }
        }
        else {
            waiting = false;
            match c {
                'e' => coord.0 += 1,
                'w' => coord.0 -= 1,
                _ => unreachable!(),
            }
        }
    }
    return coord;
}