pub fn collatz(mut n: u64) -> Option<u64> {
    let mut count = 0;
    
    while n >= 1 {
        if n.is_power_of_two() {
            return Some(count + f64::log2(n as f64) as u64);
        }
        if n % 2 == 0 {
            n /= 2
        } else {
            n = n.checked_mul(3)?.checked_add(1)?;
        }
        count += 1;
    }

    match count {
        0 => None,
        _ => Some(count),
    }
}
