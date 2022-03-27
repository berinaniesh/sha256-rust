use super::constants;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::Error;

pub fn calc(argument: String) -> String {
    let file_result: Result<File, Error> = File::open(Path::new(&argument));
    match file_result {
        Ok(x) => {
            
            return "Hello".to_string();

        }
        Err(e) => {
            println!("Error for {}: {}", argument, e.to_string());
            return "".to_string();
        }
    }

    "".to_string()

}