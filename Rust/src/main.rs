mod heat;
mod matrix_multiplier;
mod programmability;
// mod racecondition;

use heat::main as heat_main;
use matrix_multiplier::main as mat_mul_main;
use programmability::{BenchmarkCase, BenchmarkParams, ImplKind};
// use racecondition::main as rc_main;

fn main() {
    println!("max threads  =  {}", rayon::current_num_threads());
    let _params = BenchmarkParams {
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
    // For mat mult
    mat_mul_main();

    // For heat
    // heat_main();

    // For race condition example
    // rc_main();

    // For programmability example
    // let results = run_programmability_benchmark(&params).expect("benchmark failed");
    //
    // for r in results {
    //     println!(
    //         "case={} impl={} repeat={} elapsed_ms={:.3} verified={}",
    //         r.case_name,
    //         r.impl_name,
    //         r.repeat,
    //         r.elapsed.as_secs_f64() * 1000.0,
    //         r.verified
    //     );
    // }
}
