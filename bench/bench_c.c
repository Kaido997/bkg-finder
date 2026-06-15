#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define BKG_IMPL
#include "../bkg.h"

static const float ALL_MEASURES[] = {
    0.0500f, 0.1000f, 0.2000f, 0.2500f, 0.5000f, 0.7500f, 0.8000f,
    1.0000f, 2.0000f, 3.0000f, 4.0000f,
};
static const int ALL_MEASURES_LEN = 11;

int validate(float *combo, float measure) {
    float sum = 0.0f;
    for (int i = 0; i < 16; i++) {
        if (combo[i] < EPSILON)
            break;
        sum += combo[i];
    }
    return fcomp(sum, measure);
}

double get_time_sec(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec + ts.tv_nsec / 1e9;
}

int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "usage: bench_c <count>\n");
        return 1;
    }
    int count = atoi(argv[1]);
    int ok = 0;
    int fail = 0;

    init_bkg_set();

    double start = get_time_sec();
    for (int i = 0; i < count; i++) {
        float measure = ALL_MEASURES[i % ALL_MEASURES_LEN];

        float **combinations = (float **)malloc(sizeof(float *));
        combinations[0] = (float *)calloc(16, sizeof(float));
        float exclusions[64] = {0};
        int ex_size = 0;
        int error_code = -1;

        upperbound_count = 0;

        find_combination(measure, 1, exclusions, ex_size, combinations, &error_code);

        if (validate(combinations[0], measure)) {
            ok++;
        } else {
            fail++;
        }

        free(combinations[0]);
        free(combinations);
    }

    double elapsed = get_time_sec() - start;
    printf("count=%d ok=%d fail=%d elapsed_ms=%.3f\n", count, ok, fail, elapsed * 1000.0);

    return 0;
}
