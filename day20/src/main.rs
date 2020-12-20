#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
// use std::collections::HashSet;
use std::collections::HashMap;
// use id_tree::*;

fn readLines(path :&str) -> std::vec::Vec<String>{
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            // Try the work computer's path
            println!("Error: {},\nTrying work path...",error);
            File::open("C:\\Repos\\adventofcode\\2020\\day20\\src\\input.txt").unwrap()
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
    // doPartTwo();
}

static TileSize : usize = 10;
type Tile = Vec<Vec<char>>;
type Edge = Vec<char>;


fn doPartOne(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day20\\src\\input.txt");

    let tiles = readTiles(&v);

    let mut edgeNums = HashMap::<Edge, usize>::new();


    for t in &tiles{
        for mut e in getEdges(&t.1){
            if !edgeNums.contains_key(&e){
                edgeNums.insert(e.clone(), 1);
            }
            else {
                *edgeNums.get_mut(&e).unwrap() += 1;
            }

            e.reverse();
            if !edgeNums.contains_key(&e){
                edgeNums.insert(e.clone(), 1);
            }
            else {
                *edgeNums.get_mut(&e).unwrap() += 1;
            }

        }
    }

    let mut corners = Vec::<usize>::new();

    for t in &tiles {
        let mut numUnmatched = 0;
        for mut e in getEdges(&t.1){
            if edgeNums[&e] > 1{
                continue
            }
            e.reverse();
            if edgeNums[&e] > 1{
                continue;
            }
            numUnmatched += 1;
        }
        if numUnmatched >1{
            corners.push(*t.0);
        }
    }

    assert!(corners.len() == 4);
    let mut prod: i64 = 1;
    for id in corners{
        prod *= id as i64;
    }
    println!("Part 1: {}", prod);

    let pic = Vec::<Vec<Tile>>::new();

    // println!("{}", tiles.len());
    // for e in edgeNums{
    //     println!("{}", e.1);
    //     if e.1 > 2{
    //         println!("Alert!  Found a high number edge!");
    //     }
    // }

}

// fn createImage(tiles: &HashMap<usize, Tile>, ) -> Tile {

// }

fn getEdgeNums(tiles: &HashMap<usize, Tile>) -> HashMap<Edge, usize>{
    let mut edgeNums = HashMap::<Edge, usize>::new();

    for t in tiles{
        for mut e in getEdges(&t.1){
            if !edgeNums.contains_key(&e){
                edgeNums.insert(e.clone(), 1);
            }
            else {
                *edgeNums.get_mut(&e).unwrap() += 1;
            }

            e.reverse();
            if !edgeNums.contains_key(&e){
                edgeNums.insert(e.clone(), 1);
            }
            else {
                *edgeNums.get_mut(&e).unwrap() += 1;
            }

        }
    }
    return edgeNums;
}

fn readTiles(v: &Vec<String>) -> HashMap<usize, Tile>
{
    let mut tiles = HashMap::<usize, Tile>::new();

    let mut tileNum = 0;// : usize;
    let mut tile = Vec::<Vec<char>>::new();

    for line in v {
        if line.is_empty() {
            tiles.insert(tileNum, tile);
            tile = Vec::<Vec<char>>::new();
            continue;
        }
        else if line.contains(":") {
            assert!(line.contains(" "));
            let space = line.find(" ").unwrap()+1;
            let colon = line.find(":").unwrap();
            // println!("{}", &line[space .. colon]);
            tileNum = line[space .. colon].parse::<usize>().unwrap()
        }
        else {
            let mut row = Vec::<char>::new();
            for c in line.chars() {
                row.push(c);
            }
            tile.push(row);
        }
    }
    tiles.insert(tileNum, tile);
    return tiles;
}

fn getEdges(t: &Tile) -> Vec<Edge> {
    let mut edges = Vec::<Edge>::new();
    let mut r = Vec::<char>::new();
    let mut l = Vec::<char>::new();

    for row in t {
        l.push(row[0]);
        r.push(row[TileSize-1]);
    }

    // RH rule
    let mut top = t[0].clone();
    top.reverse();
    r.reverse();

    edges.push(top);
    edges.push(l);
    edges.push(t[TileSize-1].clone());
    edges.push(r);

    return edges;
}




