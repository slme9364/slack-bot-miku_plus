extern crate slack;

static CHANNEL: &'static str = "#test";

pub fn reply_message(cli: &mut slack::RtmClient, element: &str) {
    if element.contains("疲れ") || element.contains("つかれ") {
        let _ = cli.send_message(CHANNEL, "お疲れ様です、マスター♪");
    }
    if element.contains("進捗") {
        let _ = cli.send_message(CHANNEL, "進捗どうですか？");
    }
    if element.contains("おはよ") {
        let _ = cli.send_message(CHANNEL, "おはようございます〜\n今日も1日コーディング頑張りましょう！");
    }
    if element.contains("たのし") {
        let _ = cli.send_message(CHANNEL,
                                 "マスターが楽しいなら、私も嬉しいです♪");
    }
    if element.contains("man miku_plus") || element.contains("miku_plus --help") {
        let _ =
            cli.send_message(CHANNEL,
                             "対話型bot, miku_plusです\n気軽に話かけてください♪");
    } else if element.contains("miku_plus") {
        let _ = cli.send_message(CHANNEL, "マスター、お呼びでしょうか？");
    }
}
