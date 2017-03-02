extern crate slack;

mod reply;
mod my_handler;

fn main() {
    let api_key = env!("SLACK_MIKU_PLUS_API_TOKEN");
    println!("set api key");

    let mut handler = my_handler::MyHandler;
    let mut cli = slack::RtmClient::new(&api_key);
    println!("create cli");

    let r = cli.login_and_run::<my_handler::MyHandler>(&mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }

    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}
