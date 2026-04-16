use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::cell::UnsafeCell;
use std::sync::Arc;

// pub fn main() {
//     let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();
//
//     let mut counter = 0;
//
//     // will not compile since threads are updating counter simultaneously
//     pool.install(|| {
//         [0..10000].par_iter().for_each(|_| counter += 1);
//     });
//     print!("counter: {counter}");
// }

// unsafe main function that will compile but also have data race issues
struct SharedCounter(UnsafeCell<usize>);
unsafe impl Sync for SharedCounter {}

pub fn main() {
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    let counter = Arc::new(SharedCounter(UnsafeCell::new(0)));

    pool.install(|| {
        (0..10_000usize).into_par_iter().for_each({
            let counter = Arc::clone(&counter);
            move |_| unsafe {
                *counter.0.get() += 1;
            }
        });
    });

    let final_count = unsafe { *counter.0.get() };
    println!("counter: {}", final_count);
}
