#![deny(missing_docs)]
/// Strcuture holding the vector of elements and the size of the vector for fast lookup
pub struct Rusted {
    vector: Vec<i32>,
    size: i32,
}

impl Rusted {
    /// create a new sorted list
    pub fn new() -> Rusted {
        return Rusted {
            vector: Vec::new(),
            size: 0,
        };
    }
    /// insert a new elemenet into the list by using the binary search method
    /// Complexity : O(log n)
    pub fn insert(&mut self, element: i32) {
        self.insert_element_binary(element, 0, self.size);
    }
    /// retreave the current size of the list
    pub fn get_size(&self) -> i32 {
        return self.size;
    }
    /// retreve the element at a position index by an i32.
    /// If index smaller 0 or bigger size, will return -1
    pub fn get_element_at(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }
        if index > self.size - 1 {
            return -1;
        }
        return self.vector.get(index as usize).unwrap().clone();
    }
    /// get all element as a vector
    pub fn get_list(&self) -> Vec<i32> {
        return self.vector.clone();
    }
    /// pop out the last element of the list
    pub fn pop(&mut self) -> i32 {
        self.size -= 1;
        return self.vector.pop().unwrap();
    }
    /// Remvoing any dublicates in the list
    /// Having a Complexity of O(n)
    pub fn convert_to_set(&mut self) {
        self.vector.dedup();
        self.size = self.vector.len() as i32;
    }

    fn insert_element_binary(&mut self, element: i32, min: i32, max: i32) {
        if max == 0 {
            self.size += 1;
            self.vector.push(element);
            return;
        }
        if max == 1 {
            if element > self.vector[0] {
                self.vector.push(element);
            } else {
                self.vector.insert(0, element);
            }
            return;
        }
        if max - min == 1 {
            self.size += 1;
            if element < self.vector[max as usize] {
                self.vector.insert((max) as usize, element);
            } else {
                self.vector.insert((max - 1) as usize, element);
            }
            return;
        } else {
            let middle = ((max + min) / 2) as i32;
            let middle_value = self.vector[middle as usize];
            if element < middle_value {
                self.insert_element_binary(element, min, middle + 1);
            } else {
                self.insert_element_binary(element, middle, max);
            }
        }
    }
}
