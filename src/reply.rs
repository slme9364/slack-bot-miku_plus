extern crate slack;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::str;
use github;
use rnd_meal;

pub static CHANNEL: &'static str = "#random";
//pub static CHANNEL: &'static str = "#test";

//get contains keywords from doc/contains.txt
fn contains_msg(msg: &str) -> Option<usize> {
    let file_path = Path::new("doc/reply/contains.txt");
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut contain = String::new();
    let _ = match file.read_to_string(&mut contain) {
        Ok(val) => val,
        Err(_) => return None,
    };

    let contain_vec: Vec<&str> = contain.as_str().split('\n').collect();

    //find contains keywords
    for i in 0..contain_vec.len() {
        if msg.contains(contain_vec[i]) {
            return Some(i);
        }
    }
    None
}

//get reply msg from doc/reply_msg.txt
fn get_reply_msg(index: usize) -> Option<String> {
    let file_path = Path::new("doc/reply/reply_msg.txt");
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut reply = String::new();
    let _ = match file.read_to_string(&mut reply) {
        Ok(val) => val,
        Err(_) => return None,
    };
    let reply_vec: Vec<&str> = reply.as_str().split('\n').collect();
    let reply_text = reply_vec[index].replace("\\n", "\n");

    Some(reply_text)
}

fn is_meal_roulette(text_data: &str) -> bool {
    let meal_words = ["ご飯", "ごはん", "めし"];
    for word in meal_words.iter() {
        if text_data.contains("ルーレット") && text_data.contains(word) {
            return true;
        }
    }
    false
}

pub fn reply_message(cli: &mut slack::RtmClient, text_data: &str) {
    //get github contribution
    if text_data.contains("github=") {
        let user = github::get_username(text_data);
        let today_contribution = github::get_today_contribution(user);
        let send_msg = today_contribution.as_str();
        match cli.send_message(CHANNEL, send_msg) {
            Ok(_) => println!("sending_message"),
            Err(_) => println!("Error: can't send msg"),
        }
        return;
    }

    //get meal information
    if is_meal_roulette(text_data) {
        match cli.send_message(CHANNEL, "今日の食事は") {
            Ok(_) => println!("sending_message"),
            Err(_) => println!("Error: can't send msg"),
        }
        let end_txt = "です！";
        let meal = match rnd_meal::rnd_meal(text_data) {
            Some(val) => val + end_txt,
            None => return,
        };
        let meal_str = meal.as_str();
        match cli.send_message(CHANNEL, meal_str) {
            Ok(_) => println!("sending_message"),
            Err(_) => println!("Error: can't send msg"),
        }
        return;
    }


    //not use other function
    let index = match contains_msg(text_data) {
        Some(index) => index,
        None => return,
    };
    let reply_text = match get_reply_msg(index) {
        Some(text) => text,
        None => return,
    };
    let reply_text_str = reply_text.as_str();
    match cli.send_message(CHANNEL, reply_text_str) {
        Ok(_) => println!("sending_message"),
        Err(_) => println!("Error: can't send msg"),
    }
}
