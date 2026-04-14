#include <stdio.h>
#include <omp.h>

int main() {
    int counter = 0;
    #pragma omp parallel for num_threads(3)
    for (int i = 0; i<10000; i++) {
        counter = counter+1;
    }
    printf("counter: %d\n", counter);
    return 0;
}