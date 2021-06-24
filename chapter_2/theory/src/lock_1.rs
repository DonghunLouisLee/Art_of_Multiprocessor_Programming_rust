use std::{collections::HashMap, thread::ThreadId, time::Duration};

use theory::{Flag, Lock};

struct Lock1 {
    id: String, //identifier
    flag: Flag,
}

impl Lock1 {
    fn new() -> Self {
        Self {
            id: nanoid::nanoid!(5),
            flag: Flag::new(),
        }
    }
}
impl Lock for Lock1 {
    fn lock(&mut self) {
        let thread_id = std::thread::current().id();
        self.flag.change_value(thread_id, true);
        loop {
            let other_value = self.flag.get_other_value(thread_id);
            if !other_value {
                break;
            }
        }
    }

    fn unlock(&mut self) {
        let thread_id = std::thread::current().id();
        self.flag.change_value(thread_id, false);
    }
}

fn main() {
    let mut lock = Lock1::new();
    let lock_ptr: *const Lock1 = &lock;

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
