#include "utils.h"
#include <stdio.h>
#define EPSILON 0.00001f

/*
* Compare for equality 2 floats by a defined tolerance EPISILON
*/
bool fcomp(float x, float y) {
    assert(EPSILON);
    return fabsf(x - y) < EPSILON;
}

int to_int(float f, enum SIDE s){
    int right_side = (int)((f - (int)f) * 10000);
    int left_side = (int)f;
    if (left_side > 0  && right_side != 0) {
        right_side++;        
    }

    if (s == left) {
        return left_side;
    } 
    else if (s == right) {
        return right_side;
    }

    return -1;
}

void debug_print(char what[], float* t, int s) {
    printf("%s -> ", what);
    for (int i = 0; i < s ; ++i) {
        if ( i + 1 == s) {
            printf("%.4f", t[i]);
        } else {
            printf("%.4f, ", t[i]);
        }
    }
    printf("\n");

}
