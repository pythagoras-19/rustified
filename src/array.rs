use std::collections::*;

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

    pub fn is_empty(&self) -> bool {
        if self.get_size() > 0 {
            false
        } else {
            true
        }
    }

    pub fn set_element(&mut self, index: usize, value: i64) {
        if index < self.size {
            self.elements[index] = value;
        }
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

    pub fn get_max(&self) -> Option<i64> {
        let mut max = self.get_element(0);
        let mut index = 0;

        while index < self.get_size() {
            if self.get_element(index) > max {
                max = self.get_element(index);
            }
            index += 1;
        }
        max
    }

    pub fn get_min(&self) -> Option<i64> {
        if self.elements.is_empty() {
            None
        } else {
            let mut min = self.get_element(0);
            let mut index = 0;

            while index < self.get_size() {
                if self.get_element(index) < min {
                    min = self.get_element(index);
                }
                index += 1;
            }
            min
        }
    }

    pub fn get_most_occurring(&self) -> Option<i64> {
        let mut num_times = HashMap::new();
        let mut index = 0;

        while index < self.get_size() {
            match self.get_element(index) {
                Some(value) => {
                    let count = num_times.entry(value).or_insert(0);
                    *count += 1;
                },
                None => (),
            }
            index += 1;
        }
        // convert hashmap into an iterator
        // max_by_key is provided by iterators and returns the max element of the iterator
        // |&(_, count)| count) is a closure (a small anonymous function like lambda that
        // takes a tuple as input and returns the count
        // .map() is another function provided by iterators
        // it also converts the iterator of key-val pairs into an iterator of keys
        // _ is a wildcard pattern that ignores any value that comes its way
        // _ can also be used to ignore variable values
        println!("Number of elements: {}", num_times.clone().into_iter().len());
        num_times.into_iter().max_by_key(|&(_, count)| count).map(|(val, _)| val)
    }

    pub fn get_median(&self) -> Option<f64> {
        if self.get_size() == 0 {
            None
        } else {
            let mut elements = self.elements.clone();
            elements.sort();
            let middle = self.get_size() / 2;

            if self.get_size() % 2 == 0 {
                Some((self.elements[middle - 1] as f64 + self.elements[middle] as f64) / 2.0 )
            } else {
                Some(self.elements[middle] as f64)
            }
        }
    }

    pub fn get_location(&self) {
        let loc = &self as *const _;
        println!("Memory location of arr is: {:?}", loc);
    }

    pub fn pointer_stuff(&self) {
        let x = &self.get_element(3);
        let raw = &x as *const &Option<i64>;
        let points_at = unsafe { *raw };
        println!("{:?}", points_at);
    }

    pub fn get_sum(&self) -> i64 {
        let mut sum = 0;
        for &value in &self.elements {
            sum += value;
        }
        sum
    }

    pub fn reverse(&mut self) {
        self.elements.reverse();
    }
 }

