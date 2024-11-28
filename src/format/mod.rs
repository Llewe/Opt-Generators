pub mod qubo;
use std::string::String;
pub trait Format {
    fn to_json_str(&self) -> String;
    fn from_json_str(json_str: String) -> Self;

    fn random(size:usize, seed: u64) -> Self;
}