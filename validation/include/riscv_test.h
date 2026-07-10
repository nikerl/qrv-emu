// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

#ifndef _RISCV_QRVEMU_TEST_H
#define _RISCV_QRVEMU_TEST_H

#ifndef TEST_FUNC_NAME
#  define TEST_FUNC_NAME inst_test
#  define TEST_FUNC_TXT "inst_test"
#  define TEST_FUNC_RET inst_test_ret
#endif

#define RVTEST_RV32U
#define TESTNUM x28

#define RVTEST_CODE_BEGIN \
	.text; \
	.global _start; \
	.global TEST_FUNC_NAME; \
	.global TEST_FUNC_RET; \
TEST_FUNC_NAME: \
	lui		a0,%hi(.test_name);	\
	addi	a0,a0,%lo(.test_name); \
	lui		a2,0x10001000>>12; \
.prname_next: \
	lb		a1,0(a0); \
	beq		a1,zero,.prname_done; \
	sw		a1,0(a2); \
	addi	a0,a0,1; \
	jal		zero,.prname_next; \
.test_name:	\
	.ascii 	TEST_FUNC_TXT; \
	.byte 	0x00; \
	.balign 4, 0; \
.prname_done: \
	addi	a1,zero,'.'; \
	sw		a1,0(a2); \
	sw		a1,0(a2);

#define RVTEST_FAIL \
    li 		a0, 0x10000000; \
    li 		a1, 3; \
    sw 		a1, 0(a0);

#define RVTEST_PASS \
    li 		a0, 0x10000000; \
    li 		a1, 1; \
    sw 		a1, 0(a0);

#define RVTEST_CODE_END
#define RVTEST_DATA_BEGIN .balign 4;
#define RVTEST_DATA_END

#endif
