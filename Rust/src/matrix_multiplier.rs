use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::env;
use std::process;
use std::time::Instant;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: incorrect number of inputs try again");
        process::exit(1);
    }

    let num_threads: usize = match args[1].parse() {
        Ok(n) if n > 0 => n,
        _ => {
            eprintln!("Invalid input: num_threads should be > 0");
            process::exit(1);
        }
    };

    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Failed to build Rayon thread pool: {e}");
            process::exit(1);
        });

    let simple_start = Instant::now();
    pool.install(|| {
        rayon::scope(|_| {});
    });
    let simple_total_time = simple_start.elapsed().as_secs_f64();

    println!(
        "Time taken for simple thread creation = {:.6}",
        simple_total_time
    );

    benchmark(&pool, num_threads, 10);
    benchmark(&pool, num_threads, 100);
    benchmark(&pool, num_threads, 500);
}

fn mat_multiplier(m: usize, p: usize, a: &[i32], b: &[i32], c: &mut [i32]) {
    // chunk into n slices instead of looping parallel for
    c.par_chunks_mut(p).enumerate().for_each(|(i, row)| {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = (0..m).map(|k| a[i * m + k] * b[k * p + j]).sum();
        }

        // for j in 0..p {
        //     row[j] = 0;
        //     for k in 0..m {
        //         row[j] += a[i * m + k] * b[k * p + j];
        //     }
        // }
    });
}

fn benchmark(pool: &rayon::ThreadPool, num_threads: usize, n: usize) {
    let mut a = vec![0_i32; n * n];
    let mut b = vec![0_i32; n * n];
    let mut c = vec![0_i32; n * n];

    for i in 0..n {
        a[i * n + i] = 1;
        b[i * n + i] = 1;
        c[i * n + i] = 1;
    }

    let start = Instant::now();
    pool.install(|| {
        mat_multiplier(n, n, &a, &b, &mut c);
    });
    let total_time = start.elapsed().as_secs_f64();
    println!(
        "Time taken for matrix size N = {} and threads = {} completed with time = {:.6}",
        n, num_threads, total_time
    );
}
