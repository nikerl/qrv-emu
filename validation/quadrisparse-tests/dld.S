# Copyright 2026
# Apache License, Version 2.0, see LICENSE for details.
#
# Author: Nik Erlandsson

#include "riscv_test.h"
#include "test_macros.h"
#include "quadrisparse.h"

RVTEST_RV32U
RVTEST_CODE_BEGIN

dld_test:
    li TESTNUM, 2

    la x20, a_val               # a_val base address
    la x21, a_col               # a_col base address
    la x22, scratch             # scratch base address
    la x23, expected            # expected base address
    .word ENCODE_SPLD(0, 4)     # spld m0 4 x20 x21
    la x20, b_mat               # b base address
    li x21, 4                   # stride
    .word ENCODE_DLD(1, 0)      # spld m0 4 x20 x21
    .word ENCODE_MST(1, 22, 21) # mst m0 x22 x24

CHECK_MAT_RESULT(x22, x23)

TEST_PASSFAIL
RVTEST_CODE_END


    .data
RVTEST_DATA_BEGIN

    .balign 16
a_val:
    .word 1, 4, 6, 9
    .word 2, 7, 3, 5

    .balign 16
a_col:
    .word 0, 3, 1, 2
    .word 5, 2, 1, 4

    .balign 16
b_mat:
    .word 0, 0, 0, 0
    .word 1, 1, 1, 1
    .word 2, 2, 2, 2
    .word 3, 3, 3, 3
    .word 4, 4, 4, 4
    .word 5, 5, 5, 5
    .word 6, 6, 6, 6

    .balign 16
scratch:
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0

    .balign 16
expected:
    .word 0, 0, 0, 0
    .word 3, 3, 3, 3
    .word 1, 1, 1, 1
    .word 2, 2, 2, 2

RVTEST_DATA_END
