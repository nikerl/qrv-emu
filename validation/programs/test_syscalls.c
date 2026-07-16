// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

static int failures = 0;

#define CHECK(cond, msg) \
    do { \
        if (cond) { \
            printf("PASS: %s\n", msg); \
        } else { \
            printf("FAIL: %s\n", msg); \
            failures++; \
        } \
    } while (0)

int main(void) {
    // write (stdout) - if this doesn't work you won't see any output at all
    printf("== syscall test start ==\n");

    // brk / malloc+free
    int* heap_block = malloc(64 * sizeof(int));
    CHECK(heap_block != NULL, "malloc returned non-null");
    for (int i = 0; i < 64; i++) {
        heap_block[i] = i;
    }
    int sum = 0;
    for (int i = 0; i < 64; i++) {
        sum += heap_block[i];
    }
    CHECK(sum == (63 * 64) / 2, "heap read/write round-trips correctly");
    free(heap_block);

    // a second, larger allocation, to exercise brk growing again
    char* big_block = malloc(4096);
    CHECK(big_block != NULL, "second malloc (4096 bytes) returned non-null");
    memset(big_block, 0xAB, 4096);
    CHECK((unsigned char)big_block[0] == 0xAB &&
          (unsigned char)big_block[4095] == 0xAB,
          "large heap block read/write at both ends");
    free(big_block);

    // open (write, create, truncate) + write + close
    const char* path = "syscall_test_tmp.txt";
    FILE* wf = fopen(path, "w");
    CHECK(wf != NULL, "fopen for write succeeded");
    if (wf) {
        int written = fprintf(wf, "hello syscalls\n");
        CHECK(written > 0, "fprintf wrote data");
        fclose(wf);
    }

    // open (read) + read + lseek + close
    FILE* rf = fopen(path, "r");
    CHECK(rf != NULL, "fopen for read succeeded");
    if (rf) {
        char buf[64] = {0};
        char* got = fgets(buf, sizeof(buf), rf);
        CHECK(got != NULL, "fgets read a line");
        CHECK(strncmp(buf, "hello syscalls", 14) == 0, "read content matches what was written");

        // lseek back to start and read again to confirm seeking works
        int rc = fseek(rf, 0, SEEK_SET);
        CHECK(rc == 0, "fseek(SEEK_SET) returned success");

        char buf2[64] = {0};
        got = fgets(buf2, sizeof(buf2), rf);
        CHECK(got != NULL, "fgets after seek read a line");
        CHECK(strcmp(buf, buf2) == 0, "content after seek matches first read");

        // lseek to end to check offset math
        rc = fseek(rf, 0, SEEK_END);
        long end_pos = ftell(rf);
        CHECK(rc == 0 && end_pos > 0, "fseek(SEEK_END)/ftell reports nonzero file size");

        fclose(rf);
    }

    // clock_gettime - just check it doesn't error and moves forward
    struct timespec t1, t2;
    int rc1 = time(NULL);
    volatile long spin = 0;
    for (long i = 0; i < 2000000; i++) spin += i;
    int rc2 = time(NULL);

    CHECK(rc1 < rc2 , "time doesnt move backwards");

    // read from stdin - only runs if the harness pipes something in;
    // otherwise this will just show as informational, not pass/fail
    printf("(optional) type a line for stdin read test, or press ctrl-d: \n");
    char stdin_buf[64] = {0};
    if (fgets(stdin_buf, sizeof(stdin_buf), stdin) != NULL) {
        printf("stdin echo: %s", stdin_buf);
    } else {
        printf("no stdin provided, skipping\n");
    }

    printf("== syscall test done: %d failure(s) ==\n", failures);

    if (failures > 0) {
        return 1;
    }
    return 0;
}
