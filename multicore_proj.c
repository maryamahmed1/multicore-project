#include <stdio.h>
#include <omp.h>

void mat_multiplier(int n, int m, int p, int A[n][m], int B[m][p], int C[n][p]) {
    #pragma omp parallel for collapse(2)
    for (int i = 0; i<n; i++) {
        for (int j=0; j<p; j++) {
            C[i][j] = 0;
            for (int k = 0; k<m; k++) {
                C[i][j] += A[i][k] * B[k][j];
            }
        }
    }
}