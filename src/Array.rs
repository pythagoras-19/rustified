pub(crate) struct Array {
    size: usize,
    elements: Vec<i64>
}

impl Array {
    pub fn new(size: usize) -> Self {
        Self { size, elements: vec![0; size] }
    }
    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn set_element(&mut self, element: Vec<i64>) {
        self.elements = element;
    }

    pub fn get_element(&self, index: usize) -> Option<i64> {
        if index < self.size {
            Some(self.elements[index])
        } else {
            None
        }
    }

    pub fn get_elements(&self) {
        let mut index = 0;
        while index < self.get_size() {
            println!("{:?}", self.get_element(index));
            index += 1;
        }
    }
 }

