use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum BenchmarkCase {
    ArrayMin,
    Histogram,
    MatrixMul,
}

#[derive(Debug, Clone, Copy)]
pub enum ImplKind {
    Seq,
    RayonLocal,
    Mutex,
}

#[derive(Debug, Clone)]
pub struct BenchmarkParams {
    pub case: BenchmarkCase,
    pub implementation: ImplKind,
    pub n: usize,     // general size parameter
    pub bins: usize,  // used by histogram
    pub rows: usize,  // used by matrix multiply
    pub cols: usize,  // used by matrix multiply
    pub inner: usize, // used by matrix multiply
    pub repeats: usize,
    pub verify: bool,
    pub num_threads: usize,
}

#[derive(Debug, Clone)]
pub struct BenchmarkRunResult {
    pub case_name: &'static str,
    pub impl_name: &'static str,
    pub repeat: usize,
    pub elapsed: Duration,
    pub verified: bool,
    pub notes: &'static str,
}

pub fn run_programmability_benchmark(
    params: &BenchmarkParams,
) -> Result<Vec<BenchmarkRunResult>, String> {
    validate_params(params)?;

    let mut results = Vec::with_capacity(params.repeats);

    let pool = ThreadPoolBuilder::new()
        .num_threads(params.num_threads)
        .build()
        .map_err(|e| format!("failed to build Rayon thread pool: {e}"))?;

    for repeat in 1..=params.repeats {
        let result = pool.install(|| match params.case {
            BenchmarkCase::ArrayMin => run_array_min(params, repeat),
            BenchmarkCase::Histogram => run_histogram(params, repeat),
            BenchmarkCase::MatrixMul => run_matrix_mul(params, repeat),
        })?;
        results.push(result);
    }

    Ok(results)
}

fn validate_params(params: &BenchmarkParams) -> Result<(), String> {
    if params.repeats == 0 {
        return Err("repeats must be > 0".to_string());
    }

    match params.case {
        BenchmarkCase::ArrayMin => {
            if params.n == 0 {
                return Err("array_min requires n > 0".to_string());
            }
        }
        BenchmarkCase::Histogram => {
            if params.n == 0 {
                return Err("histogram requires n > 0".to_string());
            }
            if params.bins == 0 {
                return Err("histogram requires bins > 0".to_string());
            }
        }
        BenchmarkCase::MatrixMul => {
            if params.rows == 0 || params.cols == 0 || params.inner == 0 {
                return Err("matrix_mul requires rows > 0, cols > 0, inner > 0".to_string());
            }
        }
    }

    Ok(())
}

fn run_array_min(params: &BenchmarkParams, repeat: usize) -> Result<BenchmarkRunResult, String> {
    let data = generate_array_data(params.n);

    let start = Instant::now();
    let result = match params.implementation {
        ImplKind::Seq => array_min_seq(&data),
        ImplKind::RayonLocal => array_min_rayon(&data),
        ImplKind::Mutex => {
            return Err(
                "array_min + mutex is intentionally unsupported in this scaffold; \
                 reductions are better expressed as seq or rayon_local"
                    .to_string(),
            );
        }
    };
    let elapsed = start.elapsed();

    let verified = if params.verify {
        let baseline = array_min_seq(&data);
        result == baseline
    } else {
        false
    };

    Ok(BenchmarkRunResult {
        case_name: "array_min",
        impl_name: impl_name(params.implementation),
        repeat,
        elapsed,
        verified,
        notes: "simple reduction benchmark",
    })
}

fn run_histogram(params: &BenchmarkParams, repeat: usize) -> Result<BenchmarkRunResult, String> {
    let data = generate_histogram_data(params.n, params.bins);

    let start = Instant::now();
    let hist = match params.implementation {
        ImplKind::Seq => histogram_seq(&data, params.bins),
        ImplKind::RayonLocal => histogram_rayon_local(&data, params.bins),
        ImplKind::Mutex => histogram_mutex(&data, params.bins),
    };
    let elapsed = start.elapsed();

    let verified = if params.verify {
        let baseline = histogram_seq(&data, params.bins);
        hist == baseline
    } else {
        false
    };

    Ok(BenchmarkRunResult {
        case_name: "histogram",
        impl_name: impl_name(params.implementation),
        repeat,
        elapsed,
        verified,
        notes: "shared-write / reduction benchmark",
    })
}

fn run_matrix_mul(params: &BenchmarkParams, repeat: usize) -> Result<BenchmarkRunResult, String> {
    let a = generate_matrix(params.rows, params.inner);
    let b = generate_matrix(params.inner, params.cols);

    let start = Instant::now();
    let c = match params.implementation {
        ImplKind::Seq => matmul_seq(&a, &b, params.rows, params.inner, params.cols),
        ImplKind::RayonLocal => matmul_rayon_rows(&a, &b, params.rows, params.inner, params.cols),
        ImplKind::Mutex => {
            return Err(
                "matrix_mul + mutex is intentionally unsupported in this scaffold; \
                 row-wise parallelism is the intended safe expression"
                    .to_string(),
            );
        }
    };
    let elapsed = start.elapsed();

    let verified = if params.verify {
        let baseline = matmul_seq(&a, &b, params.rows, params.inner, params.cols);
        approx_eq(&c, &baseline, 1e-9)
    } else {
        false
    };

    Ok(BenchmarkRunResult {
        case_name: "matrix_mul",
        impl_name: impl_name(params.implementation),
        repeat,
        elapsed,
        verified,
        notes: "regular parallel loop / work partition benchmark",
    })
}

fn impl_name(kind: ImplKind) -> &'static str {
    match kind {
        ImplKind::Seq => "seq",
        ImplKind::RayonLocal => "rayon_local",
        ImplKind::Mutex => "mutex",
    }
}

/* =========================
Data generation
========================= */

fn generate_array_data(n: usize) -> Vec<i64> {
    let mut data = Vec::with_capacity(n);
    for i in 0..n {
        let value = ((i as i64 * 48271) % 1_000_003) - 500_000;
        data.push(value);
    }
    data
}

fn generate_histogram_data(n: usize, bins: usize) -> Vec<usize> {
    let mut data = Vec::with_capacity(n);
    for i in 0..n {
        data.push((i * 17 + 13) % bins);
    }
    data
}

fn generate_matrix(rows: usize, cols: usize) -> Vec<f64> {
    let mut m = vec![0.0; rows * cols];
    for r in 0..rows {
        for c in 0..cols {
            m[r * cols + c] = ((r * 131 + c * 17) % 1000) as f64 / 100.0;
        }
    }
    m
}

/* =========================
array_min
========================= */

fn array_min_seq(data: &[i64]) -> i64 {
    let mut min_val = data[0];
    for &x in &data[1..] {
        if x < min_val {
            min_val = x;
        }
    }
    min_val
}

fn array_min_rayon(data: &[i64]) -> i64 {
    *data.par_iter().min().expect("data must be non-empty")
}

/* =========================
histogram
========================= */

fn histogram_seq(data: &[usize], bins: usize) -> Vec<usize> {
    let mut hist = vec![0usize; bins];
    for &x in data {
        hist[x] += 1;
    }
    hist
}

fn histogram_rayon_local(data: &[usize], bins: usize) -> Vec<usize> {
    data.par_iter()
        .fold(
            || vec![0usize; bins],
            |mut local, &x| {
                local[x] += 1;
                local
            },
        )
        .reduce(
            || vec![0usize; bins],
            |mut a, b| {
                for i in 0..bins {
                    a[i] += b[i];
                }
                a
            },
        )
}

fn histogram_mutex(data: &[usize], bins: usize) -> Vec<usize> {
    let hist = Arc::new(Mutex::new(vec![0usize; bins]));

    data.par_iter().for_each({
        let hist = Arc::clone(&hist);
        move |&x| {
            let mut guard = hist.lock().expect("mutex poisoned");
            guard[x] += 1;
        }
    });

    Arc::try_unwrap(hist)
        .expect("Arc still has multiple owners")
        .into_inner()
        .expect("mutex poisoned")
}

/* =========================
matrix multiplication
C = A(rows x inner) * B(inner x cols)
========================= */

fn matmul_seq(a: &[f64], b: &[f64], rows: usize, inner: usize, cols: usize) -> Vec<f64> {
    let mut c = vec![0.0; rows * cols];

    for i in 0..rows {
        for k in 0..inner {
            let aik = a[i * inner + k];
            for j in 0..cols {
                c[i * cols + j] += aik * b[k * cols + j];
            }
        }
    }

    c
}

fn matmul_rayon_rows(a: &[f64], b: &[f64], rows: usize, inner: usize, cols: usize) -> Vec<f64> {
    let mut c = vec![0.0; rows * cols];

    c.par_chunks_mut(cols).enumerate().for_each(|(i, row_out)| {
        for k in 0..inner {
            let aik = a[i * inner + k];
            for j in 0..cols {
                row_out[j] += aik * b[k * cols + j];
            }
        }
    });

    c
}

/* =========================
verification helpers
========================= */

fn approx_eq(a: &[f64], b: &[f64], eps: f64) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for (x, y) in a.iter().zip(b.iter()) {
        if (x - y).abs() > eps {
            return false;
        }
    }

    true
}

/* =========================
Example future main()
Not CLI yet; just showing usage.
========================= */

fn main() {
    let params = BenchmarkParams {
        case: BenchmarkCase::Histogram,
        implementation: ImplKind::RayonLocal,
        n: 10_000_000,
        bins: 256,
        rows: 512,
        cols: 512,
        inner: 512,
        repeats: 3,
        verify: true,
        num_threads: 1,
    };

    match run_programmability_benchmark(&params) {
        Ok(results) => {
            for r in results {
                println!(
                    "case={} impl={} repeat={} elapsed_ms={:.3} verified={} notes={}",
                    r.case_name,
                    r.impl_name,
                    r.repeat,
                    r.elapsed.as_secs_f64() * 1000.0,
                    r.verified,
                    r.notes
                );
            }
        }
        Err(e) => {
            eprintln!("benchmark error: {e}");
            std::process::exit(1);
        }
    }
}
