use std::process::Command;
use std::str;


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

//Align the format yyyy-mm-dd
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

pub fn get_day_of_week() -> String {
    let date_cmd = match Command::new("date").output() {
        Ok(date) => date,
        Err(_) => return String::new(),
    };
    let mut date_cmd_str = match str::from_utf8(&date_cmd.stdout) {
        Ok(date) => date,
        Err(_) => return String::new(),
    };
    date_cmd_str = str::trim_matches(date_cmd_str, '\n');
    let date_cmd_split: Vec<&str> = date_cmd_str.split(' ').collect();

    date_cmd_split[0].to_owned()
}

//get date from date command
pub fn get_date() -> String {
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

//test program

#[test]
fn collect_return_month() {
    assert_eq!("01", num_month("Jan"));
}

#[test]
fn collect_return_day() {
    assert_eq!("01", num_day("1"));
    assert_eq!("12", num_day("12"))
}
