pub mod three;
pub mod two;

// enum for canonical orientations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orient {
    Negative,
    Positive,
    Zero,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
