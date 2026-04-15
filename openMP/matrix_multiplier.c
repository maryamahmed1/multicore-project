#include <omp.h>
#include <stdio.h>
#include <stdlib.h>

// I want to setup a benchmark program
// I need a main function that runs everything
// one test to show the parallelized code with 10x10 matrix, 100x100, 1000x1000
// matrix we will get the times for each matrix I want the main functoin to run
// the code and make a table to compare the times ran for rust to do this and
// openmp to do this
void benchmark(const int num_threads, const int N);

int main(int argc, char *argv[]) {
  if (argc != 2) {
    printf("Usage: incorrect number of inputs try again");
    return 1;
  }
  int num_threads = atoi(argv[1]);

  if (num_threads <= 0) {
    fprintf(stderr, "Invalid input:num_threads should be > 0");
    return 1;
  }

  // This will show how long it takes just to create a thread
  // it will show the time it takes to startup, create threads, end
  double simple_start = omp_get_wtime();
#pragma omp parallel
  {
  }
  double simple_end = omp_get_wtime();
  double simple_total_time = 0;
  simple_total_time = simple_end - simple_start;
  printf("DATA:overhead,OpenMP,0,%d,%f\n", num_threads, simple_total_time);

  benchmark(num_threads, 10);
  benchmark(num_threads, 100);
  benchmark(num_threads, 500);

  return 0;
}

void mat_multiplier(int n, int m, int p, int A[n][m], int B[m][p],
                    int C[n][p]) {
#pragma omp parallel for collapse(2)
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < p; j++) {
      C[i][j] = 0;
      for (int k = 0; k < m; k++) {
        C[i][j] += A[i][k] * B[k][j];
      }
    }
  }
}

void benchmark(const int num_threads, const int N) {
  omp_set_num_threads(num_threads);
  // initializing the matrices
  int A[N][N];
  int B[N][N];
  int C[N][N];
  for (int i = 0; i < N; i++) {
    for (int j = 0; j < N; j++) {
      A[i][i] = 1;
      B[i][i] = 1;
      C[i][i] = 1;
    }
  }
  double total_time = 0.0;
  double start = omp_get_wtime();
  mat_multiplier(N, N, N, A, B, C);
  double end = omp_get_wtime();
  total_time = (end - start);
  printf("DATA:mat_mul,OpenMP,%d,%d,%f\n", N, num_threads, total_time);
}
