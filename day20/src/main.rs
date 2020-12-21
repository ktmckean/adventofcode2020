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

static TILE_SIZE : usize = 10;
static GRID_SIZE : usize = 12;
type Tile = Vec<Vec<char>>;
type Edge = Vec<char>;


fn doPartOne(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day20\\src\\input.txt");

    let tiles = readTiles(&v);

    let edgeNums = getEdgeNums(&tiles);

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

    let mut tiles = readTiles(&v);
    let _edgeNums = getEdgeNums(&tiles);

    placeTiles(&mut tiles);


    // println!("Part 1: {}", prod);
}

fn createImage(tiles: &HashMap<usize, Tile>, ) -> Tile {
    let _edgeNums = getEdgeNums(&tiles);


    return Tile::new();
}

fn placeTiles(mut tiles: &mut HashMap<usize, Tile>) -> Vec<Vec<Tile>> {    
    let edgeNums = getEdgeNums(&tiles);
    let mut grid = generateSquareGrid();    // Already sized and indexable

    // let mut grid = initializeGrid(&mut tiles, &edgeNums);

    placeTopRow(&mut tiles, &edgeNums, &mut grid);

    // let mut edgePieces = getSingleEdgePieces(tiles, &edgeNums);
    // let mut corners = getCornerPieces(tiles, &edgeNums);
    // assert!(corners.len() == 2);
    // assert!(edgePieces.len() == 30);

    // println!("{:?}",grid);

    for row in 1 .. 2{
        placeRowMatchingTop(&mut tiles, &mut grid, row, &edgeNums);
    }


    // println!("{:?}", grid);





    return grid;
}

fn placeRowMatchingTop(mut tiles: &mut HashMap<usize, Tile>, mut grid: &mut Vec<Vec<Tile>>, row:usize,  edgeNums: &HashMap<Edge, usize>) {
    // let mut edgeNums = getEdgeNums(&tiles);
    // for n in edgeNums{
    //     print!("{}",n.1);
    //     if *n.1 != 1 && *n.1 != 2{
    //         println!("Alert!");
    //     }
    // }


    println!("\nEdge nums of row: {}",row-1);
    for tile in &grid[row-1] {
        let edges = &getEdges(&tile);
        
        for e in edges {
            print!("{}",edgeNums[e]);
        }
        print!("\n",);

        // let edgeToMatch = &edges[2];
        // // println!("here");
        // let mut numMatches = 0;
        // numMatches = edgeNums[&*edgeToMatch];
        // print!("{} ", numMatches);
    }
    // // assert!(edgeNums[edgeToMatch] == 2);

    // for i in 0 .. grid[row].len() {
    //     println!("Matching row, col: {},{}",row,i);
    //     placePieceMatchingTop(tiles, grid, row, i);
    // }
}


fn placeTopRow(mut tiles: &mut HashMap<usize, Tile>, edgeNums: &HashMap<Edge, usize>, mut grid: &mut Vec<Vec<Tile>>) {
    placeTopLeftCorner(&mut tiles, edgeNums, &mut grid);
    placeRowInterior(&mut tiles, &mut grid, 0);
    placePieceMatchingLeft(&mut tiles, &mut grid, 0, GRID_SIZE-1);
}

fn placeRowInterior(mut tilePool: &mut HashMap<usize, Tile>, mut grid: &mut Vec<Vec<Tile>>, row: usize) {  
    for i in 1 .. grid[row].len()-1 {
        placePieceMatchingLeft(tilePool, grid, row, i);
    }
}

fn placePieceMatchingTopAndLeft(mut tilePool: &mut HashMap<usize, Tile>, mut grid: &mut Vec<Vec<Tile>>, row: usize, col: usize){
    assert!(!grid[row-1][col].is_empty());
    let edgeToMatch = &getEdges(&grid[row-1][col])[2];

    let mut edgeNums = getEdgeNums(&tilePool);
    assert!(edgeNums[edgeToMatch] == 2);

    let mut matchedNum = 0;

    for mut piece in &mut *tilePool{
        let mut pieceEdges = getEdges(&piece.1);
        
        for (j, mut edge) in pieceEdges.iter().enumerate() {
            if *edge == *edgeToMatch{
                println!("Matched no flip! {}",j);
                grid[row][col] = flipAndRotatePiece(&mut piece.1, 4-j, false, false);
                // grid[row][col] = (*piece.1).clone();
                matchedNum = *piece.0;
                break;
            }
            else {
                let mut rev = edge.clone();
                rev.reverse();

                if rev == *edgeToMatch{
                    println!("Matched flipped! {}",j);

                    grid[row][col] = flipAndRotatePiece(&mut piece.1, 4-j, false, true);
                    // grid[row][col] = (*piece.1).clone();
                    matchedNum = *piece.0;
                    break;
                }
            }
        }
        if matchedNum != 0 {
            break;                
        }
    }

    assert!(matchedNum != 0);
    if matchedNum != 0 {
        tilePool.remove(&matchedNum);
    }
    else {
        println!("No match found");
    }
}

fn placePieceMatchingLeft(mut tilePool: &mut HashMap<usize, Tile>, mut grid: &mut Vec<Vec<Tile>>, row: usize, col: usize){
    let edgeNums = getEdgeNums(&tilePool);
    
    assert!(!grid[row][col-1].is_empty());
    let edgeToMatch = &getEdges(&grid[row][col-1])[3];
    let mut matchedNum = 0;

    for mut piece in &mut *tilePool{
        let mut pieceEdges = getEdges(&piece.1);

        for (j, mut edge) in pieceEdges.iter().enumerate() {


            if *edge == *edgeToMatch{
                print!("no flip! Matched {}: ",j);
                for e in &pieceEdges{
                    print!("{}", edgeNums[e])
                }
                print!(" ===> ");

                let flipped = flipAndRotatePiece(&piece.1, 5-j, false, false);

                grid[row][col] = flipped;

                for e in getEdges(&grid[row][col]) {
                    print!("{}", edgeNums[&e])
                }

                if edgeNums[&getEdges(&grid[row][col])[2]] == 1 {
                    let twiceFlipped = flipAndRotatePiece(&grid[row][col], 2, false, false);
                    
                    if grid[row][col] == twiceFlipped{
                        print!(" xxx ");
                    }

                    grid[row][col] = twiceFlipped;


                    print!(" ===> ");
                    for e in getEdges(&grid[row][col]) {
                        print!("{}", edgeNums[&e])
                    }
                }



                print!("\n");
                // grid[row][col] = (*piece.1).clone();
                matchedNum += 1;//= *piece.0;
                // matchedNum = *piece.0;
                break;
            }
            else {
                let mut rev = edge.clone();
                rev.reverse();

                if rev == *edgeToMatch{
                    print!("flipped! Matched {}: ",j);
                    for e in &pieceEdges{

                        print!("{}", edgeNums[e])
                    }
                    print!(" ===> ");
                    grid[row][col] = flipAndRotatePiece(&piece.1, 5-j, true, false);
                    for e in getEdges(&grid[row][col]) {
                        print!("{}", edgeNums[&e])
                    }

                    if edgeNums[&getEdges(&grid[row][col])[2]] == 1 {
                        grid[row][col] = flipAndRotatePiece(&grid[row][col], 2, true, true);
                        print!(" ===> ");
                        for e in getEdges(&grid[row][col]) {
                            print!("{}", edgeNums[&e])
                        }
                    }

                    print!("\n");
                    // grid[row][col] = (*piece.1).clone();
                matchedNum += 1;//= *piece.0;
                // matchedNum = *piece.0;
                    break;
                }
            }
        }
        // if matchedNum != 0 {
        //     break;                
        // }
    }
    assert!(matchedNum != 0);
    if matchedNum != 0 {
        println!("matched {} tiles", matchedNum);
        // tilePool.remove(&matchedNum);
    }
    else {
        println!("No match found");
    }
}

fn flipAndRotatePiece(tile: &Tile, mut numRotations: usize, flipV: bool, flipH: bool) -> Tile {
    let mut retTile = tile.clone();

    while numRotations % 4 > 0{
        retTile = rotate(&retTile);
        numRotations -= 1;
    }

    if flipV{
        retTile = flipVertical(&retTile);
    }
    if flipH{
        retTile = flipHorizontal(&retTile);
    }
    return retTile;
}

fn placeTopLeftCorner(mut tiles: &mut HashMap<usize, Tile>, edgeNums: &HashMap<Edge, usize>, mut grid: &mut Vec<Vec<Tile>>)
{
    let startNum = getFirstCorner(tiles, &edgeNums);
    let mut start = tiles[&startNum].clone();
    while edgeNums[ &getEdges(&start) [0] ] != 1
       || edgeNums[ &getEdges(&start) [1] ] != 1  {
        start = rotate(&start);
    }
    grid[0][0] = start;
    // println!("StartNum: {}",startNum);
    tiles.remove(&startNum);
}

fn getFirstCorner(tiles: &mut HashMap<usize, Tile>, edgeNums: &HashMap<Edge, usize>) -> usize {
    for t in tiles {
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
            return *t.0;
        }
    }
    unreachable!();
}

fn generateSquareGrid() -> Vec<Vec<Tile>> {
    let mut grid = Vec::<Vec<Tile>>::new();
    
    for _i in 0 .. GRID_SIZE{
        let mut row = Vec::<Tile>::with_capacity(GRID_SIZE);
        row.resize(GRID_SIZE, Tile::new());
        grid.push(row);
    }
    return grid;
}

fn flipVertical(tile: &Tile) -> Tile {
    let mut retTile = tile.clone();
    retTile.reverse();
    return retTile;


    // let mut retTile: Tile = vec![ vec![ '.'; tile.len()]; tile.len()];
    
    // for (j,row) in tile.iter().enumerate() {
    //     for (i,cell) in row.iter().rev().enumerate() {
    //         retTile[i][tile.len() -1 -j] = tile[i][j];
    //     }
    // }
    // return retTile;
}

fn flipHorizontal(tile: &Tile) -> Tile {
    let mut retTile = tile.clone();
    
    for mut row in &mut retTile{
        row.reverse();
    }
    return retTile;

    // let mut retTile: Tile = vec![ vec![ '.'; tile.len()]; tile.len()];
    
    // for (j,row) in tile.iter().enumerate() {
    //     for (i,cell) in row.iter().rev().enumerate() {
    //         retTile[tile.len() -1 -i][j] = tile[i][j];
    //     }
    // }
    // return retTile;
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

fn getSingleEdgePieces(tiles: &HashMap<usize, Tile>, edgeNums: &HashMap<Edge, usize>) -> HashMap<usize, Tile>{
    let mut edgeNonCorners = HashMap::<usize, Tile>::new();

    for t in tiles {
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
        if numUnmatched == 1{
            edgeNonCorners.insert(*t.0, (*t.1).clone());
        }
    }
    return edgeNonCorners
}

fn getCornerPieces(tiles: &HashMap<usize, Tile>, edgeNums: &HashMap<Edge, usize>) -> HashMap<usize, Tile>{
    let mut corners = HashMap::<usize, Tile>::new();

    for t in tiles {
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
            corners.insert(*t.0, (*t.1).clone());
        }
    }
    return corners
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
        r.push(row[TILE_SIZE-1]);
    }

    // RH rule
    let mut top = t[0].clone();
    top.reverse();
    r.reverse();

    edges.push(top);
    edges.push(l);
    edges.push(t[TILE_SIZE-1].clone());
    edges.push(r);

    return edges;
}




