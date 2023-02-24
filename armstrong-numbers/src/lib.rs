pub fn is_armstrong_number(mut num: u32) -> bool {
    let mut sum = num;

    let exp = f64::log10(num as f64).trunc() as u32 + 1; // Number of digits

    while num > 0 {
        let last_digit = num % 10;
        match sum.checked_sub(last_digit.pow(exp)) {
            Some(result) => sum = result,
            None => return false,
        }
        num /= 10;
    }

    sum == 0
}
