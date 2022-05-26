use std::sync::{Condvar, Mutex};

pub struct Semaphore(Mutex<isize>, Condvar);

impl Default for Semaphore {
    fn default() -> Self {
        Self(Mutex::new(1), Default::default())
    }
}

impl Semaphore {
    pub fn new(count: isize) -> Self {
        Self(Mutex::new(count), Condvar::new())
    }

    pub fn acquire(&self) {
        *(self
            .1
            .wait_while(self.0.lock().unwrap(), |k| *k <= 0)
            .unwrap()) -= 1;
    }

    pub fn release(&self) {
        *(self.0.lock().unwrap()) += 1;
        self.1.notify_one();
    }

    #[allow(dead_code)]
    pub fn count(&self) -> isize {
        *(self.0.lock().unwrap())
    }
}
