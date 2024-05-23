#[cfg(test)]
mod second_test {
    use lists::second::List;
    #[test]
    fn basic() {
        let mut list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        let mut i = list.iter();
        assert_eq!(i.next(), Some(&3));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), None);
        
    }
}