#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::collections::HashSet;
use std::collections::HashMap;
// use id_tree::*;

fn readLines(path :&str) -> std::vec::Vec<String>{
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            // Try the work computer's path
            println!("Error: {},\nTrying work path...",error);
            File::open("C:\\Repos\\adventofcode\\2020\\day21\\src\\input.txt").unwrap()
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

struct Recipe {
    pub all : HashSet<String>,
    pub ing : HashSet<String>,
}

fn main() {
    // let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day18\\src\\input.txt");
    doPartOne();
    doPartTwo();
}

fn doPartOne(){
    let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day21\\src\\input.txt");

    let mut allergens = HashSet::<String>::new();
    let mut ingredients = HashSet::<String>::new();

    let mut recipes = Vec::<Recipe>::new();

    for line in &v {
        let split : Vec<&str> = line.split(" (contains ").collect();

        let mut ing = HashSet::<String>::new();
        for i in split[0].split(" ").collect::<Vec<&str>>() {
            ing.insert(i.to_string());
            ingredients.insert(i.to_string());
        }

        assert!(split.len() == 2);
        // println!("{:?}", split[1]);


        let mut all = HashSet::<String>::new();
        for a in split[1].split(" ").collect::<Vec<&str>>() {

            // println!("{}", &a[0..a.len()-1].to_string());
            all.insert(a[0..a.len()-1].to_string());
            allergens.insert(a[0..a.len()-1].to_string());
        }

        recipes.push(Recipe{ all :all, ing :ing});
    }   
    getHypoallergenic(&mut recipes, &allergens, &ingredients);
}

fn doPartTwo(){
    //     let v = readLines("C:\\Users\\Kerry\\coding\\aoc2020\\day20\\src\\input.txt");

    //     testRotate();

    //     let mut tiles = readTiles(&v);
    //     let _edgeNums = getEdgeNums(&tiles);

    //     placeTiles(&mut tiles);


    //     // println!("Part 1: {}", prod);
}

fn getHypoallergenic(recipes: &mut Vec<Recipe>, allergens: &HashSet<String>, ingredients: &HashSet<String>, ){

    
    let mut allIng = identifyAllergens(&mut recipes, &allergens, &ingredients);




    for (k,v) in &allIng {
        println!("{}. {:?}", k, v);
    }

}

fn identifyAllergens(recipes: &mut Vec<Recipe>, allergens: &HashSet<String>, ingredients: &HashSet<String>, ) -> HashMap<String, String> {
    let mut allIngredientsSet = HashSet::<String>::new();
    for i in ingredients{
        allIngredientsSet.insert(i.to_string());
    }

    let possibleAllergens = HashMap::<String, HashSet<String>>::new();
    for a in allergens{
        // println!("{}", a);
        possibleAllergens.insert(a.to_string(), allIngredientsSet.clone());
    }

    for r in recipes{
        for a in &r.all{
            *possibleAllergens.get_mut(a).unwrap() =  possibleAllergens[a].intersection(&r.ing).cloned().collect();
        }
    }

    for (k,v) in &possibleAllergens{
        println!("{}. {:?}", k, v);
    }

    // reduceAllergensToSingleIngredient(&mut possibleAllergens);

    // get single ingredient out of list 
    return HashMap::<String, String>::new()
    // return possibleAllergens
}

// fn reduceAllergensToSingleIngredient(hypo: &HashMap::<String, HashSet<String>>){
//     let mut retHypo = HashMap::<String, String>::new();
    
    
//     let mut allergenHasMultiple = true;
//     while allergenHasMultiple {
//         allergenHasMultiple = false;
//         let mut firstSingleInd = "".to_string();
//         let mut firstHypo = "".to_string();

//         for (k,v) in &*hypo {
//             if v.len() >= 1{
//                 allergenHasMultiple = true;
//             }
//             else {
//                 retHypo.insert(k.to_string(), v.iter().last().unwrap().to_string());
//                 // firstSingleInd = k.to_string().clone();
//                 break;
//             }
//         }

//         for (k, v) in &retHypo {
//             for (j, u) in &mut *hypo {
//                 if *j == *k{
//                     continue;
//                 }
//                 if u.len() > 1 && u.contains(k) {
//                     u.remove(v);
//             }


//                 // if *k == firstSingleInd{
//                 //     continue;
//                 // }
//                 // if v.len() > 1 && v.contains(k) {
//                 //     v.remove(k); //hypo.get_mut(k).unwrap().remove(k);
//                 // }
//                 // if firstSingle==*k{
//                 //     continue;
//                 // }
//                 // hypo.insert(*k, v.difference(&hypo[k]).cloned().collect());

//             }
//         }



//     }
// }