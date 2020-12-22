#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::collections::HashSet;
// use std::collections::HashMap;
use std::collections::LinkedList;
// use id_tree::*;

fn readLines(path :&str) -> std::vec::Vec<String>{
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            // Try the work computer's path
            println!("Error: {},\nTrying work path...",error);
            File::open("C:\\Repos\\adventofcode\\2020\\day22\\src\\input.txt").unwrap()
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
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day22\\src\\input.txt");

    let mut p1 = LinkedList::<usize>::new();
    let mut p2 = LinkedList::<usize>::new();

    for line in &v[1..] {
        if line.is_empty() {break;}
        
        p1.push_back(line.parse::<usize>().unwrap())
    }
    for line in &v[p1.len() +3..]{
        // println!("{}",line);
        p2.push_back(line.parse::<usize>().unwrap())
    }
    // assert!(p2.len() == p1.len());

    // let mut roundNum =
    while !p1.is_empty() && !p2.is_empty() {
        let v1 = p1.pop_front().unwrap();
        let v2 = p2.pop_front().unwrap();

        // println!("Player 1 plays: {}",v1);
        // println!("Player 2 plays: {}",v2);

        if v1 > v2 {
            p1.push_back(v1);
            p1.push_back(v2);
        }
        else if v2 > v1 {
            p2.push_back(v2);
            p2.push_back(v1);
        }
        else{
            println!("Tie!  I don't know what to do!");
        }
        // println!("-----");


        // println!("-- Round --");
        // println!("Player 1's deck:");// 9, 2, 6, 3, 1");
        // println!("Player 2's deck:");// 5, 8, 4, 7, 10");
    }

    let winner : &LinkedList<usize>;//::new();



    if p1.len() > p2.len(){
        winner = &p1;
    } else{
        winner = &p2;
    }

    // for c in winner {
    //     print!("{}, ",c);
    // }
    // print!("\n");

    let mut score  = 0;
    let mut modifier = 1;
    for card in winner.iter().rev() {
            //     print!("{}, ",c);
        score += modifier * card;
        modifier += 1;
    }
    println!("Part 1: {}",score);
    
    
}

fn doPartTwo(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day22\\src\\input.txt");

    let mut p1 = LinkedList::<usize>::new();
    let mut p2 = LinkedList::<usize>::new();

    for line in &v[1..] {
        if line.is_empty() {break;}
        
        p1.push_back(line.parse::<usize>().unwrap())
    }
    for line in &v[p1.len() +3..]{
        // println!("{}",line);
        p2.push_back(line.parse::<usize>().unwrap())
    }
    assert!(p2.len() == p1.len());

    let mut gameNum = 0;
    let winner = playRecGame(&mut p1, &mut p2, &mut gameNum).1;

    let mut score  = 0;
    let mut modifier = 1;
    for card in winner.iter().rev() {
        score += modifier * card;
        modifier += 1;
    }

    // assert!(17687 < score);
    assert!(32433 > score);

    println!("Part 2: {}",score);
}


// fn playGame(p1: &mut LinkedList<usize>, p2: &mut LinkedList<usize>) {
//         // let mut roundNum =
//         while !p1.is_empty() && !p2.is_empty() {
//             let v1 = p1.pop_front().unwrap();
//             let v2 = p2.pop_front().unwrap();
    
//             // println!("Player 1 plays: {}",v1);
//             // println!("Player 2 plays: {}",v2);
    
//             if v1 > v2 {
//                 p1.push_back(v1);
//                 p1.push_back(v2);
//             }
//             else if v2 > v1 {
//                 p2.push_back(v2);
//                 p2.push_back(v1);
//             }
//             else{
//                 println!("Tie!  I don't know what to do!");
//             }
//             println!("-----");
    
    
//             // println!("-- Round --");
//             // println!("Player 1's deck:");// 9, 2, 6, 3, 1");
//             // println!("Player 2's deck:");// 5, 8, 4, 7, 10");
//         }
// }

fn playRecGame(p1: &mut LinkedList<usize>, p2: &mut LinkedList<usize>, gameNum: &mut usize) -> (bool, LinkedList<usize>) {
    *gameNum += 1;
    // let mut gamesCompleted;
    // println!("=== Game {} ===", gameNum);

    let mut p1Decks = HashSet::<LinkedList<usize>>::new();
    let mut p2Decks = HashSet::<LinkedList<usize>>::new();

    let mut roundNum = 0;
    while !p1.is_empty() && !p2.is_empty() {
        roundNum += 1;
        // println!("-- Round {} (Game {}) --",roundNum, gameNum);
        let repeat = !(p1Decks.insert(p1.clone()) && p2Decks.insert(p2.clone()));
        if repeat{
            return (true, p1.clone()); // p1 win
        }

        let v1 = p1.pop_front().unwrap();
        let v2 = p2.pop_front().unwrap();

        //             // println!("Player 1's deck:");// 9, 2, 6, 3, 1");
//             // println!("Player 2's deck:");// 5, 8, 4, 7, 10");
        // println!("Player 1 plays: {}",v1);
        // println!("Player 2 plays: {}",v2);

        let p1wonRound : bool;
        if v1 <= p1.len() && v2 <= p2.len() {
            let mut subDeck1 = createSubDeck(p1, v1);
            let mut subDeck2 = createSubDeck(p2, v2);
            p1wonRound = playRecGame(&mut subDeck1, &mut subDeck2, gameNum).0;
            *gameNum -= 1;
            // continue;
        }
        else{
            p1wonRound = v1 > v2;
        }

        if p1wonRound {
            p1.push_back(v1);
            p1.push_back(v2);
        }
        else {
            p2.push_back(v2);
            p2.push_back(v1);
        }
    }

    if p1.len() > p2.len(){
        return (true, p1.clone());
    } else{
        return (false, p2.clone());
    }
}

fn createSubDeck(deck: &LinkedList<usize>, size: usize) -> LinkedList<usize>{
    let mut subDeck = LinkedList::<usize>::new();

    // println!("subdeck size: {}", size);
    // print!("Deck: ");
    for card in deck{
        if subDeck.len() >= size{
            return subDeck;
        }
        // print!("{}, ", card);
        subDeck.push_back(card.clone());
    }
    // print!("\n");
    subDeck
}