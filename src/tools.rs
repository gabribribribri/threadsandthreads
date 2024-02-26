pub fn is_prime_erat(x: usize) -> bool {
    let mut pool: Vec<bool> = vec![true; x - 1];

    for i in 2..x {
        if !pool[i - 2] {
            continue;
        }

        let mut runner = 2 * i;
        while runner < pool.len() + 2 {
            pool[runner - 2] = false;
            runner += i
        }
    }
    pool[x - 2]
}
