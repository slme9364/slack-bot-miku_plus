extern crate slack;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::str;

static CHANNEL: &'static str = "#random";
//static CHANNEL: &'static str = "#test";

fn contains_msg(msg: &str) -> Option<usize> {
    let file_path = Path::new("contains.txt");
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

    for i in 0..contain_vec.len() {
        if msg.contains(contain_vec[i]) {
            return Some(i);
        }
    }
    None
}

fn get_reply_msg(index: usize) -> Option<String> {
    let file_path = Path::new("reply_msg.txt");
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
