#include <stdio.h>
#include <stdlib.h>
#include "utils.h"
#include <string.h>
#define BKG_SET_TYPE 81
#define UPPER_BOUND_RECURSION_LIMIT 100000000

float* block_gauge_set;
long upperbound_count = 0;

void push_range(float* target, float from, float to, float step, int offset) {
    if (target == NULL) return; 
    int idx = offset;
    for (float i = from; i < to || fcomp(i, to); i+= step) {
         target[idx] = i;
         idx++;
    }
}

void init_bkg_set(void) {
    block_gauge_set = (float*)calloc(BKG_SET_TYPE, sizeof(float));
    push_range(block_gauge_set, 0.1001f, 0.1009f, 0.0001f, 0);
    push_range(block_gauge_set, 0.1010f, 0.1490f, 0.0010f, 9);
    push_range(block_gauge_set, 0.0500f, 0.9500f, 0.0500f, 58);
    push_range(block_gauge_set, 1.0000f, 4.0000f, 1.0000f, 77);
}

int bkg_find_rbt(
    float target, 
    float current_sum, 
    int index, 
    float* blocks, 
    float sub_sets[], 
    int sub_sets_size, 
    float exclusions[],
    int ex_size
) 
{
    upperbound_count++;
    if (upperbound_count >= UPPER_BOUND_RECURSION_LIMIT) {
        printf("ITERATION LIMIT REACHED: TERMINAITNG...\n");
        return 1;
    }
        
    if (current_sum > target) {
        return 0;
    }

    for (int j = 0; j < ex_size; j++) {
        if (fcomp(blocks[index], exclusions[j])) {
            //printf("Exculding: %.4f\n", exclusions[j]);
            if (bkg_find_rbt(target, current_sum, index - 1, blocks, sub_sets, sub_sets_size, exclusions, ex_size)) {
                return 1;
            } else return 0;
        }
    }

    if (fcomp(current_sum, target)) {
        for (int i = 0; i < sub_sets_size; i++) {
            printf("%.4f ", sub_sets[i]);
        }
        printf("\n");
        return 1;
    }
    
    if (index < 0) {
        return 0;
    }

    sub_sets[sub_sets_size] = blocks[index];

    if (bkg_find_rbt(target, current_sum + blocks[index], index - 1, blocks, sub_sets, sub_sets_size + 1, exclusions, ex_size)) return 1;
    if (bkg_find_rbt(target, current_sum, index - 1, blocks, sub_sets, sub_sets_size, exclusions, ex_size)) return 1;

    return 0;
}

void find_combination(float measure, int max_comb, float exclusions[], int ex_size) {
    float* sub_sets;

    for (int i = 0; i < max_comb; i++) {
        //printf("Running n.%d iteration, with n.%d exclusions", i, ex_size);
        sub_sets = malloc(sizeof(float) * 16);
        if (bkg_find_rbt(measure, 0.0f, 80, block_gauge_set, sub_sets, 0, exclusions, ex_size)) {
            int counter = 0;
            while (sub_sets[counter]) {
                float v = sub_sets[counter];
                exclusions[ex_size + counter] = v;
                counter++;
            }
            ex_size += counter;
ii        }
        free(sub_sets);
    }
}

int main(int argv, char** argc) {
    if (argv <= 1) exit(0);

    int max_comb = 2;
    float measure = (float)atof(argc[1]);
    float exclusions[64];
    int ex_size = 0;

    for (int i = 2; i < argv; i++) {
        if (i + 1 == argv) {
            break;
        }

        if (strcmp(argc[i], "-max") == 0) 
        {
            max_comb = atoi(argc[i+1]);
        }

        else if (strcmp(argc[i], "-ex") == 0) 
        { 
            for (int j = i+1; j < argv; j++) {
                ex_size++;
                exclusions[ex_size - 1] = (float)atof(argc[j]);
        }
         
            break;
        } 
    }

    //printf("COMMANDS:\n - max comb: %d\n - measure: %.4f\n", max_comb, measure);

//    debug_print("exclusions", exclusions, ex_size);
  //  printf("Ex size = %d\n", ex_size);
    
    init_bkg_set();
    find_combination(measure, max_comb, exclusions, ex_size);
    free(block_gauge_set);
    return 0;
}
