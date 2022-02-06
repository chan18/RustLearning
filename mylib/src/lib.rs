//! this is my library documentation.

/// four() is a function that returns `4`
/// ```
/// use mylib::four;
/// let x = four();
/// assert_eq!(four(), 4);
/// ```
pub fn four() -> i32 { 4 }

#[cfg(test)]
mod tests {
    use super::four;
    #[test]
    fn it_works() {
        //let result = 2 + 2;
        assert_eq!(four(), 4);
    }
}
