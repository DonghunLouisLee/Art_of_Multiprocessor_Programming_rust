use std::{collections::HashMap, thread::ThreadId, time::Duration};

use theory::{Flag, Lock};

// starvation free lock
struct PetersonLock {
    id: String, //identifier
    victim: Option<ThreadId>,
    flag: Flag,
}

impl PetersonLock {
    fn new() -> Self {
        Self {
            id: nanoid::nanoid!(5),
            victim: None,
            flag: Flag::new(), //todod
        }
    }
}

impl Lock for PetersonLock {
    fn lock(&mut self) {
        let thread_id = std::thread::current().id();
        self.flag.change_value(thread_id, true);
        match self.victim {
            Some(other_thread_id) => {
                if other_thread_id == thread_id {
                    println!("i was the last one to access");
                    return;
                } else {
                    self.victim = Some(thread_id);
                    while self.flag.get_other_value(thread_id) && other_thread_id == thread_id {
                        //wait
                        println!("waiting...");
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            }
            //starting condition
            //only reach here once
            None => {
                self.victim = Some(thread_id);
                return;
            }
        }
    }

    fn unlock(&mut self) {
        let thread_id = std::thread::current().id();
        self.flag.change_value(thread_id, false);
    }
}

fn main() {
    let mut lock = PetersonLock::new();
    let lock_ptr: *const PetersonLock = &lock;

    //this weird code is needed to send our custom locks over boundaries
    unsafe {
        std::thread::spawn(move || loop {
            println!("lock 1, trying to get the lock");
            lock.lock();
            println!("lock 1, got the lock");
            std::thread::sleep(Duration::from_secs(3));
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
