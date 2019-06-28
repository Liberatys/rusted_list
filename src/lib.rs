/// the sorted list mod
pub mod rusted;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insertion() {
        let mut list = rusted::Rusted::new();
        list.insert(3);
        list.insert(5);
        list.insert(3);
        list.insert(5);
        list.insert(3);
        list.insert(5);
        list.insert(6);
        list.insert(-3);
        list.pop();
        assert_eq!(list.get_list(), vec![-3, 3, 3, 3, 5, 5, 5]);
    }
}
