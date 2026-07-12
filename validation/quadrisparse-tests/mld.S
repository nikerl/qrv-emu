# Copyright 2026
# Apache License, Version 2.0, see LICENSE for details.
#
# Author: Nik Erlandsson

#include "riscv_test.h"
#include "test_macros.h"
#include "quadrisparse.h"

RVTEST_RV32U
RVTEST_CODE_BEGIN

mld_test:
    li TESTNUM, 2

    la x20, expected                # load base / expected result for comparison
    la x21, scratch                 # store base address
    li x22, 4                       # stride
    .word ENCODE_MLD(0, 20, 22)     # mld m0 x20 x22
    .word ENCODE_MST(0, 21, 22)     # mst m0 x21 x22

CHECK_MAT_RESULT(x20, x21)

TEST_PASSFAIL
RVTEST_CODE_END


    .data
RVTEST_DATA_BEGIN

    .balign 16
scratch:
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0

    .balign 16
expected:
    .word 1, 1, 1, 1
    .word 2, 2, 2, 2
    .word 3, 3, 3, 3
    .word 4, 4, 4, 4

RVTEST_DATA_END
