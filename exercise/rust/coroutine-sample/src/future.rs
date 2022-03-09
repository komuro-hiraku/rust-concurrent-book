use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Hello {
    state: StateHello
}

enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    pub fn new() -> Self {
        Hello {
            state: StateHello::HELLO,   // 初期状態
        }
    }
}

// Future Trait を実装
impl Future for Hello {

    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
        match (*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                (*self).state = StateHello::WORLD;  // WORLD 状態へ遷移
                Poll::Pending   // 再呼び出し可能
            }
            StateHello::WORLD => {
                print!("World!");
                (*self).state = StateHello::END;    // END 状態へ遷移
                Poll::Pending   // 再呼び出し可能
            }
            StateHello::END => {
                Poll::Ready(())
            }
        }
    }
}