use std::fmt::Debug;

//Stack is a data structure that stores a list of elements(of given type) in a Last In First Out order
#[derive(Debug)]
pub struct Stack{
    stack: Vec<i32>,
}

//TODO Fix the implementation of the Stack (it isn't pushing and popping elements correct and only stores 1 element at a time)

//Methods
impl<> Stack<> {

    //Constructor
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
        }
    }

    //Push an element to the top of the stack
    pub fn push(&mut self, item: i32) {
        self.stack.push(item);
    }

    //Pop the top element from the stack
    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    //Return true if the stack is empty
    pub fn is_empty(&self) -> bool {

        self.stack.is_empty()
    }

    //Return the number of elements in the stack
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    //Return the next element in the stack without removing it
    pub fn peek(&self) -> Option<&i32> {
        self.stack.last()
    }

    //prints the stack
    pub fn print(&self) {
        println!("{:?}", self.stack);
    }

}



