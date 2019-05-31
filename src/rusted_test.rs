use rusted::RustedList;

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        let rusted = RustedList::new();
        rusted.insert(5);
        rusted.insert(1);
        assert_eq!(rusted.peek(0), 1);
    }
}
