use crate::stack::Stack;
use crate::ascii_table;

//Creates a lexer structure that holds the input data, the stack which is borrowed from main, and the current position we are at while lexing
//Note that there is a lifetime here and that is because the in order to mutably reference this stack structure we need keep it alive in memory for the duration of the lexer
pub struct Lexer <'a>{
    stack: &'a mut Stack<i64>,
    src: Vec<char>,
    pos: usize,
    ascii: [(char, i32); 216],
}

impl<'a> Lexer <'a>{

    //Constructor for the lexer
    pub fn new(input_file: String, stack: &'a mut Stack<i64>) -> Self {

        Self {
            stack,
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
                '+' => {
                    //Check that there are enough numbers to complete the operation
                    if self.stack.len() >= 2 {
                        let num1 = self.stack.pop().unwrap();
                        let num2 = self.stack.pop().unwrap();
                        self.stack.push(num2 + num1);
                    }else {
                        panic!("Not enough numbers to complete the operation");
                    }
                    self.pos += 1;
                },

                '-' => {
                    //Check that there are enough numbers to complete the operation
                    if self.stack.len() >= 2 {
                        let num1 = self.stack.pop().unwrap();
                        let num2 = self.stack.pop().unwrap();
                        self.stack.push(num2 - num1);
                    }else {
                        panic!("Not enough numbers to complete the operation");
                    }
                    self.pos += 1;
                },

                '*' => {
                    //Check that there are enough numbers to complete the operation
                    if self.stack.len() >= 2 {
                        let num1 = self.stack.pop().unwrap();
                        let num2 = self.stack.pop().unwrap();
                        self.stack.push(num2 * num1);
                    }else {
                        panic!("Not enough numbers to complete the operation");
                    }
                    self.pos += 1;
                },

                '/' => {
                    //Check that there are enough numbers to complete the operation
                    if self.stack.len() >= 2 {
                        let num1 = self.stack.pop().unwrap();
                        let num2 = self.stack.pop().unwrap();
                        self.stack.push(num2 / num1);
                    }else {
                        panic!("Not enough numbers to complete the operation");
                    }
                    self.pos += 1;
                },

                '^' => {
                    //Check that there are enough numbers to complete the operation
                    if self.stack.len() >= 2 {
                        let num1 = self.stack.pop().unwrap();
                        let num2 = self.stack.pop().unwrap();

                        self.stack.push(num2.pow(num1 as u32));

                    }else {
                        panic!("Not enough numbers to complete the operation");
                    }
                    self.pos += 1;
                },

                '%' => {
                    //Check that there are enough numbers to complete the operation
                    if self.stack.len() >= 2 {
                        let num1 = self.stack.pop().unwrap();
                        let num2 = self.stack.pop().unwrap();

                        self.stack.push(num2 % num1);

                    }else {
                        panic!("Not enough numbers to complete the operation");
                    }
                    self.pos += 1;
                },

                //check for pop command
                '.' => {

                    //Check if the next character is s(stack)

                    self.pos += 1;

                    if  self.current_char() == 's' {
                        self.pos += 1;
                        //print the stack
                        println!("{:?}", self.stack);
                    //If there isn't an s just pop the stack
                    }else if !self.stack.is_empty() {
                        println!("{}", self.stack.pop().unwrap());
                    }else{
                        panic!("Stack is empty");
                    }
                },

                //check for alphabetic characters
                '"' => {

                    self.pos += 1;
                    let mut buf = String::new();

                    //Loop until non-alphabetic character is found
                    while self.current_char() != '"' {
                        let position:u32 = self.ascii.iter()
                            .position(|&x| x.0 == self.current_char())
                            .unwrap() as u32;

                        let digit_string = self.ascii[position as usize].1.to_string();

                        buf.push_str(&digit_string);

                        self.pos += 1;
                    }

                    let num = buf.parse::<i64>().unwrap();

                    //Push the string to the stack as an integer
                    self.stack.push(num);

                    self.pos += 1;
                },

                //check for numeric characters
                _ if c.is_numeric() => {

                    self.pos += 1;

                    let mut buf = String::new();

                    buf.push(c);

                    //Loop until non-numeric character is found
                    while self.current_char().is_numeric() {
                        buf.push(self.current_char());
                        self.pos += 1;
                    }

                    self.stack.push(buf.parse::<i64>().unwrap());
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

}

