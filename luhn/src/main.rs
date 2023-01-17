pub fn luhn(cc_number: &str) -> bool {
    // remove whitespaces
    let digits = cc_number.replace(" ", "");
    // if less than 2 digits, then return false
    if digits.len() < 2 {
        return false;
    }

    // check whether all are numbers
    let are_numbers = digits.parse::<i64>().is_ok();
    if !are_numbers {
        return false;
    }

    let mut i:usize= digits.chars().count();
    let mut sum_undoubled: i32 = 0;
    let mut sum_doubled: i32 = 0;
    while i != 0 {
        let undoubled = match digits.chars().nth(i-1) {
            Some(char) => char,
            _ => '0'
        } as i32 - 0x30;

        sum_undoubled += undoubled;

        let doubled:i32 = if i > 1 {
            let mut calc = match digits.chars().nth(i-2) {
                Some(char) => char,
                _ => '0'
            } as i32 - 0x30;
            i -= 2;
            // double the digit
            calc *= 2;
            let calc: String = calc.to_string();
            let mut sum:i32 = 0;
            // sum the digits
            for d in calc.chars() {
                sum += d as i32 - 0x30;
            }
            sum
        } else {
            i -= 1;
            0
        };

        sum_doubled += doubled;
    }
    
    // if last digit of total = 0, then cc is valid.
    // the last digit can be checked by calculating total mod 10 = 0
    let total = sum_undoubled + sum_doubled;
    if total % 10 == 0 {
        true
    } else {
        false
    }
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {
    luhn("4223 9826 4026 9929");
}