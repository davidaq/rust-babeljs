use std::cell::{ Cell, RefCell };
use std::collections::VecDeque;
use std::sync::Mutex;
use std_semaphore::Semaphore;
use std::marker::{ Sync, Send };

pub struct Queue<T> {
  ended: Cell<bool>,
  list: RefCell<VecDeque<T>>,
  waiting: Cell<usize>,
  syn: Mutex<u8>,
  sem: Semaphore,
  name: String,
}

unsafe impl<T> Sync for Queue<T> {}
unsafe impl<T> Send for Queue<T> {}

impl<T> Queue<T> {

  pub fn new (name: &str) -> Self {
    Queue::<T> {
      syn: Mutex::new(0),
      sem: Semaphore::new(0),
      waiting: Cell::new(0),
      ended: Cell::new(false),
      list: RefCell::new(VecDeque::with_capacity(20)),
      name: String::from(name),
    }
  }

  pub fn push (&self, item: T) {
    let lock = self.syn.lock();
    if self.ended.get() {
      panic!("Ended queue \"{}\" is read only!", self.name);
    }
    self.list.borrow_mut().push_back(item);
    self.sem.release();
  }

  pub fn interrupt (&self) {
    let lock = self.syn.lock();
    for x in 0..self.waiting.get() {
      self.sem.release();
    }
  }

  pub fn end (&self) {
    let lock = self.syn.lock();
    self.ended.set(true);
    for x in 0..self.waiting.get() {
      self.sem.release();
    }
  }

  pub fn hasMore (&self) -> bool {
    let lock = self.syn.lock();
    return self.list.borrow().len() > 0;
  }

  pub fn pop (&self) -> Option<T> {
    let mut ended = false;
    {
      let lock = self.syn.lock();
      ended = self.ended.get();
      if !ended {
        self.waiting.set(self.waiting.get() + 1);
      }
    }
    if !ended {
      self.sem.acquire();
    }
    let lock = self.syn.lock();
    if !ended {
      self.waiting.set(self.waiting.get() - 1);
    }
    return self.list.borrow_mut().pop_front();
  }
}
