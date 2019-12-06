use std::collections::HashSet;

fn main() {
    let possible_passwords: Vec<_> = (145852..=616942).into_iter().filter(|&num| possible_password(num)).collect();
    println!("There are {} passwords", possible_passwords.len());
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
    let digits = number_to_digits(num);
    never_decrease(&digits) && duplicate_digits(&digits)
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
