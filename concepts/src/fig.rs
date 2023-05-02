fn fig(is_number: bool) -> Option<i32> {
    if is_number {
        Some(10)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fig() {
        assert_eq!(fig(true), Some(10));
        assert_eq!(fig(false), None);
    }
}
