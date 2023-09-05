
mod buffer;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: (self.x + other.x), y: (self.y + other.y) }
    }
}

fn main() {
    let mut vec: Vec<Point> = Vec::new();
    for i in 0..5 {
        vec.push(Point { x: i, y: 10 - i });
    }
    let mut buf: buffer::Buffer<Point> = buffer::Buffer::new(&vec);
    // 测试sum方法
    match buf.sum() {
        Some(p) => {
            println!("({}, {})", p.x, p.y);
        }
        None => {
            println!("The vec is empty");
        }
    }
    buf.add_item(Point { x: 4, y: 5 });
    // 增加一个元素后再次测试sum方法
    match buf.sum() {
        Some(p) => {
            println!("({}, {})", p.x, p.y);
        }
        None => {
            println!("The vec is empty");
        }
    }
    // 当buf是空时测试sum方法
    let new_vec: Vec<Point> = Vec::new();
    let buf: buffer::Buffer<Point> = buffer::Buffer::new(&new_vec);
    match buf.sum() {
        Some(p) => {
            println!("({}, {})", p.x, p.y);
        }
        None => {
            println!("The vec is empty");
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_sum() {
        let mut vec: Vec<Point> = Vec::new();
        for i in 0..5 {
            vec.push(Point { x: i, y: 10 - i });
        }
        let mut buf: buffer::Buffer<Point> = buffer::Buffer::new(&vec);
        // 测试sum方法
        match buf.sum() {
            Some(p) => {
                assert_eq!(p.x, 10);
                assert_eq!(p.y, 40);
            }
            None => {
                assert!(false);
            }
        }
        buf.add_item(Point { x: 4, y: 5 });
        // 增加一个元素后再次测试sum方法
        match buf.sum() {
            Some(p) => {
                assert_eq!(p.x, 14);
                assert_eq!(p.y, 45);
            }
            None => {
                assert!(false);
            }
        }
        // 当buf是空时测试sum方法
        let new_vec: Vec<Point> = Vec::new();
        let buf: buffer::Buffer<Point> = buffer::Buffer::new(&new_vec);
        match buf.sum() {
            Some(_) => {
                assert!(false);
            }
            None => {
            }
        };  
    }   
}
