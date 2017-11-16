extern crate slack;

use date;
use reply::CHANNEL;

static mut day: &'static str = "";

pub fn send_mention(cli: &mut slack::RtmClient) {
    let today_string = date::get_day_of_week();
    let today = match today_string.as_str() {
        "Mon" => "月",
        "Tue" => "火",
        "Wed" => "水",
        "Thu" => "木",
        "Fri" => "金",
        "Sun" => "土",
        "Sat" => "日",
        _ => "",
    };
    unsafe {
        if day != today {
            if today == "火" || today == "金" {
                cli.send_message(CHANNEL, "今日は燃えるゴミの日です！");
            }
            if today == "月" {
                cli.send_message(CHANNEL, "今日は燃えるゴミ以外の日です！");
            }
            day = today;
        }
    }
}
