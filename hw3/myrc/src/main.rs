mod my_rc;
mod my_rc1;


fn main() {
    println!("The implementation that uses arc");
    {
        let five = my_rc::Rc::new(5);
        let five1 = five.clone();

        println!("{}", five1);
        println!("{}", my_rc::Rc::strong_count(&five1));

        {
            let five2 = five1.clone();
            println!("{}", my_rc::Rc::strong_count(&five2));
        }

        println!("{}", my_rc::Rc::strong_count(&five1));
    }

    println!("The implementation that uses bare pointer");
    {
        let five = my_rc1::Rc::new(5);
        let five1 = five.clone();

        println!("{}", five1);
        println!("{}", my_rc1::Rc::strong_count(&five1));

        {
            let five2 = five1.clone();
            println!("{}", my_rc1::Rc::strong_count(&five2));
        }

        println!("{}", my_rc1::Rc::strong_count(&five1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_rc_arc() {
        let s = my_rc::Rc::new("123");
        let s1 = s.clone();

        assert_eq!(my_rc::Rc::strong_count(&s1), 2);
        assert_eq!(*s1, "123");

        {
            let s2 = s1.clone();
            assert_eq!(my_rc::Rc::strong_count(&s1), 3);
            assert_eq!(*s2, "123");
        }
        assert_eq!(my_rc::Rc::strong_count(&s1), 2);
        assert_eq!(*s, "123");
    }

    #[test]
    fn test_my_rc_bare_pointer() {
        let s = my_rc1::Rc::new("123");
        let s1 = s.clone();

        assert_eq!(my_rc1::Rc::strong_count(&s1), 2);
        assert_eq!(*s1, "123");

        {
            let s2 = s1.clone();
            assert_eq!(my_rc1::Rc::strong_count(&s1), 3);
            assert_eq!(*s2, "123");
        }
        assert_eq!(my_rc1::Rc::strong_count(&s1), 2);
        assert_eq!(*s, "123");
    }
}
