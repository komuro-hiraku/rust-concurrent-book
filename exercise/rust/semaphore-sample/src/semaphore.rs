use std::sync::{Mutex, Condvar};

pub struct Semaphore {
    mutex: Mutex<isize>,
    cond: Condvar,
    max: isize,
}

impl Semaphore {

    pub fn new(max: isize) -> Semaphore {
        Semaphore {
            mutex: Mutex::new(0),
            cond: Condvar::new(),
            max
        }
    }

    // Instance Method
    pub fn wait(&self) {
        let mut count = self.mutex.lock().unwrap();

        // カウントが max 以上なら条件変数の wait で待機
        while *count > self.max {
            count = self.cond.wait(count).unwrap();
        }
        *count += 1;
    }

    pub fn post(&self) {
        let mut count = self.mutex.lock().unwrap();
        *count -= 1;

        // カウントが max 以下なら待機中のスレッドへ通知
        if *count <= self.max {
            self.cond.notify_one();
        }
    }

}
