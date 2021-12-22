use std::fmt::Debug;

//Stack is a data structure that stores a list of elements(of given type) in a Last In First Out order
#[derive(Debug)]
pub struct Stack<T>{
    stack: Vec<T>,
}

//Methods
impl<T> Stack<T> {

    //Constructor
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
        }
    }

    //Push an element to the top of the stack
    pub fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    //Pop the top element from the stack
    pub fn pop(&mut self) -> Option<T> {
        //check that the stack is not empty
        if !self.stack.is_empty(){
            self.stack.pop()
        }else{
            panic!("Stack underflow: cannot pop from an empty stack");
        }
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
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    pub fn swap(&mut self) {
        //Top element is moved to the second spot
        let temp1 = self.stack.pop().unwrap();

        if !self.stack.is_empty() {
            //Second element is moved to the top
            let temp2 = self.stack.pop().unwrap();

            self.stack.push(temp1);
            self.stack.push(temp2);
        }else {
            self.stack.push(temp1);
        }

    }

}



