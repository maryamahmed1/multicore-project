#include <stdio.h>
#include <stdlib.h>
#include <omp.h>

void compute_histogram(char *page, int page_size, int *histogram, int num_buckets);
int main(int argc, char *argv[]) {
    if (argc != 3) {
        fprintf(stderr, "Usage %s page_size threads\n", argv[0]);
        return 1;
    }
    int page_size = atoi(argv[1]);
    int threads = atoi(argv[2]);
    int num_buckets = 10;

    omp_set_num_threads(threads);
    char *page = malloc(page_size*sizeof(char));
    int *histogram = calloc(num_buckets, sizeof(int));
    int *simple_histogram = calloc(num_buckets, sizeof(int));

    for (int i = 0; i < page_size; i++) {
        page[i] = rand() % 10;
    }

    // basic implementation that isnt always correct
    #pragma omp parallel for
    for (int i = 0; i<page_size; i++) {
        simple_histogram[page[i]]++;
    }
    for (int i = 0; i < num_buckets; i++) {
        if (simple_histogram[i] > 0) {
            printf("simple histogram bucket %d has weight %d\n", i, simple_histogram[i]);
        }
    }

    compute_histogram(page, page_size, histogram, num_buckets);
    for (int i = 0; i < num_buckets; i++) {
        if (histogram[i] > 0) {
            printf("bucket %d has weight %d\n", i, histogram[i]);
        }
    }

    free(page);
    free(histogram);
    free(simple_histogram);
    return 0;
}

void compute_histogram(char * page, int page_size, int *histogram, int num_buckets) {
    int num_threads = omp_get_max_threads();
    int local_histogram[num_threads+1][num_buckets];
    #pragma omp parallel
    {
        
        int tid = omp_get_thread_num();
        #pragma omp for
        for (int i = 0; i<page_size; i++) {
            char read_character = page[i];
            local_histogram[tid][read_character]++;
        }
        #pragma omp for
        for (int i = 0; i<num_buckets; i++) {
            for (int t = 0; t<num_threads; t++) {
                histogram[i] += local_histogram[t][i];
            }
        }
    }
}