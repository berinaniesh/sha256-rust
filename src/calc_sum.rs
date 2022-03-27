use super::constants;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Error;

pub fn calc(argument: &String) -> String {
    let file_result: Result<File, Error> = File::open(Path::new(argument));
    
    "Hello".to_string()
}