use std::thread;

pub fn main() {
    example1();
}

fn example1() {
    let (sender, rx) = crossbeam::channel::bounded(0);
    let (sender2, rx2) = (sender.clone(), rx.clone());

    thread::spawn(move || {
        rx2.recv().unwrap();
        sender2.send(2).unwrap();
    });

    sender.send(1).unwrap();
    rx.recv().unwrap();
}
