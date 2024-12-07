use std::fs;

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read input file")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dummy() {
        assert_eq!(true, true);
    }
}