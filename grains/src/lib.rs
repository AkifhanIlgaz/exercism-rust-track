pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2_u64.pow(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (0..64).fold(0, |sum, num| sum + 2_u64.pow(num))

    // 2_u128.pow(64) - 1 as u64
}
