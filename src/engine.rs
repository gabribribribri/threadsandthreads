use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

pub struct Engine {
    workload: Vec<usize>,
    workdone: Arc<Mutex<Vec<usize>>>,
    workers: Vec<JoinHandle<()>>,
}

impl Engine {
    pub fn new(threads_num: usize, workload: Vec<usize>) -> Self {
        Self {
            workdone: Arc::new(Mutex::new(Vec::new())),
            workload,
            workers: Vec::with_capacity(threads_num),
        }
    }

    pub fn start(
        &mut self,
        worker: impl FnMut(Vec<usize>, Arc<Mutex<Vec<usize>>>) + Send + 'static,
    ) {
        for i in 0..self.workers.len() {
            let mut personal_workload = Vec::new();
            let mut picker = i;
            while picker < self.workload.len() {
                personal_workload.push(self.workload[picker]);
                picker += self.workers.len();
            }

            let thread_local_workdone = Arc::clone(&self.workdone);
            self.workers[i] = std::thread::spawn(|| {
                worker(personal_workload, thread_local_workdone);
            });
        }
    }

    pub fn join_all(self) {
        for thd in self.workers.iter() {
            thd.join().expect("Thread crashed ? I dunno.");
        }
    }

    pub fn get_workdone(self) -> Vec<usize> {
        (*self.workdone).lock().unwrap().clone()
    }
}
