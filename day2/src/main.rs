fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {    
    #[test]
    fn test_first() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_second() {
        assert_eq!(1, 1);
    }
}