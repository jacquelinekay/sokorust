#[test]
fn lel() {
    assert!(true);
}

pub struct Data<T> {
    items: ::std::vec::Vec<T>,
    indices: ::std::vec::Vec<usize>,
}


impl<T> Data<T> {
    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn push(&mut self, item: T) {
        self.items.push(item)
    }

    fn new() -> Self {
        Self {
            items: Vec::new(),
            indices: Vec::new(),
        }
    }
}

#[test]
fn t0() {
    let mut d = Data::<i32>::new();
    assert!(d.is_empty() == true);
    
    d.push(100);
    assert!(d.is_empty() == false);
}