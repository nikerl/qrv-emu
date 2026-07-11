// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

#include <stdio.h>
#include <stdint.h>
#include "quadrisparse.h"


int main(void) {
    int a[4][4] = {
        {1, 2, 3, 4}, 
        {1, 2, 3, 4}, 
        {1, 2, 3, 4}, 
        {1, 2, 3, 4},
    };

    int b[4][4] = {
        {6, 7, 8, 9}, 
        {6, 7, 8, 9}, 
        {6, 7, 8, 9}, 
        {6, 7, 8, 9},
    };

    int c[4][4];

    int ref[4][4] = {
        {80, 80, 80, 80}, 
        {80, 80, 80, 80}, 
        {80, 80, 80, 80}, 
        {80, 80, 80, 80},
    };


    // Populate registers with matrix pointers
    register int* r_a0 asm("x10") = *a;
    register int* r_a1 asm("x11") = *b;
    register int* r_a2 asm("x12") = *c;
    register int  r_a3 asm("x13") = 4; // stride

    // Encode and insert matrix instructions 
    asm volatile(
        ".word %0\n"   // mld md=0, base=x10, stride=x13
        ".word %1\n"   // mld md=1, base=x11, stride=x13
        ".word %2\n"   // mld md=2, base=x12, stride=x13
        ".word %3\n"   // mmasa md=2, ms1=0, ms2=1
        ".word %4\n"   // mst  ms1=2, base=x12, stride=x13
        :
        : "i" (ENCODE_MLD(0, 10, 13)),
        "i" (ENCODE_MLD(1, 11, 13)),
        "i" (ENCODE_MLD(2, 12, 13)),
        "i" (ENCODE_MMASA(2, 0, 1)),
        "i" (ENCODE_MST(2, 12, 13)),
        "r" (r_a0), "r" (r_a1), "r" (r_a2), "r" (r_a3)
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
