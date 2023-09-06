#[macro_use]
mod map_macro;


fn main() {
    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "eleven" => 11
    };

    println!("{:?}", map);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_hash_map() {
        let mut map = hash_map!(2 => "two", 1 => "one", 20 => "twenty");
        assert_eq!(map.get(&1), Some(&"one"));
        assert_eq!(map.get(&2), Some(&"two"));
        assert_eq!(map.get(&20), Some(&"twenty"));
        map.remove(&20);
        assert!(!map.contains_key(&20));
    }
}
