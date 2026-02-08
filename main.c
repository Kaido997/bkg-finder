#define BKG_IMPL
#include "bkg.h"

void print_sets(float *sub_sets) {
  int counter = 0;
  while (!fcomp(sub_sets[counter], 0.0f)) {
    printf("%.4f ", sub_sets[counter]);
    ++counter;
  }
  printf("\n");
}

int main(int argv, char **argc) {
  if (argv <= 1)
    exit(0);

  int max_comb = 2;
  float measure = (float)atof(argc[1]);
  float exclusions[64];
  int ex_size = 0;

  for (int i = 2; i < argv; i++) {
    if (i + 1 == argv) {
      break;
    }

    if (strcmp(argc[i], "-max") == 0) {
      max_comb = atoi(argc[i + 1]);
    }

    else if (strcmp(argc[i], "-ex") == 0) {
      for (int j = i + 1; j < argv; j++) {
        ex_size++;
        exclusions[ex_size - 1] = (float)atof(argc[j]);
      }

      break;
    }
  }

  // printf("COMMANDS:\n - max comb: %d\n - measure: %.4f\n", max_comb,
  // measure);

  //    debug_print("exclusions", exclusions, ex_size);
  //  printf("Ex size = %d\n", ex_size);
  float **combinations = (float **)malloc(sizeof(float *) * max_comb);
  for (int i = 0; i < max_comb; i++) {
    combinations[i] = (float *)malloc(sizeof(float) * 16);

    // Initialize to 0
    for (int j = 0; j < 16; j++) {
      combinations[i][j] = 0.0f;
    }
  }
  int *error_code = (int *)malloc(sizeof(int));
  init_bkg_set();
  find_combination(measure, max_comb, exclusions, ex_size, combinations,
                   error_code);
  if (*error_code == 0) {
    printf("ITERATION LIMIT REACHED: TERMINAITNG...\n");
    return 1;
  }
  for (int i = 0; i < max_comb; ++i) {
    print_sets(combinations[i]);
  }
  return 0;
}
