use tokio::runtime::{Builder, Runtime};

fn main() {
    let mut rt = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("demo-thread")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .on_thread_start(|| {
            eprintln!("thread started");
        })
        .on_thread_stop(|| {
            eprintln!("thread stopped");
        })
        .on_thread_park(|| {
            eprintln!("thread parked");
        })
        .on_thread_unpark(|| {
            eprintln!("thread unparked");
        })
        .build()
        .unwrap();

    rt.block_on(async {
        eprintln!("Hello, world!");
    });
}
