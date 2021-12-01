use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // スレッドセーフ参照カウンタ型スマートポイント
    let lock0 = Arc::new(Mutex::new(0));
    
    // 参照カウンタがインクリメントされるのみ。中身はクローンされない
    let lock1 = lock0.clone();

    let thread0 = thread::spawn(move || {
        some_func(lock0);
    });

    let thread1 = thread::spawn(move || {
        some_func(lock1);
    });

    // 待ち合わせ
    thread1.join().unwrap();
    thread0.join().unwrap();
}

fn some_func(lock: Arc<Mutex<u64>>) {
    loop {
        // ロックしないと Mutex 型の中の値は参照不可
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("{}", *val);
    }
}