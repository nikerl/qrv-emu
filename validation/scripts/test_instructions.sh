#!/usr/bin/env bash 

# Copyright 2026
# Apache License, Version 2.0, see LICENSE for details.
#
# Author: Nik Erlandsson

BINARY_PATH="bin"
pass=0
fail=0
total=0
failed_tests=()

TEST_SUITES=()

case "$1" in
    rv32im)
        TEST_SUITES=("validation/rv32im-tests")
        ;;
    quadrisparse)
        TEST_SUITES=("validation/quadrisparse-tests")
        ;;
    "")
        TEST_SUITES=("validation/rv32im-tests" "validation/quadrisparse-tests")
        ;;
    -h)
        echo "Usage: $0 [rv32im|quadrisparse]"
        exit 1
        ;;
esac

echo ""
for test_path in "${TEST_SUITES[@]}"; do
    suite_name=$(basename "$test_path")

    echo "===================="
    echo "${suite_name^^}"
    echo ""

    for instruction in "$test_path"/*; do
        filename=$(basename "$instruction")
        stripped_instruction="${filename%.*}"
        total=$((total + 1))

        output=$(make run-test INSTR=$stripped_instruction 2>&1)

        if echo "$output" | grep -q "Test PASSED"; then
            echo "$stripped_instruction: PASSED"
            pass=$((pass + 1))
        elif echo "$output" | grep -q "Test FAILED"; then
            echo "$stripped_instruction: FAILED"
            fail=$((fail + 1))
            failed_tests+=("$stripped_instruction")
        else
            echo "$stripped_instruction: UNKNOWN"
            echo "$output"
            fail=$((fail + 1))
            failed_tests+=("$stripped_instruction")
        fi

        rm $BINARY_PATH/${stripped_instruction}_test.elf
    done
    echo ""
done

echo "===================="
echo "Passed $pass/$total tests"
if [ ${#failed_tests[@]} -gt 0 ]; then
    echo "Failed: ${failed_tests[*]}"
    exit 1
fi

exit 0
