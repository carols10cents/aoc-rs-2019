fn main() {
    // for candidate in 145852..=616942 {
        // turn number into vector of digits
        // sort digits ascending and see if it's the same after
        // if so, see if there are any duplicates by putting in a set and seeing if len < 6
    // }
}

fn number_to_digits(mut num: i32) -> Vec<i32> {
    let mut v = vec![];
    while num > 0 {
        v.push(num % 10);
        num /= 10;
    }
    v.into_iter().rev().collect()
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