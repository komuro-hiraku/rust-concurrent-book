use crate::semaphore::Semaphore;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex, Condvar};

// 複数のデータをバルクでエンキューすることでロック獲得回数を減らしスループットの向上を見込める
// 少数データしかない場合、ブロックに満たないためエンキューできない事象を回避するため、一定時間後のFlushも必要
#[derive(Clone)]
pub struct Sender<T> {
    sem: Arc<Semaphore>,            // 有限性を実現するセマフォ
    buf: Arc<Mutex<LinkedList<T>>>, // キュー
    cond: Arc<Condvar>,             // 読み込み側の条件変数
}

impl<T: Send> Sender<T> {
    // 送信関数
    pub fn send(&self, data: T) {
        self.sem.wait();    // キューの最大値に到達したら待機
        let mut buf = self.buf.lock().unwrap();
        buf.push_back(data);    // enqueue
        self.cond.notify_one(); // 読み込み側の待機スレッドへ通知
    }
}

pub struct Receiver<T> {
    sem: Arc<Semaphore>,
    buf: Arc<Mutex<LinkedList<T>>>,
    cond: Arc<Condvar>,     // 読み込み側の条件変数
}

impl <T> Receiver<T> {
    
    pub fn recv(&self) -> T {
        let mut buf = self.buf.lock().unwrap();
        // キューから取り出す
        loop {
            if let Some(data) = buf.pop_front() {
                self.sem.post();    // Semaphore デクリメントと開放
                return data;
            }

            // 空の場合待機
            buf = self.cond.wait(buf).unwrap();
        }
    }
}

pub fn channel<T>(max: isize) -> (Sender<T>, Receiver<T>) {
    assert!(max > 0);   // max が0以下だったら Assertion Error
    let sem = Arc::new(Semaphore::new(max));
    let buf = Arc::new(Mutex::new(LinkedList::new()));
    let cond = Arc::new(Condvar::new());

    let tx = Sender {
        sem: sem.clone(),
        buf: buf.clone(),
        cond: cond.clone(),
    };

    let rx = Receiver {sem, buf, cond};
    (tx, rx)
}
