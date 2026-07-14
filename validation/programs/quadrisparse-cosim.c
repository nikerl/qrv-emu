// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include "quadrisparse.h"

void load_matrix(int* array, char* path, int num_elements) {
    FILE *fptr = fopen(path, "r");
    char buf[16];

    int index = 0;
    while (fgets(buf, sizeof(buf), fptr) && index < num_elements) {
        array[index] = (int)strtol(buf, NULL, 16);
        index++;
    }

    fclose(fptr);
}

int count_lines(const char* path) {
    FILE* fp = fopen(path, "r");
    if (!fp) return -1;

    int lines = 0;
    int c;
    while ((c = fgetc(fp)) != EOF) {
        if (c == '\n') lines++;
    }

    fclose(fp);
    return lines;
}

void issue_instruction(uint32_t instr, uint32_t rs1_val, uint32_t rs2_val) {
    static uint32_t code_buf[2];
    code_buf[0] = instr;
    code_buf[1] = 0x00008067; // ret (jalr x0, ra, 0)

    register uint32_t r_x20 asm("x20") = rs1_val;
    register uint32_t r_x21 asm("x21") = rs2_val;

    asm volatile (
        "jalr ra, %[buf], 0\n"
        :
        : [buf] "r" (code_buf), "r" (r_x20), "r" (r_x21)
        : "ra", "memory"
    );
}

int main(void) {
    int a_nnz = count_lines("mat_32_0.9_a_val.hex");
    int n_cols = 32;

    int a_val[a_nnz];
    int a_col[a_nnz];
    int a_row[n_cols + 1];
    int b[n_cols * n_cols];
    int c[n_cols * n_cols];
    int ref[n_cols * n_cols];

    load_matrix(a_val, "mat_32_0.9_a_val.hex", a_nnz);
    load_matrix(a_col, "mat_32_0.9_a_col.hex", a_nnz);
    load_matrix(a_row, "mat_32_0.9_a_row.hex", n_cols + 1);
    load_matrix(b, "mat_32_0.9_b.hex", n_cols * n_cols);
    load_matrix(ref, "mat_32_0.9_ref.hex", n_cols * n_cols);

    int sparse_reg = 0;
    int dense_regs[2] = {1, 2};
    int acc_regs[4] = {4, 5, 6, 7};

    for (int row_idx = 0; row_idx < n_cols; row_idx++) {
        // Skip empty rows
        if (a_row[row_idx] == a_row[row_idx + 1]) {
            continue;
        } 

        for (int col_tile_start = 0; col_tile_start < n_cols / 4; col_tile_start += 4) {
            int tiles_in_group = n_cols / 4 - col_tile_start;
            if (tiles_in_group > 4) {
                tiles_in_group = 4;
            }

            // Zero all accumulator registers in this group
            for (int tile = 0; tile < tiles_in_group; tile++){
                issue_instruction(ENCODE_MZERO(acc_regs[tile]), 0, 0);
            }

            // Walk the sparse row once per tile group and reuse each sparse chunk
            for (int val_ptr = a_row[row_idx]; val_ptr < a_row[row_idx + 1]; ) {
                int nnz_to_load = a_row[row_idx + 1] - val_ptr;

                int chunk_limit = 4 - (val_ptr & 0b11);
                if (nnz_to_load > chunk_limit) nnz_to_load = chunk_limit;
                if (nnz_to_load > 4) nnz_to_load = 4;


                issue_instruction(ENCODE_SPLD(sparse_reg, nnz_to_load), (unsigned int)(uintptr_t)(a_val + val_ptr), (unsigned int)(uintptr_t)(a_col + val_ptr));
                val_ptr += nnz_to_load;

                for (int tile = 0; tile < tiles_in_group; tile++) {
                    int col_tile_idx = col_tile_start + tile;

                    issue_instruction(ENCODE_DLD(dense_regs[tile % 2], sparse_reg), (unsigned int)(uintptr_t)(b + col_tile_idx * 4), n_cols);
                    issue_instruction(ENCODE_SPMAC(acc_regs[tile], sparse_reg, dense_regs[tile % 2]), 0, 0);
                }
            }

            // Store the completed output tile group
            for (int tile = 0; tile < tiles_in_group; tile++) {
                int col_tile_idx = col_tile_start + tile;
                issue_instruction(ENCODE_MST(acc_regs[tile], 20, 21), (unsigned int)(uintptr_t)(c + row_idx * n_cols + col_tile_idx * 4), n_cols);
            }
        }
    }

    int fail = 0;
    for (int i = 0; i < n_cols * n_cols; i++) {
        if (c[i] != ref[i]) {
            printf("mismatch at row=%d col=%d (i=%d): c=%d ref=%d\n", i / n_cols, i % n_cols, i, c[i], ref[i]);
            fail = 1;
        }
    }
    if (fail) {
        printf("FAIL, Matrix doesnt match refernce\n");
        return 1;
    }
    printf("PASS, All elements match\n");
    return 0;
}
