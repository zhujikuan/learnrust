pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[test]
fn mytest() {
    assert_eq!(add(2, 2), 4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn it_works2() -> Result<(), String> {
        let result = add(2, 2);
        if result == 3 {
            Ok(())
        } else {
            Err(String::from("两数相加结果不正确"))
        }
    }
}
