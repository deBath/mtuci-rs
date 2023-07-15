#[derive(Debug)]
struct Vector<T> {
    data: Vec<T>,
}

impl<T: std::default::Default> Vector<T> {

    pub fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }


    fn resize(&mut self, new_size: usize) where T: Clone {
        self.data.resize(new_size, Default::default());
    }


    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }


    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }


    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }


    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }


    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

}

fn main() {
    let mut vector: Vector<i32> = Vector::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);

    println!("Vector: {:?}", vector);

    let item = vector.pop();
    println!("Popped item: {:?}", item);

    let removed_item = vector.remove(0);
    println!("Removed item: {:?}", removed_item);

    let item = vector.get(0);
    println!("Item at index 0: {:?}", item);

    vector.resize(5);
    println!("Resized vector: {:?}", vector);
}
