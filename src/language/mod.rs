mod mochi;

use std::fs;

use pest::Parser;
use mochi::Rule;
use mochi::data::*;

pub fn demo_mochi() {
    let input_data = fs::read("mochi/wip.mochi");
    let buffer = input_data.expect("Could not read file.");
    let decoded_str = String::from_utf8(buffer).expect("Could not decode file data to UTF-8.");

    let result = mochi::MochiParser::parse(Rule::file, decoded_str.as_str());

    match result {
        Err(e) => println!("{:?}", e),
        Ok(data) => {
            let categories = parse_categories(data.clone());
            for category in categories {
                println!("╭{}", category.identifier);
                for channel in category.channels {
                    println!("├┬ {}{} Channel '{}'",
                        if channel.nsfw { "[NSFW] " } else { "" },
                        channel.channel_type,
                        channel.identifier);
                    if let Some(private_list) = channel.private {
                        if private_list.is_empty() {
                            println!("│╰── Topic: {}", channel.topic);
                            continue;
                        }

                        let len = private_list.len();
                        println!("│├── Topic: {}", channel.topic);
                        if len > 1 {
                            for i in 0..(len-1) {
                                println!("│├── Accessible by: {}", private_list.get(i).unwrap());
                            }
                        }
                        println!("│╰── Accessible by: {}", private_list.get(len - 1).unwrap());
                    } else {
                        println!("│╰── Topic: {}", channel.topic);
                    }
                }
                println!("╰──────")
            }
        }
    }
}
