extern crate rand;

use self::rand::Rng;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn rnd_meal_lunch() -> Option<String> {
    //open file
    let file_path = Path::new("doc/meal/lunch.txt");
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    let mut meal_string = String::new();
    let _ = match file.read_to_string(&mut meal_string) {
        Ok(val) => val,
        Err(_) => return None,
    };

    //string -> split('n') -> vector
    let meal_vec: Vec<&str> = meal_string.as_str().split('\n').collect();

    //select index by random engine
    let mut rnd = rand::thread_rng();
    let rnd_index = rnd.gen::<usize>() % meal_vec.len();

    let meal = meal_vec[rnd_index].to_string();
    Some(meal)
}

pub fn rnd_meal_dinner() -> Option<String> {
    //open file
    let file_path = Path::new("doc/meal/dinner.txt");
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    let mut meal_string = String::new();
    let _ = match file.read_to_string(&mut meal_string) {
        Ok(val) => val,
        Err(_) => return None,
    };

    //string -> split('n') -> vector
    let meal_vec: Vec<&str> = meal_string.as_str().split('\n').collect();

    //select index by random engine
    let mut rnd = rand::thread_rng();
    let rnd_index = rnd.gen::<usize>() % meal_vec.len();

    let meal = meal_vec[rnd_index].to_string();
    Some(meal)
}

pub fn rnd_cafe() -> Option<String> {
    //open file
    let file_path = Path::new("doc/meal/cafe.txt");
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    let mut cafe_string = String::new();
    let _ = match file.read_to_string(&mut cafe_string) {
        Ok(val) => val,
        Err(_) => return None,
    };

    //string -> split('n') -> vector
    let cafe_vec: Vec<&str> = cafe_string.as_str().split('\n').collect();

    //select index by random engine
    let mut rnd = rand::thread_rng();
    let rnd_index = rnd.gen::<usize>() % cafe_vec.len();

    let cafe = cafe_vec[rnd_index].to_string();
    Some(cafe)
}
