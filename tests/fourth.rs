
#[cfg(test)]
mod fourth_test {
    use lists::fourth::List;
    #[test]
    fn basic() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn peek() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&1));
        list.pop();
        assert_eq!(list.peek(), Some(&2));
        list.pop();
        assert_eq!(list.peek(), Some(&3));
        list.pop();
        assert_eq!(list.peek(), None);
    }
    #[test]
    fn peek_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek_mut().unwrap(), &1);
        list.pop();
        assert_eq!(list.peek_mut().unwrap(), &2);
        list.pop();
        assert_eq!(list.peek_mut().unwrap(), &3);
        list.pop();
        assert_eq!(list.peek_mut(), None);
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut i = list.into_iter();
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), Some(3));
        assert_eq!(i.next(), None);
    }
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut i = list.iter();
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&3));
        assert_eq!(i.next(), None);
    }
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut i = list.iter_mut();
        assert_eq!(i.next(), Some(&mut 1));
        assert_eq!(i.next(), Some(&mut 2));
        assert_eq!(i.next(), Some(&mut 3));
        assert_eq!(i.next(), None);
    }
}