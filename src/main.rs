fn is_prime(x: usize) -> bool {
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

use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Instant;

macro_rules! time {
    {$to_time: expr} => {
        let timer = Instant::now();
        $to_time;
        println!("> Finished in {:?}", timer.elapsed());
    };
}

const MAX: usize = 1_000_000;
const THREADS_NUM: usize = 8;

fn main() {
    let finished_pool: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));
    let mut workers: Vec<JoinHandle<()>> = Vec::new();
    let mut workloads: [Vec<usize>; THREADS_NUM] =
        vec![Vec::with_capacity(MAX / THREADS_NUM + 2); THREADS_NUM]
            .try_into()
            .expect("Failed to convert Vec to array");

    println!("[ 1 ] Initializing workload...");
    time! {
        for i in 2..MAX {
            workloads[i%THREADS_NUM].push(i);
        }
    }

    let workloads_arc: Arc<[Vec<usize>; THREADS_NUM]> = Arc::new(workloads);

    println!("[ 2 ] Starting workload...");
    time! {
        for thd in 0..THREADS_NUM {
            let workloads_local = Arc::clone(&workloads_arc);
            let finished_pool_local = Arc::clone(&finished_pool);
            workers.push(std::thread::spawn(move || {
               for i in &workloads_local[thd] {
                    if !is_prime(*i) {continue}

                    Arc::clone(&finished_pool_local).lock().unwrap().push(*i);
                }
            }));
        }
    }

    println!("[ 3 ] Catching finished workload");
    time! {
        for thd in workers {
            let _ = thd.join();
        }
    }

    let mut results = (*finished_pool).lock().unwrap().clone();
    results.sort();
    println!("> All primes from 2 to {} sorted : {:?}", MAX, &results);
}
