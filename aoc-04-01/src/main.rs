use std::collections::HashSet;

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

fn never_decrease(list: &[i32]) -> bool {
    let mut sorted = list.to_owned();
    sorted.sort();
    sorted == list
}

fn duplicate_digits(list: &[i32]) -> bool {
    let digits: HashSet<_> = list.into_iter().collect();
    digits.len() < list.len()
}

fn possible_password(num: i32) -> bool {
    false
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

    #[test]
    fn can_tell_when_digits_never_decrease() {
        assert!(never_decrease(&[1, 2]));
        assert!(never_decrease(&[1, 1]));
        assert!(!never_decrease(&[2, 1]));
    }

    #[test]
    fn can_tell_if_theres_duplicate_digits() {
        assert!(duplicate_digits(&[1, 1]));
        assert!(!duplicate_digits(&[1, 2]));
    }

    #[test]
    fn can_tell_if_a_number_is_possibly_a_password() {
        assert!(possible_password(111111));
        assert!(!possible_password(223450));
        assert!(!possible_password(123789));
    }
}
