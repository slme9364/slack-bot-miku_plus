extern crate slack;
extern crate rustc_serialize;

use self::rustc_serialize::json::Json;
use reply;

pub struct MyHandler;

//static CHANNEL: &'static str = "test";

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self,
                cli: &mut slack::RtmClient,
                event: Result<slack::Event, slack::Error>,
                raw_json: &str) {
        if raw_json.contains("\"user\":") && raw_json.contains("\"text\":") {

            let json = Json::from_str(raw_json).unwrap();
            let text_string = json.find("text").unwrap().to_string();
            let text_data = text_string.as_str();

            reply::reply_message(cli, text_data);
        }
        println!("on_event(event: {:?}, raw_json: {:?})", event, raw_json);
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
