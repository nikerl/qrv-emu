<img width="1983" height="550" alt="pun intended" src="https://github.com/user-attachments/assets/ede982fb-1391-495a-b2ff-abbb966c4116" />


# QRV-Emu: The QuadriSparse RISC-V Emulator
A RISC-V emulator for RV32IM + [QuadriSparse](https://github.com/nikerl/quadrisparse) written in Rust.

The ultimate goal is to create a Golden Reference Model which can be used to validate QuadriSparse against. It also enables easier development of software targeting QuadriSparse. 


## Install RISC-V Toolchain
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
export PATH=/opt/riscv/bin:PATH
```
