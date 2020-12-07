use std::fs;
//mod common;

pub fn read_data(filepath: String ) -> String {
    let contents = fs::read_to_string(filepath)
        .expect("Something went wrong reading the file");

    
    return contents;
}