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
    doPartTwo();
}

static TileSize : usize = 10;
type Tile = Vec<Vec<char>>;
type Edge = Vec<char>;


fn doPartOne(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day20\\src\\input.txt");

    let tiles = readTiles(&v);

    let mut edgeNums = getEdgeNums(&tiles);

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
}

fn doPartTwo(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day20\\src\\input.txt");

    testRotate();

    let tiles = readTiles(&v);
    let mut edgeNums = getEdgeNums(&tiles);


    // println!("Part 1: {}", prod);
}

fn createImage(tiles: &HashMap<usize, Tile>, ) -> Tile {
    let edgeNums = getEdgeNums(&tiles);


    return Tile::new();
}

fn placeTiles(tiles: &HashMap<usize, Tile>) -> Vec<Vec<Tile>>{


    let mut retTile: Vec<Vec<Tile>> = Vec::new();//= vec![ vec![ '.'; tile.len()]; tile.len()];//Vec::with_capacity(tile.len());
    return retTile;
}

fn testRotate(){
    let mut tile = Tile::new();
    tile.push(['#','.','.'].to_vec());
    tile.push(['.','.','.'].to_vec());
    tile.push(['.','.','.'].to_vec());
    assert!(rotate(&rotate(&rotate(&rotate(&tile)))) == tile);

    let mut tile2 = tile.clone();
    tile2.remove(0);
    tile2.push(['.','.','#'].to_vec());
    assert!(&rotate(&rotate(&tile)) == &tile2);
}

fn rotate(tile: &Tile) -> Tile {
    let mut retTile: Tile = vec![ vec![ '.'; tile.len()]; tile.len()];//Vec::with_capacity(tile.len());
    // retTile.resize()

    for (j,row) in tile.iter().enumerate() {
        for (i,cell) in row.iter().rev().enumerate() {
            retTile[tile.len()-1 - i][j] = tile[j][i];
        }
    }
    return retTile;
}

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




