use std::io;

fn main() {
    let sum = sum(5, 5);
    let divided = divide_by(10, 2);
    println!("{}", sum);
    println!("{}", divided);
}

fn sum(num: i32, num2: i32) -> i32 {
    num + num2
}

fn divide_by(num: i32, divisor: i32) -> i32 {
    num / divisor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn punds_to_kilos() {
        assert_eq!(1, 1);
    }

    #[test]
    fn verify_sum() {
        assert_eq!(10, sum(5, 5));
    }

    #[test]
    fn verify_divide_by() {
        let divisor = 2;
        let num = 10;
        assert_eq!(5, divide_by(num, divisor));
    }

    #[test]
    fn create_comand() -> io::Result<()> {
        io::stdin();
        Ok(())
    }
}
