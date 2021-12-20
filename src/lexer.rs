use crate::stack::Stack;

pub struct Lexer {
    stack: Stack<i32>,
    src: vec<char>,
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

    fn lex(&mut self) {

        //todo: implement lexer

    }

}

