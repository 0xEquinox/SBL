use std::ptr::null;
use crate::stack::Stack;
use crate::ascii_table;

//Creates a lexer structure that holds the input data, the stack which is borrowed from main, and the current position we are at while lexing
//Note that there is a lifetime here and that is because the in order to mutably reference this stack structure we need keep it alive in memory for the duration of the lexer
pub struct Lexer <'a>{
    stack: &'a mut Stack<i64>,
    expression_stack: Stack<String>,
    src: Vec<char>,
    pos: usize,
    ascii: [(char, i32); 216],
}

impl<'a> Lexer <'a>{

    //Constructor for the lexer
    pub fn new(input_file: String, stack: &'a mut Stack<i64>) -> Self {

        Self {
            stack,
            expression_stack: Stack::new(),
            src: input_file.chars().collect(),
            pos: 0,
            ascii: ascii_table::init(),
        }

    }

    pub fn lex(&mut self){

        while self.pos < self.src.len(){

            //Sets the current character to what ever we left it on the previous iteration
            let c = self.current_char();

            //Syntax pattern matcher for SDL
            match c {

                //Check for arithmetic operators
                '+' | '-' | '*' | '/' | '^' | '%' => {
                    if self.stack.len() >= 2 {
                        let num1 = self.stack.pop().unwrap();
                        let num2 = self.stack.pop().unwrap();
                        match c {
                            '+' => self.stack.push(num2 + num1),
                            '-' => self.stack.push(num2 - num1),
                            '*' => self.stack.push(num2 * num1),
                            '/' => self.stack.push(num2 / num1),
                            '^' => self.stack.push(num2.pow(num1 as u32)),
                            '%' => self.stack.push(num2 % num1),
                            _ => panic!("Invalid operator"),
                        }
                    } else {
                        panic!("Not enough numbers to complete the operation");
                    }
                    self.pos += 1;
                }


                //check for pop command
                '.' => {

                    self.pos += 1;

                    //Check if the next character is s(stack) or " (for printing)
                    match self.current_char() {
                        's' => {
                            self.pos += 1;

                            //todo make the printing look nicer
                            println!("{:?}", self.stack);
                        },

                        '"' => {
                            self.pos += 1;
                            let mut string = String::new();

                            while self.current_char() != '"' {
                                string.push(self.current_char());
                                self.pos += 1;
                            }
                            println!("{}", string);
                        },

                        //The default case is to just pop the stack
                        _ => {
                            //If the stack is empty then we can't pop anything
                            if !self.stack.is_empty() {
                                println!("{}", self.stack.pop().unwrap());
                            } else {
                                panic!("Stack is empty");
                            }
                        },
                    }
                },

                //Check for expressions
                '(' => {
                    self.pos += 1;
                    let mut buf:String = String::new();

                    while self.current_char() != ')' {
                        buf.push(self.current_char());
                        self.pos += 1;
                    }

                    //Push the expression to the expression stack
                    self.expression_stack.push(buf);

                }

                //check for strings
                '"' => {

                    self.pos += 1;
                    let mut buf = String::new();

                    //Loop until end of string
                    while self.current_char() != '"' {
                        let position:u32 = self.ascii.iter()
                            .position(|&x| x.0 == self.current_char())
                            .unwrap() as u32;

                        let digit_string = self.ascii[position as usize].1.to_string();

                        buf.push_str(&digit_string);

                        self.pos += 1;
                    }

                    self.pos += 1;

                    let num = buf.parse::<i64>().unwrap();

                    //Push the string to the stack as an integer
                    self.stack.push(num);

                },

                //check for numeric characters
                _ if c.is_numeric() => {

                    let mut buf = String::new();

                    //Loop until non-numeric character is found
                    while self.pos < self.src.len() && self.current_char().is_numeric() {
                        buf.push(self.current_char());
                        self.pos += 1;
                    }

                    //temp variable to hold the number
                    let num = buf.parse::<i64>().unwrap();

                    self.stack.push(num);
                },

                //Keywords
                _ if c.is_alphabetic() => {

                    self.pos += 1;

                    let mut buf = String::new();

                    buf.push(c);

                    //Loop until non-alphabetic character is found
                    while self.current_char().is_alphabetic() {
                        buf.push(self.current_char());
                        self.pos += 1;
                    }

                    //convert the String to &str
                    let maybe_keyword = &buf[..];

                    match maybe_keyword {

                        "if" => {

                            //if it is true then evaluate the true expression which we know will be the second one on the expression stack, else continue to the code after the if that will only run if false
                            if self.stack.pop().unwrap() == 0 {
                                //If the expression stack has less than 1 expressions then we know there isn't enough expressions to execute the if statement
                                if self.expression_stack.len() > 0 {
                                    self.eval_expr();
                                } else{
                                    panic!("Missing true expression");
                                }

                                //Loop until ; is found (since a true expression was evaluated we want to skip past the rest of the else statements)
                                while self.current_char() != ';' {
                                    //Check for end of file (if we are at the end of the file then we know there is an error due to the missing ;)
                                    if self.pos >= self.src.len() {
                                        panic!("Missing ';'");
                                    }
                                    self.pos += 1;
                                }

                            } else {
                                self.pos += 1; //Skip past the if to the else segment
                            }

                        },

                        //Drop the top of the stack
                        "drop" => {
                            self.stack.pop();
                            self.pos += 1;
                        },

                        //Duplicate the top of the stack
                        "dup" => {
                            let top = self.stack.pop().unwrap();
                            self.stack.push(top);
                            self.stack.push(top);
                            self.pos += 1;
                        },

                        //Swap the top two elements of the stack
                        "swap" => {
                            self.stack.swap();
                        },

                        //For loop
                        "for" => {
                            //Check if there is enough expressions to execute the for loop
                            if self.expression_stack.len() > 0 {
                                //Check that the loop was given valid parameters
                                if self.stack.len() >= 2{
                                    for _ in self.stack.pop().unwrap() .. self.stack.pop().unwrap() {
                                        self.expression_stack.push(self.expression_stack.peek().unwrap().clone());
                                        self.eval_expr();
                                    }
                                }else{
                                    panic!("Invalid for loop parameters");
                                }

                            }else{
                                panic!("Missing expression");
                            }
                        },

                        _ => {
                            panic!("Invalid keyword");
                        }
                    }
                },



                _ => {
                    self.pos += 1;
                }

            }

        }
    }

    //Gets the current character
    //This is required because c only updates at the top of the loop and when we need to check multiple characters we need a way to go ahead
    fn current_char(&self) -> char {
        return *self.src.get(self.pos).unwrap();
    }

    //Create a new lexer for the expression manipulating the current stack
    fn eval_expr(&mut self)  { Lexer::new(self.expression_stack.pop().unwrap(), &mut self.stack).lex(); }

}

