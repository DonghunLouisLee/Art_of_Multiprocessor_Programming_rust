use std::{thread::ThreadId, time::Duration};

use theory::Lock;
struct Lock1 {
    id: String, //identifier
    first: Option<ThreadId>,
    second: Option<ThreadId>,
    flag: [bool; 2],
}

impl Lock1 {
    fn new() -> Self {
        Self {
            id: nanoid::nanoid!(5),
            first: None,
            second: None,
            flag: [false, false],
        }
    }
}
impl Lock for Lock1 {
    fn lock(&mut self) {
        //there's only going to be two threads
        let thread_id = std::thread::current().id();
        println!(
            "lock_id: {:?}, this is the thread_id: {:?}",
            self.id, thread_id
        );
        if self.first.is_none() && self.second.is_none() {
            //then this is the first one, so take this
            println!("1");
            self.first = Some(thread_id);
            self.flag[0] = true;
            while self.flag[1] {
                //wait
                std::thread::sleep(Duration::from_secs(1));
                println!("waiting...");
            }
        } else if self.first.is_some() && self.first.unwrap() != thread_id && self.second.is_none()
        {
            println!("2");

            // only the first one is taken and this is not the first one
            // then this is the second one, so take this
            self.second = Some(thread_id);
            self.flag[1] = true;
            while self.flag[0] {
                //wait
                std::thread::sleep(Duration::from_secs(1));
                println!("waiting...");
            }
        } else {
            println!("3");
            // both taken
            // check which one is me
            if self.first.unwrap() == thread_id {
                println!("4");
                self.flag[0] = true;
                while self.flag[1] {
                    //wait
                    std::thread::sleep(Duration::from_secs(1));
                    println!("waiting...");
                }
            } else {
                println!("5");
                self.flag[1] = true;
                while self.flag[0] {
                    //wait
                    std::thread::sleep(Duration::from_secs(1));
                    println!("waiting...");
                }
            }
        }
    }

    fn unlock(&mut self) {
        todo!()
    }
}

fn main() {
    let mut lock = Lock1::new();
    let lock_ptr: *const Lock1 = &lock;

    //this weird code is needed to send our custom locks over boundaries
    unsafe {
        std::thread::spawn(move || {
            loop {
                println!("lock 1, trying to get the lock");
                lock.lock();
                println!("lock 1, got the lock");
                std::thread::sleep(Duration::from_secs(1));
            }
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

