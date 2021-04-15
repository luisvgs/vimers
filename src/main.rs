mod common;

use egg_mode::search::{self, ResultType};
use chrono::prelude::*;
use std::io::{stdin, BufRead};
use egg_mode::error::Result;

#[tokio::main]
async fn main() ->Result<()>{ 
    let config = common::Config::load().await;

    println!("User to look up:");
    let line: String = stdin().lock().lines().next().unwrap().unwrap();
    
    let rustlang = egg_mode::user::show(line, &config.token).await.unwrap();

    println!("{}, (@{})", rustlang.name, rustlang.screen_name);
    

    println!("");
    println!("Loading user's home timeline");

    let home = egg_mode::tweet::home_timeline(&config.token).with_page_size(5);
    let (_home, feed) = home.start().await?;
    for status in feed.iter() {
        common::print_tweet(&status);
        println!("");
    }


    let consumer_key = include_str!("consumer_key").trim();
    let consumer_secret = include_str!("consumer_secret").trim();

    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let token = egg_mode::auth::bearer_token(&con_token).await.unwrap();
    println!("User timeline in 3..2..1..");
    let timeline =
        egg_mode::tweet::user_timeline("vim_tricks", false, true, &token).with_page_size(5);
    
    let (_timeline, feed) = timeline.start().await?;
    for tweet in feed.response {
        println!("");
        common::print_tweet(&tweet);
    }
    Ok(())
}
