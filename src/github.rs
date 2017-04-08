use std::process::Command;
use std::str;


pub fn get_username(text: &str) -> String {
    let text_split: Vec<&str> = text.split(':').collect();
    let username_str = text_split[1];
    let username = str::trim_matches(username_str, '\"');
    let username_string = username.to_string();
    username_string
}

//English -> num
fn num_month(month_str: &str) -> &str {
    match month_str {
        "Jan" => "01",
        "Feb" => "02",
        "Mar" => "03",
        "Apr" => "04",
        "May" => "05",
        "Jun" => "06",
        "Jul" => "07",
        "Aug" => "08",
        "Sep" => "09",
        "Oct" => "10",
        "Nov" => "11",
        "Dec" => "12",
        _ => "0",
    }
}

fn num_day(day_str: &str) -> &str {
    match day_str {
        "1" => "01",
        "2" => "02",
        "3" => "03",
        "4" => "04",
        "5" => "05",
        "6" => "06",
        "7" => "07",
        "8" => "08",
        "9" => "09",
        _ => day_str,
    }
}

fn get_date() -> String {
    let date_cmd = match Command::new("date").output() {
        Ok(date) => date,
        Err(_) => return String::new(),
    };
    let mut date_cmd_str = match str::from_utf8(&date_cmd.stdout) {
        Ok(date) => date,
        Err(_) => return String::new(),
    };
    date_cmd_str = str::trim_matches(date_cmd_str, '\n');
    let mut date_cmd_split: Vec<&str> = date_cmd_str.split(' ').collect();

    //month Engish -> num
    date_cmd_split[1] = num_month(&date_cmd_split[1]);

    let date_string: String;
    let day: &str;

    //github date -> yyyy-mm-dd
    if date_cmd_split[2] == "" {
        day = num_day(date_cmd_split[3]);
        date_string = [date_cmd_split[6], date_cmd_split[1], day].join("-");
    } else {
        day = num_day(date_cmd_split[2]);
        date_string = [date_cmd_split[5], date_cmd_split[1], day].join("-");
    }

    date_string
}

fn get_contributions(username_string: String) -> Option<String> {
    let username = username_string.as_str();
    //get date
    let today_string = get_date();
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
        return_string = "今日のContributionはまだです".to_string();
    } else {
        return_string = "おめでとうございます!\n成功です!".to_string();
    }
    return_string
}
