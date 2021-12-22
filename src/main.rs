mod lexer;
mod stack;
mod ascii_table;

use std::env;
use std::io::Write;

fn main() {
    
    // Get the filename from a command line argument
    let argument = env::args().nth(1);
    let maybe_file = argument.clone();   
    
    //Check for repl argument
    if maybe_file.unwrap() == "repl"{
        println!("SBL REPL");

        let mut stack = stack::Stack::new();

        //Lex one line at a time with live input
        loop {
            //Create the string we will be lexing
            let mut input = String::new();
            print!(">> ");

            //Get the input
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();

            //Create a new lexer struct and lex the input string
            let mut lexer = lexer::Lexer::new(input, &mut stack);

            lexer.lex();

        }

    }

    let maybe_file = argument.clone();

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
    //Create new stack

    // Create a new lexer

    // Lex the file
}
