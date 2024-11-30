// small, fast 64 bit hash function.
//
// https://github.com/N-R-K/ChibiHash
// https://nrk.neocities.org/articles/chibihash
//
// This is free and unencumbered software released into the public domain.
// For more information, please refer to <https://unlicense.org/>

#ifndef CHIBIHASH_H
#define CHIBIHASH_H

#include <stdint.h>
#include <stddef.h>

// Remove static inline and expose the function
uint64_t chibihash64_v1(const void *keyIn, ptrdiff_t len, uint64_t seed);
uint64_t chibihash64_v2(const void *keyIn, ptrdiff_t len, uint64_t seed);


#endif