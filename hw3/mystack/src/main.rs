mod mystack;

fn main() {
    let stack: mystack::MyStack<i32> = mystack::MyStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("{}", stack.pop().unwrap());
    println!("{}", stack.pop().unwrap());
    println!("{}", stack.pop().unwrap());
    println!("{}", stack.pop() == None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let stack: mystack::MyStack<i32> = mystack::MyStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        stack.push(4);
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
