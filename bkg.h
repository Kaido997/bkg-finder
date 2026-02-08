#ifndef BKG_H
#define BKG_H
#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define BKG_SET_TYPE 81
#define UPPER_BOUND_RECURSION_LIMIT 100000000

/*
 * Compare for equality 2 floats by a defined tolerance EPISILON
 */
bool fcomp(float x, float y);
void find_combination(float measure, int max_comb, float exclusions[],
                      int ex_size, float **s, int *error_code);
void init_bkg_set(void);
enum { LIMIT_REACHED, ITERNAL };
#endif

#ifdef BKG_IMPL
#define EPSILON 0.00001f
bool fcomp(float x, float y) {
  assert(EPSILON);
  return fabsf(x - y) < EPSILON;
}

static float *block_gauge_set;
static long upperbound_count = 0;

void push_range(float *target, float from, float to, float step, int offset) {
  if (target == NULL)
    return;
  int idx = offset;
  for (float i = from; i < to || fcomp(i, to); i += step) {
    target[idx] = i;
    idx++;
  }
}

void init_bkg_set(void) {
  block_gauge_set = (float *)calloc(BKG_SET_TYPE, sizeof(float));
  push_range(block_gauge_set, 0.1001f, 0.1009f, 0.0001f, 0);
  push_range(block_gauge_set, 0.1010f, 0.1490f, 0.0010f, 9);
  push_range(block_gauge_set, 0.0500f, 0.9500f, 0.0500f, 58);
  push_range(block_gauge_set, 1.0000f, 4.0000f, 1.0000f, 77);
}

/*
 *
 * */
int bkg_find_rbt(float target, float current_sum, int index, float *blocks,
                 float sub_sets[], int sub_sets_size, float exclusions[],
                 int ex_size, int *error) {
  upperbound_count++;
  if (upperbound_count >= UPPER_BOUND_RECURSION_LIMIT) {
    // printf("ITERATION LIMIT REACHED: TERMINAITNG...\n");
    *error = LIMIT_REACHED;
    return 1;
  }

  if (current_sum > target) {
    return 0;
  }

  for (int j = 0; j < ex_size; j++) {
    if (fcomp(blocks[index], exclusions[j])) {
      // printf("Exculding: %.4f\n", exclusions[j]);
      if (bkg_find_rbt(target, current_sum, index - 1, blocks, sub_sets,
                       sub_sets_size, exclusions, ex_size, error)) {
        return 1;
      } else
        return 0;
    }
  }

  if (index < 0) {
    return 0;
  }

  sub_sets[sub_sets_size] = blocks[index];
  if (fcomp(current_sum, target)) {

    sub_sets[sub_sets_size] = 0.0f;
    return 1;
  }

  if (bkg_find_rbt(target, current_sum + blocks[index], index - 1, blocks,
                   sub_sets, sub_sets_size + 1, exclusions, ex_size, error))
    return 1;
  if (bkg_find_rbt(target, current_sum, index - 1, blocks, sub_sets,
                   sub_sets_size, exclusions, ex_size, error))
    return 1;

  return 0;
}

void find_combination(float measure, int max_comb, float exclusions[],
                      int ex_size, float **s, int *error_code) {
  float *sub_sets;
  *error_code = -1;
  for (int i = 0; i < max_comb; i++) {
    sub_sets = (float *)malloc(sizeof(float) * 16);

    if (bkg_find_rbt(measure, 0.0f, 80, block_gauge_set, sub_sets, 0,
                     exclusions, ex_size, error_code)) {
      int counter = 0;
      while (sub_sets[counter]) {
        float v = sub_sets[counter];
        exclusions[ex_size + counter] = v;
        s[i][counter] = v;
        counter++;
      }
      ex_size += counter;
    }
    free(sub_sets);
  }
}
#endif
