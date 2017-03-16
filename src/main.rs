extern crate slack;

mod reply;
mod my_handler;

fn main() {
    //set api key
    let api_key = env!("SLACK_MIKU_PLUS_API_TOKEN");
    println!("set api key:{}", api_key);

    //make client
    let mut handler = my_handler::MyHandler;
    let mut cli = slack::RtmClient::new(&api_key);
    println!("create cli");

    //Bot running
    let r = cli.login_and_run::<my_handler::MyHandler>(&mut handler);
    match r {
        Ok(_) => {}
        Err(err) => println!("Error: {}", err),
    }

    println!("{}", cli.get_name().unwrap());
    println!("{}", cli.get_team().unwrap().name);
}
