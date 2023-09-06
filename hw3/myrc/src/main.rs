mod my_rc;


fn main() {
    let five = my_rc::Rc::new(5);
    let five1 = five.clone();

    println!("{}", five1);
    println!("{}", my_rc::Rc::strong_count(&five1));
}
