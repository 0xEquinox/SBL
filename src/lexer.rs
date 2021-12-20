use crate::stack::Stack;

//Creates a lexer structure that holds the input data, the stack which is borrowed from main, and the current position we are at while lexing
//Note that there is a lifetime here and that is because the in order to mutably reference this stack structure we need keep it alive in memory for the duration of the lexer
pub struct Lexer <'a>{
    stack: &'a mut Stack,
    src: Vec<char>,
    pos: usize,
}

impl<'a> Lexer <'a>{

    //Constructor for the lexer
    pub fn new(input_file: String, stack: &'a mut Stack) -> Self {

        Self {
            stack,
            src: input_file.chars().collect(),
            pos: 0,
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
                        self.stack.push(num1 + num2);
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
                        self.stack.push(num1 - num2);
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
                        self.stack.push(num1 * num2);
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
                        self.stack.push(num1 / num2);
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
                        self.stack.print();
                    //If there isn't an s just pop the stack
                    }else if !self.stack.is_empty() {
                        self.stack.pop();
                    }else{
                        panic!("Stack is empty");
                    }
                },

                //check for alphabetic characters
                '"' => {

                    self.pos += 1;
                    let mut buf = String::new();

                    buf.push(c);

                    //Loop until non-alphabetic character is found
                    while self.current_char().is_alphanumeric() {
                        buf.push(c);
                        self.pos += 1;
                    }

                    //Push the string to the stack as an integer
                    self.stack.push(buf.parse::<i32>().unwrap());

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

                    self.stack.push(buf.parse::<i32>().unwrap());
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

