use std::thread::*;

pub trait MultithreadedMap {
    fn map(self, threads_count: usize, callback: fn(Vec<usize>) -> Vec<usize>) -> Vec<usize>;
}

impl MultithreadedMap for Vec<usize> {
    fn map(self, threads_count: usize, callback: fn(Vec<usize>) -> Vec<usize>) -> Vec<usize> {
        let mut working_threads: Vec<JoinHandle<Vec<usize>>> = Vec::new();

        for i in 0..threads_count {
            println!("> Preparing thread {i}");
            let mut workload = Vec::new();
            let mut jumper = i;
            while jumper < self.len() {
                workload.push(self[jumper]);
                jumper += threads_count;
            }
            working_threads.push(spawn(move || callback(workload)));
            println!("> Thread {i} launched !");
        }

        let mut processed_vec = Vec::new();
        for thd in working_threads {
            processed_vec.extend(thd.join().unwrap())
        }

        processed_vec
    }
}
