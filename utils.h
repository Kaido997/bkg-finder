#include <assert.h>
#include <math.h>
#include <stdbool.h>
typedef enum SIDE {
    left,
    right
} SIDE;


int to_int(float f, enum SIDE s);

/*
* Compare for equality 2 floats by a defined tolerance EPISILON
*/
bool fcomp(float x, float y);

