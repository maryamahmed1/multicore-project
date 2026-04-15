use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::env;
use std::process;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn compute_histogram(page: &[u8], histogram: &mut [usize], num_buckets: usize, num_threads: usize) {
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Failed to build Rayon thread pool: {e}");
            process::exit(1);
        });

    let chunk_size = page.len().div_ceil(num_threads);

    let local_histograms = pool.install(|| {
        page.par_chunks(chunk_size)
            .map(|chunk| {
                let mut local = vec![0usize; num_buckets];
                for &value in chunk {
                    local[value as usize] += 1;
                }
                local
            })
            .collect::<Vec<_>>()
    });

    for bucket in 0..num_buckets {
        histogram[bucket] = 0;
        for local in &local_histograms {
            histogram[bucket] += local[bucket];
        }
    }
}

fn compute_histogram_mutex(page: &[u8], histogram: &mut [usize], num_threads: usize) {
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Failed to build Rayon thread pool: {e}");
            process::exit(1);
        });

    let shared = Arc::new(Mutex::new(vec![0usize; histogram.len()]));

    pool.install(|| {
        page.par_iter().for_each({
            let shared = Arc::clone(&shared);
            move |&value| {
                let mut guard = shared.lock().expect("mutex poisoned");
                guard[value as usize] += 1;
            }
        });
    });

    let result = Arc::try_unwrap(shared)
        .expect("Arc still had multiple owners")
        .into_inner()
        .expect("mutex poisoned");

    histogram.copy_from_slice(&result);
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage {} page_size threads", args[0]);
        process::exit(1);
    }

    let page_size: usize = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Invalid page_size");
        process::exit(1);
    });

    let threads: usize = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Invalid threads");
        process::exit(1);
    });

    if page_size == 0 {
        eprintln!("page_size must be > 0");
        process::exit(1);
    }

    if threads == 0 {
        eprintln!("threads must be > 0");
        process::exit(1);
    }

    let num_buckets = 10usize;

    let mut page = vec![0u8; page_size];
    for x in &mut page {
        *x = rand::random::<u8>() % num_buckets as u8;
    }

    let mut histogram = vec![0usize; num_buckets];
    let mut simple_histogram = vec![0usize; num_buckets];

    println!(
        "simple histogram version: omitted in Rust because the direct shared-mutation version is rejected by the compiler"
    );

    let start = Instant::now();
    compute_histogram_mutex(&page, &mut simple_histogram, threads);
    let mutex_time = start.elapsed().as_secs_f64();
    println!(
        "DATA:hist_mutex,Rust,{},{},{:.6}",
        page_size, threads, mutex_time
    );
    (0..num_buckets).for_each(|i| {
        if simple_histogram[i] > 0 {
            println!(
                "simple histogram bucket {} has weight {}",
                i, simple_histogram[i]
            );
        }
    });

    let start = Instant::now();
    compute_histogram(&page, &mut histogram, num_buckets, threads);
    let local_time = start.elapsed().as_secs_f64();
    println!(
        "DATA:hist_local,Rust,{},{},{:.6}",
        page_size, threads, local_time
    );
    (0..num_buckets).for_each(|i| {
        if histogram[i] > 0 {
            println!("bucket {} has weight {}", i, histogram[i]);
        }
    });
}
