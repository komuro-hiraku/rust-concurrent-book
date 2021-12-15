mod semaphore;
mod channel;
use semaphore::Semaphore;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 8;
const SEM_NUM: isize = 4;
static mut CNT: AtomicUsize = AtomicUsize::new(0);

fn main() {

    // let mut v = Vec::new();
    // let sem = Arc::new(Semaphore::new(SEM_NUM));

    // for i in 0..NUM_THREADS {
    //     let s = sem.clone();
    //     let t = std::thread::spawn(move || {
    //         for _ in 0..NUM_LOOP {
    //             // Semaphore クリティカルセクションへ入る
    //             s.wait();

    //             unsafe { 
    //                 CNT.fetch_add(1, Ordering::SeqCst)
    //             };
    //             let n = unsafe {
    //                 CNT.load(Ordering::SeqCst)
    //             };
    //             println!("semaphore: i={}, CNT={}", i, n);
    //           assert!((n as isize)  <= SEM_NUM);  // n は SEM_NUM 以下のはず
    //             unsafe {
    //                 CNT.fetch_sub(1, Ordering::SeqCst)
    //             };

    //             // Semaphore をデクリメント
    //             s.post();
    //         }
    //     });
    //     v.push(t);
    // }

    // for t in v {
    //     t.join().unwrap();
    // }


    // channel
    let (tx, rx) = channel::channel(4);
    let mut v = Vec::new();

    // 受信用スレッド
    let t = std::thread::spawn(move || {
        let mut cnt = 0;
        while cnt < NUM_THREADS * NUM_LOOP {
            let n = rx.recv();
            println!("recv: n={:?}", n);
            cnt += 1;
        }
    });

    v.push(t);

    // 送信要スレッド
    for i in 0..NUM_THREADS {
        let tx0 = tx.clone();
        let t = std::thread::spawn(move || {
            for j in 0..NUM_LOOP {
                tx0.send((i, j));
            }
        });

        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}
