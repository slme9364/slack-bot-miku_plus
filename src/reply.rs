extern crate slack;

static CHANNEL: &'static str = "#test";

pub fn reply_message(cli: &mut slack::RtmClient, element: &str) {

    let contain_text: [&'static str; 8] = ["疲れ",
                                           "つかれ",
                                           "進捗",
                                           "おはよ",
                                           "たのし",
                                           "man miku_plus",
                                           "miku_plus --help",
                                           "miku_plus"];

    let reply_text: [&'static str; 8] = ["お疲れ様です、マスター♪ ",
                                         "お疲れ様です、マスター♪ ",
                                         "進捗どうですか？",
                                         "おはようございます〜\n今日も1日コーディング頑張りましょう！",
                                         "マスターが楽しいなら、私も嬉しいです♪",
                                         "対話型bot: miku_plusです\n気軽に話しかけてください♪ ",
                                         "対話型bot, miku_plusです\n気軽に話しかけてください♪ ",
                                         "マスター、お呼びでしょうか？"];

    for i in 0..8 {
        if element.contains(contain_text[i]) {
            let _ = cli.send_message(CHANNEL, reply_text[i]);
            return;
        }
    }
}
