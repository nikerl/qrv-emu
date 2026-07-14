<img width="5000" height="1360" alt="banner" src="https://github.com/user-attachments/assets/e850d413-8313-4e37-9b00-535c98e0a58c" />



# QRV-Emu: The QuadriSparse RISC-V Emulator
A RISC-V emulator for RV32IM + [QuadriSparse](https://github.com/nikerl/quadrisparse) written in Rust.

The ultimate goal of this project is to create a Golden Reference Model which can be used to validate QuadriSparse against. It also enables easier development of software targeting QuadriSparse. The emulator supports the basic 32-bit integer RISC-V ISA with the standard multiplication extension. It also supports the custom instructions for sparse and dense matrix multiplication outlined in QuadriSparse. 

QRV-Emu is an Instruction Set Simulator (ISS) that emulates the functionality of the instruction set. It accepts ELF binaries compiled for RISC-V and so far it supports the syscalls needed for basic I/O and heap allocation (exit, open, write, read, brk).


## Setup RISC-V Toolchain
### Prebuilt binaries
xPack provides precompiled RISCV GCC distribtuion [here](https://github.com/xpack-dev-tools/riscv-none-elf-gcc-xpack)

Download and unpack:
```
sudo mkdir /opt/riscv
curl -L "https://github.com/xpack-dev-tools/riscv-none-elf-gcc-xpack/releases/download/v15.2.0-1/xpack-riscv-none-elf-gcc-15.2.0-1-linux-x64.tar.gz" -o /tmp/riscv-toolchain.tar.gz 
tar -xzf /tmp/riscv-toolchain.tar.gz -C /opt/riscv --strip-components=1
export PATH=/opt/riscv/bin:$PATH
```

### Build from source
Note this it can take up to an hour to setup.

Install requirements: 
```
sudo apt install autoconf automake autotools-dev curl python3 libmpc-dev libmpfr-dev libgmp-dev gawk build-essential bison flex texinfo gperf libtool patchutils bc zlib1g-dev libexpat-dev
```

Clone the repo and setup target dir:
```
sudo mkdir /opt/riscv
git clone --recursive https://github.com/riscv/riscv-gnu-toolchain
cd riscv-gnu-toolchain
mkdir build
```

Build:
```
cd build
../configure --prefix=/opt/riscv --with-arch=rv32im --with-abi=ilp32
sudo make -j$(nproc)
```

Add compiler to path:
```
export PATH=/opt/riscv/bin:$PATH
```

### Test the toolchain
Build a binary:
```
mkdir bin
riscv32-unknown-elf-gcc \
    -march=rv32im -mabi=ilp32 \
    -o bin/hello_world.elf \
    validation/programs/hello_world.c
```

Run the binary:
```
cargo run bin/hello_world.elf
```

This can be done with a single make command as well:
```
make run-program PROGRAM=hello_world.c
```

## Emulator Validation
To validate the correctness of the emulator and the toolchain install you can run the following script:
```
bash validation/scripts/test_instructions.sh
```

If everything is installed correctly you should see an output like this:
```
...
mst: PASSED
mzero: PASSED
spld: PASSED
spmac: PASSED

====================
Passed 52/52 tests
```
