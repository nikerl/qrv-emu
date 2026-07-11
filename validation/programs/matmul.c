// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

#include <stdio.h>
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
        {60, 70, 80, 90},
        {60, 70, 80, 90},
        {60, 70, 80, 90},
        {60, 70, 80, 90},
    };


    for (int i = 0; i < 4; i++) {
        for (int j = 0; j < 4; j++) {
            for (int k = 0; k < 4; k++) {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }

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
    }
    else {
        printf("Matmul PASS! All results match\n");
        return 0;
    }
}
