// Modular Orchestrator for Channel Infrastructure
// https://www.youtube.com/watch?v=2ISArhs6fwk

pub mod data;
pub mod structs;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "language/mochi/mochi.pest"]
pub struct MochiParser;
