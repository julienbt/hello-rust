/// Integers addition
pub fn add(a: i32, b: i32) -> i32
{
    a + b
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_add()
    {
        use super::add;
        assert_eq!(add(1, 1), 2);
    }
}
