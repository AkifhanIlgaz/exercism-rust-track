pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2, 3, 5];
    if n < 3 {
        return primes[n as usize];
    }

    let max = u32::MAX;
    let mut count = 2;

    for i in (7..max).step_by(2) {
        let mut is_prime = true;
        for prime in &primes {
            if i % prime == 0 {
                is_prime = false;
            }
        }

        if is_prime {
            primes.push(i);
            count += 1;
        }

        if count >= n {
            break;
        }
    }

    primes.last().unwrap().clone()
}
