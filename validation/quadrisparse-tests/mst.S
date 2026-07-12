# Copyright 2026
# Apache License, Version 2.0, see LICENSE for details.
#
# Author: Nik Erlandsson

#include "riscv_test.h"
#include "test_macros.h"
#include "quadrisparse.h"

RVTEST_RV32U
RVTEST_CODE_BEGIN

mst_test:
    li TESTNUM, 2

    la x10, expected                # expected result for comparison
    la x20, scratch                 # base address
    li x21, 4                       # stride
    .word ENCODE_MZERO(0)           # mzero m0
    .word ENCODE_MST(0, 20, 21)     # mst m0 x20 x21

CHECK_MAT_RESULT(x10, x20)

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
