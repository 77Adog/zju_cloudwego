mod CompareString;

fn main() {
    
}


#[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_compare_string() {
            assert_eq!(CompareString::compareString("123", "abc"), "123".cmp("abc") == std::cmp::Ordering::Greater);
            assert_eq!(CompareString::compareString("abc", "123"), "abc".cmp("123") == std::cmp::Ordering::Greater);
            assert_eq!(CompareString::compareString("123", "123"), "123".cmp("123") == std::cmp::Ordering::Greater);
            assert_eq!(CompareString::compareString("1234", "12"), "123".cmp("12") == std::cmp::Ordering::Greater);
            assert_eq!(CompareString::compareString("China", "America"), "China".cmp("America") == std::cmp::Ordering::Greater);
        }
    }
