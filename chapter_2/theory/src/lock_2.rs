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
        match self.victim {
            Some(other_thread_id) => {
                if other_thread_id == thread_id {
                    println!("i was the last one to access");
                    return;
                } else {
                    self.victim = Some(thread_id);
                    while other_thread_id == thread_id {
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
        todo!()
    }
}
fn main() {
    let mut lock = Lock2::new();
    let lock_ptr: *const Lock2 = &lock;

    //this weird code is needed to send our custom locks over boundaries
    unsafe {
        std::thread::spawn(move || loop  {
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
