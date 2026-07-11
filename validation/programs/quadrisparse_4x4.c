// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

#include <stdio.h>
#include <stdint.h>
#include "quadrisparse.h"


int main(void) {
    int a_val[4] = {1, 4, 6, 9};

    int a_col[4] = {0, 3, 1, 2};

    int b[7][4] = {
        {0, 0, 0, 0}, 
        {1, 1, 1, 1}, 
        {2, 2, 2, 2}, 
        {3, 3, 3, 3},
        {4, 4, 4, 4},
        {5, 5, 5, 5},
        {6, 6, 6, 6},
    };

    int c[4][4];

    int ref[4][4] = {
        {36, 36, 36, 36}, 
        {0, 0, 0, 0}, 
        {0, 0, 0, 0}, 
        {0, 0, 0, 0},
    };


    // Populate registers with matrix pointers
    register int* r_aval asm("x20") = a_val;
    register int* r_acol asm("x21") = a_col;
    register int* r_a3 asm("x12") = *c;

    // Encode and insert matrix instructions 
    asm volatile(
        ".word %0\n"   // mzero md=0
        ".word %1\n"   // spld md=1, a_val=x20, a_col=x21
        :
        : "i" (ENCODE_MZERO(0)),
        "i" (ENCODE_SPLD(1, 4)),
        "r" (r_aval), "r" (r_acol)
        : "memory"
    );


    register int* r_bbase asm("x20") = *b;
    register int r_bstride asm("x21") = 4;

    asm volatile(
        ".word %0\n"   // dld md=2, base=20, stride=21
        ".word %1\n"   // spmac md=0, ms1=1, ms2=2
        ".word %2\n"   // mst ms1=0, base=12, stride=21
        :
        : "i" (ENCODE_DLD(2, 1)),
        "i" (ENCODE_SPMAC(0, 1, 2)),
        "i" (ENCODE_MST(0, 12, 21)),
        "r" (r_bbase), "r" (r_bstride)
        : "memory"
    );

    
    // print result
    for (int i = 0; i < 4; i++) {
        for (int j = 0; j < 4; j++) {
            printf("%d ", c[i][j]);
        }
        printf("\n");
    }

    // check result
    int fail = 0;
    for (int i = 0; i < 4; i++) {
        for (int j = 0; j < 4; j++) {
            if (c[i][j] != ref[i][j]) {
                fail = 1;
            }
        }
    }
    if (fail) {
        printf("Matmul FAIL! Results doesnt match\n");
        return 1;
    } else {
        printf("Matmul PASS! All results match\n");
        return 0;
    }
}
