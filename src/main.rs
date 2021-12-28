mod lexer;
mod stack;
mod ascii_table;

use std::collections::HashMap;
use std::env;
use std::io::Write;
use crate::stack::Stack;

struct Input {
    stack: Stack<i64>,
    fn_var_hashmap: HashMap<String, Vec<i64>>,
}

#[derive(PartialEq)]
enum RunType {
    Repl,
    File {
        file_name: String,
    },
}

fn main() {
    
    // Get the filename from a command line argument
    let command_line_argument = &env::args().nth(1);

    if is_argument(&command_line_argument.as_ref().unwrap()){
        run(is_repl_or_filepath(&command_line_argument.as_ref().unwrap()));
    }

}

fn is_argument(argument: &String) -> bool {
    if argument.is_empty(){
        panic!("No argument provided");
    }
    return true;
}

fn is_repl_or_filepath(argument: &String) -> RunType {
    if argument.to_lowercase() == "repl"{
        return RunType::Repl;
    } else {
        return RunType::File { file_name: argument.to_string() };
    }
}

fn run(run_type: RunType){

    if run_type == RunType::Repl{
        run_repl();
    } else if let RunType::File { file_name } = run_type {
        run_file(&file_name);
    }

}

fn run_readable_file(file: &String) {
    if can_be_read(file){
        run_file(file);
    }
}

fn run_repl() {
    println!("SBL REPL");

    loop {
        println!(">>");
        run_line();
    }
}

fn run_line() {
    let mut input_output_data:Input = Input {stack: Stack::new(), fn_var_hashmap: HashMap::new()};

    let mut lexer = lexer::Lexer::new(&get_input(), &mut input_output_data.stack, &mut input_output_data.fn_var_hashmap);
    lexer.lex();
}

fn can_be_read(file: &String) -> bool {

    if std::fs::read_to_string(file).is_ok() {
        return true;
    }
    panic!("Could not open/read file");
}

fn run_file(file: &String){
    let mut input_output_data:Input = Input {stack: Stack::new(), fn_var_hashmap: HashMap::new()};
    let mut lexer = lexer::Lexer::new(file, &mut input_output_data.stack, &mut input_output_data.fn_var_hashmap);

    lexer.lex();
}

fn get_input() -> String{

    let mut input = String::new();

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    return input;
}