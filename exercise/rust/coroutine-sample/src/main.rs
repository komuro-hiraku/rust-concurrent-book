use std::{sync::{Mutex, Arc}, task::Context};
use futures::{FutureExt, task::{ArcWake, waker_ref}, future::BoxFuture};

mod future;
use future::Hello;


struct Task {
    hello: Mutex<BoxFuture<'static, ()>>
}


impl Task {
    fn new() -> Self {
        let hello = Hello::new();
        Task {
            hello: Mutex::new(hello.boxed())
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

fn main() {
    // 初期化
    let task = Arc::new(Task::new());
    let waker = waker_ref(&task);
    let mut ctx = Context::from_waker(&waker);
    let mut hello = task.hello.lock().unwrap();

    // 停止と再開の繰り返し
    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
    hello.as_mut().poll(&mut ctx);
}
