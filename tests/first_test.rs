#[cfg(test)]
mod first_test {
    use lists::first::List;
    #[test]
    fn basic() {
        let mut l = List::new();
        assert_eq!(l.pop(), None);
        l.push(1);
        l.push(2);
        l.push(3);
        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), Some(2));
        l.push(4);
        l.push(5);
        assert_eq!(l.pop(), Some(5));
        assert_eq!(l.pop(), Some(4));
        assert_eq!(l.pop(), Some(1));
        assert_eq!(l.pop(), None);
    }
    #[test]
    fn peek() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        let num = l.peek();
        assert_eq!(num, Some(&3));
    }
    #[test]
    fn peek_mut() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        assert_eq!(l.peek(), Some(&3));
        let num = l.peek_mut();
        if let Some(num) = num {
            *num = 4;
        }
        assert_eq!(l.peek(), Some(&4));
    }
    #[test]
    fn long_list() {
        let mut l = List::new();
        for i in 0..100000 {
            l.push(i);
        }
        drop(l);
    }
    #[test]
    fn into_iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        let mut i = l.into_iter();
        assert_eq!(i.next(), Some(3));
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), None);
    }
    #[test]
    fn iter() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        let mut i = l.iter();
        assert_eq!(i.next(), Some(&3));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), None);
    }
    #[test]
    fn iter_mut() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        let mut i = l.iter_mut();
        while let Some(x) = i.next() {
            *x += 1;
        }
        assert_eq!(l.pop(), Some(4));
        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), None);
    }
}