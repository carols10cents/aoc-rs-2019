fn main() {
    // for candidate in 145852..=616942 {
        // turn number into vector of digits
        // sort digits ascending and see if it's the same after
        // if so, see if there are any duplicates by putting in a set and seeing if len < 6
    // }
}

fn number_to_digits(num: i32) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_number_to_digits() {
        let number = 145852;
        let digits = number_to_digits(number);
        assert_eq!(digits, vec![1, 4, 5, 8, 5, 2]);
    }
}