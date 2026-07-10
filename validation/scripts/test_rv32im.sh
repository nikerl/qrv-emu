# Copyright 2026
# Apache License, Version 2.0, see LICENSE for details.
#
# Author: Nik Erlandsson

RV32IM_TESTS="validation/rv32im-tests"
BINARY_PATH="bin"
pass=0
fail=0
total=0
failed_tests=()

for instruction in $RV32IM_TESTS/*; do
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
echo "===================="
echo "Passed $pass/$total tests"
if [ ${#failed_tests[@]} -gt 0 ]; then
    echo "Failed: ${failed_tests[*]}"
fi
