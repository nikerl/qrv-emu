# Copyright 2026
# Apache License, Version 2.0, see LICENSE for details.
#
# Author: Nik Erlandsson

#include "riscv_test.h"
#include "test_macros.h"
#include "quadrisparse.h"

RVTEST_RV32U
RVTEST_CODE_BEGIN

mmasa_test:
    li TESTNUM, 2

    la x20, expected                # expected result for comparison
    la x21, a_mat                   # a base address
    la x22, b_mat                   # b base address
    la x23, c_mat                   # c base address
    li x24, 4                       # stride
    .word ENCODE_MLD(0, 21, 24)     # mld m0 x21 x24
    .word ENCODE_MLD(1, 22, 24)     # mld m0 x22 x24
    .word ENCODE_MMASA(2, 0, 1)     # mmasa m2 m0 m1
    .word ENCODE_MST(2, 23, 24)     # mst m2 x23 x24

CHECK_MAT_RESULT(x23, x20)

TEST_PASSFAIL
RVTEST_CODE_END


    .data
RVTEST_DATA_BEGIN

    .balign 16
a_mat:
    .word 1, 2, 3, 4
    .word 1, 2, 3, 4
    .word 1, 2, 3, 4
    .word 1, 2, 3, 4

    .balign 16
b_mat:
    .word 6, 7, 8, 9
    .word 6, 7, 8, 9
    .word 6, 7, 8, 9
    .word 6, 7, 8, 9

    .balign 16
c_mat:
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0

    .balign 16
expected:
    .word 80, 80, 80, 80
    .word 80, 80, 80, 80
    .word 80, 80, 80, 80
    .word 80, 80, 80, 80

RVTEST_DATA_END
