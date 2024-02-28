pub fn is_prime_erat(v: Vec<usize>) -> Vec<usize> {
    let mut pool = vec![true; v.len() - 2];

    for i in 0..pool.len() {
        if !pool[i] {
            continue;
        }

        let mut jumper = 2 * i + 2;

        while jumper < pool.len() {
            pool[jumper] = false;
            jumper += i + 2;
        }
    }

    pool.into_iter()
        .enumerate()
        .filter(|(_, x)| *x)
        .map(|(i, _)| i + 2)
        .collect()
}

pub fn is_prime_trial(v: Vec<usize>) -> Vec<usize> {
    let mut primes = Vec::new();

    'each_number: for n in v {
        let n_sqrt = (n as f64).sqrt() as usize + 1;
        for i in 2..n_sqrt {
            if n % i == 0 {
                continue 'each_number;
            }
        }
        primes.push(n);
    }

    primes
}
