use std::sync::{Arc, Mutex, Condvar};
use std::thread;

fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {

    // *p でスマートポインタの実体(Tuple)で & つけてそれの単なる参照
    let &(ref lock, ref cvar) = &*p;

    // Mutex Lock
    let mut started = lock.lock().unwrap();
    while !*started {   // Mutex 内の変数が false の間ループ
        // wait で待機
        started = cvar.wait(started).unwrap();
    }
    // こっちでもOK
    // cvar.wait_while(started, |s| !*s).unwrap();
    println!("child {}", id);
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    // Get Mutex lock
    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_all();
    println!("parent");
}

fn main() {
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = pair0.clone();
    let pair2 = pair0.clone();

    let c0 = thread::spawn(move || { child(0, pair0); });
    let c1 = thread::spawn(move || { child(1, pair1); });
    let p = thread::spawn(move || { parent(pair2) });

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}
