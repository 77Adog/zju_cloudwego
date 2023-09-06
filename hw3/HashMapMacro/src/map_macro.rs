macro_rules! hash_map {
    ($($key: expr => $val: expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    }
}