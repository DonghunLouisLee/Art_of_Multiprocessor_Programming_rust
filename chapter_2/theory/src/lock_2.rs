use std::{thread::ThreadId, time::Duration};

use theory::Lock;

struct Lock2 {
    id: String, //identifier
    victim: Option<ThreadId>,
}

impl Lock2 {
    fn new() -> Self {
        Self {
            id: nanoid::nanoid!(5),
            victim: None,
        }
    }
}

impl Lock for Lock2 {
    fn lock(&mut self) {
        let thread_id = std::thread::current().id();
        if self.victim.is_none() {
            self.victim = Some(thread_id); //let the other go first
            return;
        } else {
            //if other exists, wait until other is done
            while self.victim.unwrap() == thread_id {
                //wait
                println!("waiting...");
            }
        }
    }

    fn unlock(&mut self) {
        todo!()
    }
}
fn main() {
    let mut lock = Lock2::new();
    let lock_ptr: *const Lock2 = &lock;

    //this weird code is needed to send our custom locks over boundaries
    unsafe {
        std::thread::spawn(move || loop {
            println!("lock 1, trying to get the lock");
            lock.lock();
            println!("lock 1, got the lock");
            std::thread::sleep(Duration::from_secs(1));
        });
        let mut lock_2 = std::ptr::read(lock_ptr);
        loop {
            println!("lock 2, trying to get the lock");
            lock_2.lock();
            println!("lock 2, got the lock");
            std::thread::sleep(Duration::from_secs(2));
        }
    }
}
