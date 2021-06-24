pub trait Lock {
    fn lock(&mut self);
    fn unlock(&mut self);
}

use std::{collections::HashMap, thread::ThreadId};

pub struct Flag {
    inner: HashMap<ThreadId, bool>,
    count: usize,
}
impl Flag {
    pub fn new() -> Self {
        Self {
            inner: HashMap::default(),
            count: 0,
        }
    }

    pub fn change_value(&mut self, thread_id: ThreadId, value: bool) {
        if self.count <= 2 {
            match self.inner.entry(thread_id) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.insert(value);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(value);
                    self.count += 1;
                }
            };
        } else {
            unreachable!()
        }
    }

    pub fn get_other_value(&self, thread_id: ThreadId) -> bool {
        if self.count < 2 {
            //this means, other thread has not been initalized, so just return false
            false
        } else if self.count == 2 {
            //other thread has been, initialized
            for (id, value) in self.inner.iter() {
                if id != &thread_id {
                    return value.clone();
                } else {
                    continue;
                }
            }
            //this is unreachable!
            println!("this is unreachable");
            return true;
        } else {
            unreachable!()
        }
    }
}