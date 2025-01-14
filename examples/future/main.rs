mod thread1;
use thread1::*;

fn main() {
    thread_demo1();
    thread_demo2();
    send_raw_pointer();
    sync_raw_pointer();
}
