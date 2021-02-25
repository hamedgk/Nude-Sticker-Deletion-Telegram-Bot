
use rand::Rng;
use teloxide::prelude::*;
use teloxide::types::File as TgFile;
use tokio::fs::File;
use teloxide::{requests::Request, Bot};
use std::process::Command;
use nude;
use image;


type Kindam = teloxide::types::Sticker;
type Cx = UpdateWithCx<Message>;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting Hamed's Bot...");
    run().await;
}

async fn run() {
    #![allow(deprecated)]
    let bot = Bot::new("Bot Token");

    teloxide::repl(bot, |message: Cx| async move{
        // if the message is sticker accept it!
        match message.update.sticker() {
            Some(sticker_struct) => {
                
                let Kindam {ref file_id, ..} = &sticker_struct;
                let chat_id_str: &i64 = &message.chat_id();
                
                // if sticker is nude delete the message
                if is_sticker_nude(&message.bot, &file_id, &chat_id_str).await{
                    message.delete_message().send().await?;
                }

            },

            None => (),//if the message is not sticker do nothing
        }
        

        ResponseResult::<()>::Ok(())
    }).await;
    
}

async fn is_sticker_nude(_bot: &Bot, file_id_download: &String, chat_id: &i64) -> bool{
    let random_num: u32 = rand::thread_rng().gen_range(1..10000);
    let path = &format!("F:/NudeDatabase/{}-{}.png", &chat_id, &random_num);
    let mut file = File::create(&path).await.unwrap();
    let TgFile {ref file_path, .. } = _bot.get_file(file_id_download).send().await.unwrap();
    _bot.download_file(&file_path, &mut file).await.unwrap();
    
    //converting downloaded sticker into png type using magick command line tool!
    //otherwise, the nude detector can't handle the image
    convert_to_png(&path).await;
    let img = image::open(&path).expect("failed to open!");
    nude::scan(&img).is_nude()
}

async fn convert_to_png(path: &String){
    println!("{}", new_string);
    Command::new("magick")
        .arg(new_string.as_str())
        .arg(new_string.as_str())
        .spawn()
        .expect("magick command failed!");
}
