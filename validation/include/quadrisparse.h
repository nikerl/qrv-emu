// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

// compile-time macros instead of functions, so args stay constant expressions
#define ENCODE_MLD(md, base, stride) \
    (0b0101011 | ((md) << 7) | ((base) << 15) | ((stride) << 20) | 0b00000 << 27)

#define ENCODE_MST(ms1, base, stride) \
    (0b0101011 | ((ms1) << 7) | ((base) << 15) | ((stride) << 20) | 0b00001 << 27)
    
#define ENCODE_SPLD(md, im1) \
    (0b0101011 | ((md) << 7) | ((im1) << 15) | 0b00100 << 27)

#define ENCODE_DLD(md, ms1) \
    (0b0101011 | ((md) << 7) | ((ms1) << 15) | 0b00010 << 27)

#define ENCODE_MMASA(md, ms1, ms2) \
    (0b0101011 | ((md) << 15) | ((ms1) << 18) | ((ms2) << 21) | 0b11110 << 27)

#define ENCODE_SPMAC(md, ms1, ms2) \
    (0b0101011 | ((md) << 15) | ((ms1) << 18) | ((ms2) << 21) | 0b01000 << 27)

#define ENCODE_MZERO(md) \
    (0b0101011 | ((md) << 15) | 0b11111 << 27)
