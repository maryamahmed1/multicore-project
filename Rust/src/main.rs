mod programmability;

use programmability::{BenchmarkCase, BenchmarkParams, ImplKind};

use programmability::run_programmability_benchmark;
use rayon;

fn main() {
    println!("max threads  =  {}", rayon::current_num_threads());
    let params = BenchmarkParams {
        case: BenchmarkCase::Histogram,
        implementation: ImplKind::RayonLocal,
        n: 10000,
        bins: 100,
        rows: 10,
        cols: 10,
        inner: 10,
        repeats: 1,
        verify: true,
        num_threads: 8,
    };
    let results = run_programmability_benchmark(&params).expect("benchmark failed");

    for r in results {
        println!(
            "case={} impl={} repeat={} elapsed_ms={:.3} verified={}",
            r.case_name,
            r.impl_name,
            r.repeat,
            r.elapsed.as_secs_f64() * 1000.0,
            r.verified
        );
    }
}
