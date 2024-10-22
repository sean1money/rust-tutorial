use anyhow::Result;
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
};

fn main() {
    let (mut s, mut r) = unbounded::<usize>();
    s.send(1).unwrap();
    assert_eq!(r.recv().unwrap(), 1);
}

struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(32)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) -> Result<()> {
        if self.total_receivers() == 0 {
            return Err(anyhow::anyhow!("no receiver left"));
        }

        let empty_before_insert = {
            let mut inner = self.shared.queue.lock().unwrap();
            let empty = inner.is_empty();
            inner.push_back(t);
            empty
        };

        if empty_before_insert {
            self.shared.available.notify_one();
        }

        Ok(())
    }

    pub fn total_queued_items(&self) -> usize {
        self.shared.queue.lock().unwrap().len()
    }

    pub fn total_receivers(&self) -> usize {
        self.shared.receivers.load(Ordering::SeqCst)
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            shared: self.shared.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let left = self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        if left <= 1 {
            self.shared.available.notify_all();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    cache: VecDeque<T>,
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T> {
        if let Some(t) = self.cache.pop_front() {
            return Ok(t);
        }

        let mut inner = self.shared.queue.lock().unwrap();

        loop {
            match inner.pop_front() {
                Some(t) => {
                    if !inner.is_empty() {
                        std::mem::swap(&mut self.cache, &mut inner);
                    }

                    return Ok(t);
                }
                None if self.total_senders() == 0 => return Err(anyhow::anyhow!("no sender left")),
                None => {
                    inner = self
                        .shared
                        .available
                        .wait(inner)
                        .map_err(|_| anyhow::anyhow!("lock poisoned"))?;
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        self.shared.senders.load(Ordering::SeqCst)
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.receivers.fetch_sub(1, Ordering::AcqRel);
    }
}

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared::default());
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            cache: VecDeque::with_capacity(32),
        },
    )
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("hello world".to_string()).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!(msg, "hello world");
    }

    #[test]
    fn channel_should_work_with_multiple_senders() {
        let (mut s, mut r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();

        let t = thread::spawn(move || {
            s.send("hello world".to_string()).unwrap();
        });

        let t1 = thread::spawn(move || {
            s1.send("hello world 1".to_string()).unwrap();
        });

        let t2 = thread::spawn(move || {
            s2.send("hello world 2".to_string()).unwrap();
        });

        for handle in [t, t1, t2] {
            handle.join().unwrap();
        }

        let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
        result.sort();
        assert_eq!(result, ["hello world", "hello world 1", "hello world 2"]);
    }

    #[test]
    fn receiver_should_be_blocked_when_no_message() {
        let (mut s, r) = unbounded();
        let mut s1 = s.clone();

        thread::spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                assert_eq!(idx, i);
            }

            assert!(false);
        });

        thread::spawn(move || {
            for i in 0..100usize {
                s.send(i).unwrap();
            }
        });

        thread::sleep(Duration::from_millis(1));

        for i in 100..200usize {
            s1.send(i).unwrap();
        }

        thread::sleep(Duration::from_millis(1));

        assert_eq!(s1.total_queued_items(), 0);
    }

    #[test]
    fn last_sender_drop_should_error_when_received() {
        let (s, mut r) = unbounded();
        let s1 = s.clone();

        let senders = [s, s1];
        let total = senders.len();

        for mut sender in senders {
            thread::spawn(move || {
                sender.send("hello").unwrap();
            })
            .join()
            .unwrap();
        }

        for _ in 0..total {
            r.recv().unwrap();
        }

        assert!(r.recv().is_err());
    }

    #[test]
    fn receiver_drop_should_error_when_send() {
        let (mut s1, mut s2) = {
            let (s, _) = unbounded();
            let s1 = s.clone();
            let s2 = s.clone();
            (s1, s2)
        };

        assert!(s1.send(1).is_err());
        assert!(s2.send(1).is_err());
    }

    #[test]
    fn receiver_should_be_notified_when_all_senders_exit() {
        let (s, mut r) = unbounded::<usize>();

        let (mut sender, mut receiver) = unbounded();
        let t1 = thread::spawn(move || {
            sender.send(0).unwrap();
            assert!(r.recv().is_err());
        });

        thread::spawn(move || {
            receiver.recv().unwrap();
            drop(s);
        });

        t1.join().unwrap();
    }

    #[test]
    fn channel_fast_path_should_work() {
        let (mut s, mut r) = unbounded::<usize>();
        for i in 0..10usize {
            s.send(i).unwrap();
        }

        assert!(r.cache.is_empty());
        assert_eq!(0, r.recv().unwrap());
        assert_eq!(r.cache.len(), 9);
        assert_eq!(s.total_queued_items(), 0);

        for (idx, i) in r.into_iter().take(9).enumerate() {
            assert_eq!(idx + 1, i);
        }
    }
}
