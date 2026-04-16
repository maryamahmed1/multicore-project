Multicore Programming Comparison Project
=====================================

This repository contains implementations of various parallel algorithms in C using OpenMP and in Rust using Rayon, along with benchmarking tools to compare their performance across different thread counts and problem sizes.

Project Structure
-----------------

- `openMP/`: C implementations using OpenMP
  - `heat_.c`: Heat equation solver
  - `matrix_multiplier.c`: Matrix multiplication
  - `programmability.c`: Histogram computation demonstrating race conditions and fixes
  - `race_condition.c`: Race condition examples
  - `results/`: Directory for OpenMP benchmark outputs

- `Rust/`: Rust implementations using Rayon
  - `src/bin/`: Executable binaries
    - `heat.rs`: Heat equation solver
    - `matrix_multiplier.rs`: Matrix multiplication
    - `programmability.rs`: Histogram computation
    - `racecondition.rs`: Race condition examples
  - `results/`: Directory for Rust benchmark outputs
  - `Cargo.toml`: Rust dependencies

- `analysis.py`: Python script to compile and run benchmarks
- `analysis.ipynb`: Jupyter notebook for result analysis and visualization
- `benchmark_results.csv`: Collected benchmark timing data

Algorithms Implemented
----------------------

1. **Matrix Multiplication**: Parallel matrix multiplication algorithms
2. **Heat Equation**: Numerical solution of the 2D heat equation
3. **Histogram Computation**: Demonstrates race conditions in parallel histogram building and shows corrected implementations
4. **Race Condition Examples**: Various examples of race conditions and synchronization techniques

Dependencies
------------

### C/OpenMP:
- GCC with OpenMP support
- Standard C libraries

### Rust:
- Rust toolchain (cargo, rustc)
- Rayon crate for parallelism
- Rand crate for random number generation

### Python (for analysis):
- Python 3
- Standard libraries (os, subprocess, csv)

Building and Running
--------------------

### OpenMP Programs:
```bash
mkdir -p openMP/bin
gcc -O3 -fopenmp openMP/matrix_multiplier.c -o openMP/bin/mat_mul
gcc -O3 -fopenmp openMP/heat_.c -o openMP/bin/heat
gcc -O3 -fopenmp openMP/programmability.c -o openMP/bin/prog
```

### Rust Programs:
```bash
cd Rust
cargo build --release
cd ..
```

### Running Benchmarks:
```bash
python3 analysis.py
```

This will compile all programs and run benchmarks with various thread counts and problem sizes, saving results to `benchmark_results.csv`.

### Analyzing Results:
Open `analysis.ipynb` in Jupyter to visualize and analyze the benchmark results.

Usage Examples
--------------

### Matrix Multiplication:
```bash
# OpenMP
./openMP/bin/mat_mul <matrix_size> <threads>

# Rust
cargo run --bin matrix_multiplier <matrix_size> <threads>
```

### Heat Equation:
```bash
# OpenMP
./openMP/bin/heat <grid_size> <iterations> <threads>

# Rust
cargo run --bin heat <grid_size> <iterations> <threads>
```

### Histogram:
```bash
# OpenMP
./openMP/bin/prog <page_size> <threads>

# Rust
cargo run --bin programmability <page_size> <threads>
```

Results Format
--------------

Benchmark results are stored in CSV format with columns:
- Benchmark: Name of the algorithm
- Language: OpenMP or Rust
- Problem_Size: Input size parameter
- Threads: Number of threads used
- Time_Seconds: Execution time in seconds

The analysis notebook provides visualizations comparing performance across languages and thread counts.

Contributing
------------

This project is for educational purposes demonstrating multicore programming concepts and performance comparisons between OpenMP and Rust/Rayon.