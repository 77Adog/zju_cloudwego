fn main() {
    // 直接运行进行测试
    let v = vec!['a', 'b', 'c', 'd', 'e'];
    let iter = v.iter().map(|a| char::from_u32((a.clone() as u32) + 1).unwrap());
    let res: Vec<char> = iter.collect();
    assert_eq!(res, vec!['b', 'c', 'd', 'e', 'f']);
}
