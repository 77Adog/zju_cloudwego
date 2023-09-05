pub struct Buffer<T: std::ops::Add<Output = T> + Clone> {
    att: Vec<T>,
}

impl<T: std::ops::Add<Output = T> + Clone> Buffer<T> {

    pub fn new (v: &Vec<T>) -> Self {
        Buffer { att: v.clone() }
    }

    pub fn sum(&self) -> Option<T> {
        if self.att.len() == 0 {
            return None;
        }
        let mut v_iter = self.att.iter();
        let mut res: T = v_iter.next().unwrap().clone();
        loop {
            match v_iter.next() {
                None => {break;}
                Some(value) => { res = res + value.clone(); }
            }
        }
        Some(res)
    }

    pub fn add_item(&mut self, item: T) {
        self.att.push(item);
    }
}
