#include <stdio.h>

void mat_multiplier(int n, int m, int p, int A[n][m], int B[m][p], int C[n][p]) {
    for (int i = 0; i<n; i++) {
        for (int j=0; j<p; j++) {
            C[i][j] = 0;
            for (int k = 0; k<m; k++) {
                C[i][j] += A[i][k] * B[k][j];
            }
        }
    }
}

int main() {
    int n = 2, m = 3, p = 2;
    int A[2][3] = { {1, 2, 3}, {4, 5, 6} };
    int B[3][2] = { {7, 8}, {9, 10}, {11, 12} };
    int C[2][2];

    mat_multiplier(n, m, p, A, B, C);

    for (int i = 0; i < n; i++) {
        for (int j = 0; j < p; j++) {
            printf("%d ", C[i][j]);
        }
        printf("\n");
    }

    return 0;
}
