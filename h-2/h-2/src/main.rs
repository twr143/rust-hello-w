use crossbeam::scope;
use parking_lot::{Mutex, MutexGuard};

fn main() {
  let mut counter = Mutex::new(0);

  scope(|s| {
    for _ in 0..10 {
      s.spawn(|_| {
        for _ in 0..10 {
          let mut guard: MutexGuard<i32> = counter.lock();
          *guard += 1;
        }
      });
    }
  }).unwrap();

  let total: &mut i32 = counter.get_mut();
  println!("total = {}", *total)
}