use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::env;
use std::process;
use std::time::Instant;

fn index(i: usize, j: usize, n: usize) -> usize {
    i * n + j
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("usage: {} num iterations threads", args[0]);
        eprintln!("num = dimension of the square matrix");
        eprintln!("iterations = number of iterations till stopping (1 and up)");
        eprintln!("threads = number of threads for the Rust version");
        process::exit(1);
    }

    let n: usize = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Invalid num");
        process::exit(1);
    });

    let iterations: usize = args[3].parse().unwrap_or_else(|_| {
        eprintln!("Invalid iterations");
        process::exit(1);
    });

    let num_threads: usize = args[4].parse().unwrap_or_else(|_| {
        eprintln!("Invalid threads");
        process::exit(1);
    });

    if n == 0 {
        eprintln!("num must be > 0");
        process::exit(1);
    }

    if num_threads == 0 {
        eprintln!("threads must be > 0");
        process::exit(1);
    }

    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Failed to build Rayon thread pool: {e}");
            process::exit(1);
        });

    // calloc equivalent: zero-initialized
    let mut playground = vec![0.0_f32; n * n];

    // Edge initialization
    for i in 0..n {
        playground[index(i, 0, n)] = 100.0;
    }
    for i in 0..n {
        playground[index(i, n - 1, n)] = 100.0;
    }
    for j in 0..n {
        playground[index(0, j, n)] = 100.0;
    }
    for j in 0..n {
        playground[index(n - 1, j, n)] = 100.0;
    }

    let start = Instant::now();
    pool.install(|| {
        parallel_heat_dist(&mut playground, n, iterations);
    });
    let time_taken = start.elapsed().as_secs_f64();

    println!("Time taken = {}", time_taken);
}

fn parallel_heat_dist(playground: &mut [f32], n: usize, iterations: usize) {
    let upper = n - 1;

    // calloc + memcpy equivalent
    let mut temp = playground.to_vec();

    for _k in 0..iterations {
        // Update interior cells only
        temp.par_chunks_mut(n).enumerate().for_each(|(i, row)| {
            if i == 0 || i == upper {
                return;
            }

            for j in 1..upper {
                row[j] = (playground[index(i - 1, j, n)]
                    + playground[index(i + 1, j, n)]
                    + playground[index(i, j - 1, n)]
                    + playground[index(i, j + 1, n)])
                    / 4.0;
            }
        });

        // memcpy(playground, temp, ...)
        playground.copy_from_slice(&temp);
    }
}
