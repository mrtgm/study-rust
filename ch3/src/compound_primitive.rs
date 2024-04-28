pub fn compound_primitive() {
    // tuple
    let t = (1, 2, 3);
    let (a, b, c) = t;
    assert_eq!(a, 1);
    assert_eq!(t.1, 2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compound_primitive() {
        compound_primitive();
    }
}
