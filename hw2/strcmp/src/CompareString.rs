pub fn compareString(x: &str, y: &str) -> bool {
    let x_chars: Vec<char> = x.chars().collect();
    let y_chars: Vec<char> = y.chars().collect();

    let mut x_index: usize = 0;
    let mut y_index: usize = 0;

    while (x_index < x_chars.len()) && (y_index < y_chars.len()) {
        if x_chars[x_index] > y_chars[y_index] {
            return true;
        }
        if x_chars[x_index] < y_chars[y_index] {
            return false;
        }
        x_index += 1;
        y_index += 1;
    }
    if y_index == y_chars.len() && x_index < x_chars.len() {
        return true;
    }
    false
}