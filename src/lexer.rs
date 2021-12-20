use crate::stack::Stack;

pub struct Lexer {
    stack: Stack,
    src: Vec<char>,
    pos: i32,
}

impl Lexer {

    pub fn new(input_file: String) -> Self {

        Self {
            stack: Stack::new(),
            src: input_file.chars().collect(),
            pos: 0,
        }

    }

    fn peek(&self) -> char {
        self.src[self.pos as usize]
    }

    pub fn lex(&mut self) {

        while self.pos < self.src.len() as i32 {

            let c = self.src[self.pos as usize];

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
                    if self.peek() == 's' {
                        self.pos += 1;
                        self.stack.print();
                    //If there isn't an s just pop the stack
                    }else if !self.stack.is_empty() {
                        self.stack.pop();
                    }else{
                        panic!("Stack is empty");
                    }

                    self.pos += 1;
                }

                //check for alphabetic characters
                '"' => {

                    let mut buf = String::new();

                    buf.push(c);

                    //Loop until non-alphabetic character is found
                    while c.is_alphanumeric() {
                        buf.push(c);
                        self.pos += 1;
                    }

                    //Push the string to the stack as an integer
                    self.stack.push(buf.parse::<i32>().unwrap());

                }

                //todo Fix error when taking number input
                //check for numeric characters
                _ if c.is_numeric() => {

                    self.pos += 1;
                    let mut buf = String::new();

                    buf.push(c);

                    //Loop until non-numeric character is found
                    while c.is_numeric() {
                        buf.push(c);
                        self.pos += 1;
                    }

                    self.stack.push(buf.parse::<i32>().unwrap());
                }

                _ => {
                    self.pos += 1;
                }

            }

        }

    }

}

