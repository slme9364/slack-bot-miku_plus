extern crate slack;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::str;
use github;
use rnd_meal;

static CHANNEL: &'static str = "#random";
//static CHANNEL: &'static str = "#test";

//get contains keywords from doc/contains.txt
fn contains_msg(msg: &str) -> Option<usize> {
    let file_path = Path::new("doc/reply/contains.txt");
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut contain_string = String::new();
    let _ = match file.read_to_string(&mut contain_string) {
        Ok(val) => val,
        Err(_) => return None,
    };

    let contain_str = contain_string.as_str();
    let contain_vec: Vec<&str> = contain_str.split('\n').collect();

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

    let mut reply_string = String::new();
    let _ = match file.read_to_string(&mut reply_string) {
        Ok(val) => val,
        Err(_) => return None,
    };
    let reply_str = reply_string.as_str();
    let reply_vec: Vec<&str> = reply_str.split('\n').collect();
    let reply_text = reply_vec[index].replace("\\n", "\n");

    Some(reply_text)
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
    if text_data.contains("ごはんルーレット") {
        //get lunch information
        if text_data.contains("昼") {
            let text_lunch = match rnd_meal::rnd_meal_lunch() {
                Some(val) => val,
                None => return,
            };
            let text_lunch_str = text_lunch.as_str();
            match cli.send_message(CHANNEL, text_lunch_str) {
                Ok(_) => println!("sending_message"),
                Err(_) => println!("Error: can't send msg"),
            }
            return;
        }
        if text_data.contains("夜") {
            let text_dinner = match rnd_meal::rnd_meal_dinner() {
                Some(val) => val,
                None => return,
            };
            let text_dinner_str = text_dinner.as_str();
            match cli.send_message(CHANNEL, text_dinner_str) {
                Ok(_) => println!("sending_message"),
                Err(_) => println!("Error: can't send msg"),
            }
            return;
        }
    }

    //not contain "github"
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
