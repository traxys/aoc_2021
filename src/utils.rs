pub fn split2<'i, 'p>(input: &'i str, pattern: &'p str) -> Option<(&'i str, &'i str)> {
    let idx = input.find(pattern)?;
    Some((&input[..idx], &input[idx + pattern.len()..]))
}

#[cfg(test)]
mod test {
    use super::split2;

    #[test]
    fn split2_test() {
        assert_eq!(split2("a,b -> c,d", "->").unwrap(), ("a,b ", " c,d"))
    }
}
