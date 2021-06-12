use std::sync::{Arc, Mutex};

use rand::Rng;

//philosopher 0 can only use chopstick 0,1
//philosopher 1 can only use chopstick 1,2
//philosopher 2 can only use chopstick 2,3
//philosopher 3 can only use chopstick 3,4
//philosopher 4 can only use chopstick 4,0
fn main() {
    println!("question1 answer is starting");
    let count = 5; //number of philosophers = number of chopsticks
    let mut chopsticks = vec![];
    for i in 0..count {
        println!("created {}th chopstick", i);
        let chopstick = Arc::new(Mutex::new(true)); //true me
        chopsticks.push(chopstick);
    }
    for i in 0..count {
        let left = i;
        let mut right = i + 1;
        if i == count - 1 {
            right = 0;
        }
        let left_lock = chopsticks.get_mut(left as usize).unwrap().clone();
        let right_lock = chopsticks.get_mut(right as usize).unwrap().clone();
        let _ = std::thread::spawn(move || {
            println!("created philosopher_{:?}", i);
            loop {
                println!("philosopher_{:?} is currently thinking", i);
                let mut rng = rand::thread_rng();
                let ran = rng.gen_range(0..3);
                std::thread::sleep(std::time::Duration::from_secs(ran)); //think
                println!("philosopher_{:?} now wants to eat something", i);
                loop {
                    //first check if philosopher can get both locks
                    let _left = left_lock.lock().unwrap();
                    let _right = right_lock.lock().unwrap();
                    //got both locks
                    //eat
                    println!("philosopher_{:?} is currently eating", i);
                    std::thread::sleep(std::time::Duration::from_secs(1)); //eat
                    break;
                }
                println!("philosopher_{:?} is done eating", i);
            }
        });
    }

    //run a loop so that this program doesn't end
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1)); //eat for 1 second
    }
}
