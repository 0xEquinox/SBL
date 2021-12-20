mod lexer;
mod stack;

use std::env;

fn main() {

    // Get the filename from a command line argument
    let maybe_file = env::args().nth(1);

    // If there is no argument, print an error and exit
    let file = if let Some(f) = maybe_file {
        f
    }else {
        panic!("Expected a file found: {:?}", maybe_file)
    };

    //Read file
    let maybe_content = std::fs::read_to_string(file);

    // If the file could not be read, print an error and exit
    let content = if maybe_content.is_ok() {
        maybe_content.unwrap()
    }else {
        panic!("Could not open/read file")
    };

    // Create a new lexer
    let mut lexer = lexer::Lexer::new(content);

}
