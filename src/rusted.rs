pub struct RustedList {
    left_track: i32,
    center_track: i32,
    right_track: i32,
    value_list: Vec<i32>,
    current_size: i32,
    center_index: i32,
}

impl RustedList {
    pub fn new() -> RustedList {
        return RustedList {
            left_track: 1,
            center_index: 0,
            current_size: 0,
            center_track: 0,
            right_track: 0,
            value_list: Vec::new(),
        };
    }

    pub fn insert(&mut self, element: i32) {
        if self.value_list.len() == 0 {
            self.value_list.push(element);
            self.left_track = element;
            self.current_size = 1;
        } else if self.value_list.len() == 1 {
            self.current_size = 2;
            if self.left_track > element {
                self.value_list.insert(0, element);
                self.right_track = self.left_track;
                self.left_track = element;
            } else {
                self.value_list.push(element);
                self.right_track = element;
            }
        } else if self.value_list.len() == 2 {
            self.current_size = 3;
            self.center_index = 1;
            if element < self.left_track {
                self.center_track = self.left_track;
                self.left_track = element;
                self.value_list.insert(0, element);
            } else if element > self.right_track {
                self.center_track = self.right_track;
                self.right_track = element;
                self.value_list.insert(2, element);
            } else {
                self.center_track = element;
                self.value_list.insert(1, element);
            }
        } else {
            self.current_size = self.current_size + 1;
            if element < self.left_track {
                self.left_track = element;
                self.value_list.insert(0, element);
                return;
            } else if element > self.right_track {
                self.right_track = element;
                self.value_list.push(element);
                return;
            }
            if element > self.center_track {
                for i in self.center_index..self.current_size - 1 {
                    if element > self.value_list[(i as usize)] {
                        // if element is bigger than the current index_value, insert the element
                        // one to the right of the current index.
                        self.value_list.insert((i + 1) as usize, element);
                        break;
                    }
                }
            } else {
                for i in 0..self.center_index {
                    if self.value_list[(i as usize)] < element {
                        self.value_list.insert((i + 1) as usize, element);
                        break;
                    }
                }
            }
            let center_set_index: i32 = (self.value_list.len() / 2) as i32;
            println!("{}", center_set_index);
            self.center_index = center_set_index;
        }
    }
    pub fn remove(&mut self, index: i32) {
        self.value_list.remove(index as usize);
    }

    pub fn peek(&self, index: i32) -> i32 {
        return self.value_list[index as usize];
    }

    pub fn sorted(&self) -> bool {
        return true;
    }

    pub fn len(&self) -> i32 {
        return self.current_size;
    }

    pub fn get_new_insert_position(&self) {}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_value() {
        let mut listing = RustedList::new();
        listing.insert(1);
        listing.insert(5);
        assert_eq!(listing.peek(0), 1);
    }

    #[test]
    fn remove_value() {
        let mut listing = RustedList::new();
        listing.insert(5);
        listing.insert(10);
        listing.remove(1);
        assert_eq!(listing.peek(0), 5);
    }

    #[test]
    fn multiple_insert() {
        let mut listing = RustedList::new();
        for i in 0..10 {
            listing.insert(i);
        }
        listing.insert(-1);
        listing.insert(4);
        listing.insert(7);
        for i in 0..listing.len() {
            println!("{}", listing.peek(i as i32));
        }
        assert_eq!(listing.len() as i32, 13);
    }
}
