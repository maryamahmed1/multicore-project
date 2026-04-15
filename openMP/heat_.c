#include <math.h>
#include <omp.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

int numthreads = 0;
#define index(i, j, N) ((i) * (N)) + (j)

void parallel_heat_dist(float *playground, unsigned int N,
                        unsigned int interations);
int main(int argc, char *argv[]) {
  unsigned int N; /* Dimention of NxN matrix */
  int iterations = 0;
  int i, j;

  /* The 2D array of points will be treated as 1D array of NxN elements */
  float *playground;

  // to measure time taken by a specific part of the code
  double time_taken;
  double start, end;

  if (argc != 4) {
    fprintf(stderr, "usage: %s num iterations threads\n", argv[0]);
    fprintf(stderr, "num = dimension of the square matrix \n");
    fprintf(stderr,
            "iterations = number of iterations till stopping (1 and up)\n");
    fprintf(stderr, "threads = number of threads for the  OpenMP version\n");
    exit(1);
  }

  N = (unsigned int)atoi(argv[1]);
  iterations = (unsigned int)atoi(argv[2]);
  numthreads = (unsigned int)atoi(argv[3]);

  /* Dynamically allocate NxN array of floats */
  playground = (float *)calloc(N * N, sizeof(float));
  if (!playground) {
    fprintf(stderr, " Cannot allocate the %u x %u array\n", N, N);
    exit(1);
  }

  /* Initialize it: calloc already initalized everything to 0 */
  // Edge elements  initialization
  for (i = 0; i < N; i++)
    playground[index(i, 0, N)] = 100;
  for (i = 0; i < N; i++)
    playground[index(i, N - 1, N)] = 100;
  for (j = 0; j < N; j++)
    playground[index(0, j, N)] = 100;
  for (j = 0; j < N; j++)
    playground[index(N - 1, j, N)] = 100;

  start = omp_get_wtime();
  parallel_heat_dist(playground, N, iterations);
  end = omp_get_wtime();
  time_taken = (end - start);

  printf("DATA:heat,OpenMP,%u,%d,%lf\n", N, numthreads, time_taken);

  free(playground);

  return 0;
}
void parallel_heat_dist(float *playground, unsigned int N,
                        unsigned int iterations) {

  // Loop indices
  int i, j, k;
  int upper = N - 1; // used instead of N to avoid updating the border points

  // number of bytes to be copied between array temp and array playground
  unsigned int num_bytes = 0;

  float *temp;

  /* Dynamically allocate another array for temp values */
  temp = (float *)calloc(N * N, sizeof(float));
  if (!temp) {
    fprintf(stderr, " Cannot allocate temp %u x %u array\n", N, N);
    exit(1);
  }

  num_bytes = N * N * sizeof(float);

  /* Copy initial array in temp */
  memcpy((void *)temp, (void *)playground, num_bytes);

  for (k = 0; k < iterations; k++) {
/* Calculate new values and store them in temp */
#pragma omp parallel for num_threads(numthreads) private(i, j)
    for (i = 1; i < upper; i++)
      for (j = 1; j < upper; j++)
        temp[index(i, j, N)] =
            (playground[index(i - 1, j, N)] + playground[index(i + 1, j, N)] +
             playground[index(i, j - 1, N)] + playground[index(i, j + 1, N)]) /
            4.0;

    /* Move new values into old values */
    memcpy((void *)playground, (void *)temp, num_bytes);
  }

  free(temp);
}
