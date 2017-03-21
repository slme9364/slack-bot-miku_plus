extern crate slack;
extern crate rustc_serialize;

use self::rustc_serialize::json::Json;
use reply;

pub struct MyHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    //if user action in slack
    fn on_event(&mut self,
                cli: &mut slack::RtmClient,
                event: Result<slack::Event, slack::Error>,
                raw_json: &str) {
        println!("on_event(event: {:?}, raw_json: {:?})", event, raw_json);

        //popuko and pipimi bot id -> U453MJ2HW
        if !(raw_json.contains("\"user\":\"U453MJ2HW\"")) {
            //#test   channel id -> C45M040DA
            //if raw_json.contains("\"channel\":\"C45M040DA\"") {

            //#random channel id -> C45SC46VC
            if raw_json.contains("\"channel\":\"C45SC46VC\"") {
                //event -> JSON which user and text data include pick up
                if raw_json.contains("\"user\":") && raw_json.contains("\"text\":") {
                    //raw_json(str) -> json(JSON)
                    let json = match Json::from_str(raw_json) {
                        Ok(val) => val,
                        Err(_) => return,
                    };
                    //text data in json pick up
                    let text_data = match json.find("text") {
                        Some(val) => val,
                        None => return,
                    };
                    //text data cast json data to str
                    let text_string = text_data.to_string();
                    let text_str = text_string.as_str();

                    reply::reply_message(cli, text_str);
                }
            }
        }
    }

    //keep connect
    fn on_ping(&mut self, cli: &mut slack::RtmClient) {
        println!("on_ping");
    }

    //close connect
    fn on_close(&mut self, cli: &mut slack::RtmClient) {
        println!("on_close");
    }

    //start connect
    fn on_connect(&mut self, cli: &mut slack::RtmClient) {
        println!("on_connect");
    }
}
