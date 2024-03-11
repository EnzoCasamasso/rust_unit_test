fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn punds_to_kilos() {
        assert_eq!(1, 1);
    }
}
