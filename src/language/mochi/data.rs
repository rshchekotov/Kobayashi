use std::str::FromStr;

use pest::iterators::{Pair, Pairs};

use super::Rule;
use super::structs::{Channel, ChannelType, Category};

pub fn parse_categories (categories_wrapper: Pairs<'_, Rule>) -> Vec<Category> {
    let mut categories = Vec::new();
    for pair in categories_wrapper {
        if pair.as_rule() == Rule::category {
            categories.push(parse_category(pair))
        }
    }
    categories
}

pub fn parse_category (category_wrapper: Pair<'_, Rule>) -> Category {
    let mut category_data = category_wrapper.into_inner();
    let identifier_wrapper = category_data.next().expect("Identifier Wrapper expected.");
    let identifier = identifier_wrapper.into_inner().next().expect("Identifier Expected").as_str().to_string();
    let mut category = Category {
        identifier,
        channels: Vec::new()
    };
    for record in category_data {
        if record.as_rule() == Rule::channel {
            category.channels.push(parse_channel(record));
        }
    }
    category
}

pub fn parse_channel(channel_wrapper: Pair<'_, Rule>) -> Channel {
    let mut channel_data = channel_wrapper.into_inner();
    let identifier_wrapper = channel_data.next().expect("Identifier Wrapper expected.");
    let identifier = identifier_wrapper.into_inner().next().expect("Identifier Expected").as_str().to_string();
    let mut channel = Channel {
        identifier,
        channel_type: ChannelType::Text,
        private: None,
        topic: "".to_string(),
        nsfw: false
    };
    for record in channel_data {
        println!("Node: {:?}", record);
    }
    channel
}
