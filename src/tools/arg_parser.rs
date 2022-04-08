use std::env;

pub struct ArgParser;

impl ArgParser {
    pub fn get_params() -> Vec<String> {
        env::args().collect()
    }
}
