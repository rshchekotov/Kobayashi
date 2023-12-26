use std::fmt::{Display, Formatter, self};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum ChannelType {
    Text,
    Voice
}

impl FromStr for ChannelType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Text" => Ok(ChannelType::Text),
            "Voice" => Ok(ChannelType::Voice),
            _ => Err(())
        }
    }
}

impl Display for ChannelType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ChannelType::Text => "Text",
            ChannelType::Voice => "Voice",
        })
    }
}

pub struct Channel {
    pub identifier: String,
    pub channel_type: ChannelType,
    pub topic: String,
    pub nsfw: bool
}

pub struct Category {
    pub identifier: String,
    pub channels: Vec<Channel>
}
