# libopenblas

This is a Rust binding to OpenBLAS (https://github.com/xianyi/OpenBLAS).

This particular library actually compiles _two_ copies of OpenBLAS: a sequential
version, and a thread-parallel version (not the OpenMP one). The sequential BLAS
routines are prefixed by `openblas_sequential_`, and likewise the parallel BLAS
routines are prefixed by `openblas_parallel_`.

Only the CBLAS API is likely to be supported.

## Requirements

Building OpenBLAS requires a GCC toolchain (`gcc` and `gfortran`), as well as
`objcopy` from binutils to rename symbols for our dual sequential-parallel
versioning.
