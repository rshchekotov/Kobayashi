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
                println!("{}:", category.identifier);
                for channel in category.channels {
                    println!("|> {} Channel '{}'{}: {}",
                        if channel.nsfw { " [NSFW]" } else { "" },
                        channel.channel_type,
                        channel.identifier,
                        channel.topic);
                }
            }
        }
    }
}
