# Copyright 2026
# Apache License, Version 2.0, see LICENSE for details.
#
# Author: Nik Erlandsson

#include "riscv_test.h"
#include "test_macros.h"
#include "quadrisparse.h"

RVTEST_RV32U
RVTEST_CODE_BEGIN

mzero_test:
    li TESTNUM, 2

    la x20, expected                # load base / expected result for comparison
    la x21, scratch                 # store base address
    li x22, 4                       # stride
    .word ENCODE_MLD(0, 21, 22)     # mld m0 x21 x22
    .word ENCODE_MZERO(0)           # mzero m0
    .word ENCODE_MST(0, 21, 22)     # mst m0 x21 x22

CHECK_MAT_RESULT(x20, x21)

TEST_PASSFAIL
RVTEST_CODE_END


    .data
RVTEST_DATA_BEGIN

    .balign 16
scratch:
    .word 0xdeadbeef, 0xdeadbeef, 0xdeadbeef, 0xdeadbeef
    .word 0xdeadbeef, 0xdeadbeef, 0xdeadbeef, 0xdeadbeef
    .word 0xdeadbeef, 0xdeadbeef, 0xdeadbeef, 0xdeadbeef
    .word 0xdeadbeef, 0xdeadbeef, 0xdeadbeef, 0xdeadbeef

    .balign 16
expected:
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0
    .word 0, 0, 0, 0

RVTEST_DATA_END
