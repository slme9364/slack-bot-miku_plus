use std::process::Command;
use std::str;
use date;

pub fn get_username(text: &str) -> String {
    let text_split: Vec<&str> = text.split('=').collect();
    let username_split: Vec<&str> = text_split[1].split(' ').collect();
    let username_str = username_split[0];
    let username = str::trim_matches(username_str, '\"');
    let username_string = username.to_string();
    username_string
}

fn get_contributions(username_string: String) -> Option<String> {
    let username = username_string.as_str();
    //get date
    let today_string = date::get_date();
    let today = today_string.as_str();
    println!("Today: {}", today);

    //set username
    println!("User: {}", username);

    //get_contibution
    let url = ["https://github.com/users/", username, "/contributions"].join("");
    let curl_cmd = match Command::new("curl").arg(url).output() {
        Ok(curl) => curl,
        Err(_) => return None,
    };
    let curl_cmd_str = match str::from_utf8(&curl_cmd.stdout) {
        Ok(curl) => curl,
        Err(_) => return None,
    };

    //str -> str(array)
    let curl_cmd_split: Vec<&str> = curl_cmd_str.split('\n').collect();

    //find_today_contribution
    for i in 0..curl_cmd_split.len() {
        if curl_cmd_split[i].contains(&today) {
            let today_contribution = curl_cmd_split[i].to_string();
            return Some(today_contribution);
        }
    }
    None
}

pub fn get_today_contribution(username: String) -> String {
    //get_contibution
    let contributions_string = match get_contributions(username) {
        Some(contributions) => contributions,
        None => String::new(),
    };
    let contributions_str = contributions_string.as_str();

    let return_string: String;

    //today_contribution judge
    if contributions_str.contains("data-count=\"0\"") || (contributions_str == "") {
        return_string = "今日のContributionを検知できませんでした".to_string();
    } else {
        return_string = "おめでとうございます!\n成功です!".to_string();
    }
    return_string
}
