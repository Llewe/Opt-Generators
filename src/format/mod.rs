pub mod qubo;

pub trait Format {
    fn to_string(&self) -> String;
    fn from_json() -> Self;

    fn random(size:usize, seed: u64) -> Self;
}