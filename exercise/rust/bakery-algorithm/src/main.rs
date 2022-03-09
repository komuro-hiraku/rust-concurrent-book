
const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 100000;

// Volatile としてアドレスを読むマクロを定義
macro_rules! read_mem {
    ($addr: expr) => { unsafe { read_volatile($addr) }};
}

fn main() {

}
