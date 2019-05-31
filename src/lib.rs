


pub struct RustedList{
    left_track : i32,
    center_track: i32,
    right_track: i32,
    value_list: Vec<i32>,
    tracker_index: (i32,i32,i32),
}

impl RustedList{
    pub fn new() -> RustedList{
        return RustedList{
            left_track: 1,
            center_track: 0,
            right_track: 0,
            value_list: Vec::new(),
            tracker_index: (0,0,0), 
        };
    }

    pub fn insert(&self){}

    pub fn remove(&self,index: i32){

    }

    pub fn peek(&self,index: i32){

    }

    pub fn sorted(&self) -> bool{
        return false;
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
