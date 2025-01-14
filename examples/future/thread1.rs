use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub(crate) fn thread_demo1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("thread_demo1: {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("thread_demo1: {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub(crate) fn thread_demo2() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });

    handle.join().unwrap();
}

#[derive(Debug)]
struct MyBox(*mut u8);
unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}

pub(crate) fn send_raw_pointer() {
    let i = MyBox(42 as *mut u8);
    let h = thread::spawn(move || {
        println!("{:?}", i);
    });

    h.join().unwrap();
}

pub(crate) fn sync_raw_pointer() {
    let v = &MyBox(5 as *mut u8);
    let v = Arc::new(Mutex::new(v));
    let h = thread::spawn(move || {
        println!("{:?}", v.lock().unwrap().0);
    });

    h.join().unwrap();
}
