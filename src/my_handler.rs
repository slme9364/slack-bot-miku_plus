extern crate slack;
extern crate rustc_serialize;

use self::rustc_serialize::json::Json;
use reply;

pub struct MyHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self,
                cli: &mut slack::RtmClient,
                event: Result<slack::Event, slack::Error>,
                raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?})", event, raw_json);

        if raw_json.contains("\"user\":") && raw_json.contains("\"text\":") {
            let json = match Json::from_str(raw_json) {
                Ok(val) => val,
                Err(_) => return,
            };

            let text_data = match json.find("text") {
                Some(val) => val,
                None => return,
            };

            let text_string = text_data.to_string();
            let text_str = text_string.as_str();

            reply::reply_message(cli, text_str);
        }
    }

    fn on_ping(&mut self, cli: &mut slack::RtmClient) {
        println!("on_ping");
    }

    fn on_close(&mut self, cli: &mut slack::RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &mut slack::RtmClient) {
        println!("on_connect");
        //let _ = cli.send_message(CHANNEL, "テスト運用です");
        println!("send msg");
    }
}
