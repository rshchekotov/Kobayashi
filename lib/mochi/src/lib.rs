#![feature(assert_matches)]


use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "mochi.pest"]
pub struct MochiParser;

pub mod error;
pub mod discord;
pub mod util;
pub mod parse;

/* Data Structures */
#[derive(Clone, Debug)]
pub struct Role {
    pub name: String,
    pub color: u32,
    pub separate: bool,
    pub mentionable: bool,
    pub permissions: discord::Permissions,
}

#[cfg(test)]
pub mod tests;
