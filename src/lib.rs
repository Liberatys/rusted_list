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

    #[test]
    fn remove_dubplictes() {
        let mut list = rusted::Rusted::new();
        list.insert(3);
        list.insert(5);
        list.insert(3);
        list.insert(5);
        list.insert(3);
        list.insert(5);
        list.insert(6);
        list.insert(-3);
        list.convert_to_set();
        assert_eq!(list.get_list(), vec![-3, 3, 5, 6]);
    }

    #[test]
    fn check_median() {
        let mut list = rusted::Rusted::new();
        list.insert(3);
        list.insert(5);
        list.insert(3);
        list.insert(5);
        list.insert(3);
        list.insert(5);
        list.insert(6);
        list.insert(-3);
        assert_eq!(list.median(), 4);
    }
    #[test]
    fn check_median_even() {
        let mut list = rusted::Rusted::new();
        list.insert(3);
        list.insert(5);
        list.insert(3);
        list.insert(5);
        list.insert(8);
        list.insert(5);
        list.insert(3);
        list.insert(5);
        list.insert(6);
        list.insert(-3);
        assert_eq!(list.median(), 5);
    }
}
