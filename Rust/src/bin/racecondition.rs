// use rayon::ThreadPoolBuilder;
// use rayon::prelude::*;
//
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

// dummy main function so that we can compile
pub fn main() {}
