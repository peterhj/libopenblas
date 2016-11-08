extern crate libc;

use libc::*;

/*#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CblasOrder {
  RowMajor  = 101,
  ColMajor  = 102,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CblasTranspose {
  NoTrans       = 111,
  Trans         = 112,
  ConjTrans     = 113,
  ConjNoTrans   = 114,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CblasUpLo {
  Upper = 121,
  Lower = 122,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CblasDiag {
  NonUnit   = 131,
  Unit      = 132,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CblasSide {
  Left  = 141,
  Right = 142,
}

#[link(name = "openblas_sequential", kind = "static")]
extern "C" {
  pub fn openblas_sequential_cblas_snrm2(
      n: c_int,
      x: *const f32,
      incx: c_int,
  ) -> f32;
  pub fn openblas_sequential_cblas_sdot(
      n: c_int,
      alpha: f32,
      x: *const f32,
      incx: c_int,
      y: *const f32,
      incy: c_int,
  ) -> f32;
  pub fn openblas_sequential_cblas_sscal(
      n: c_int,
      alpha: f32,
      x: *mut f32,
      incx: c_int,
  );
  pub fn openblas_sequential_cblas_saxpy(
      n: c_int,
      alpha: f32,
      x: *const f32,
      incx: c_int,
      y: *mut f32,
      incy: c_int,
  );
  pub fn openblas_sequential_cblas_sgemv(
      order: CblasOrder,
      trans: CblasTranspose,
      m: c_int,
      n: c_int,
      alpha: f32,
      a: *const f32,
      lda: c_int,
      x: *const f32,
      incx: c_int,
      beta: f32,
      y: *mut f32,
      incy: c_int,
  );
  pub fn openblas_sequential_cblas_sgemm(
      order: CblasOrder,
      trans_a: CblasTranspose,
      trans_b: CblasTranspose,
      m: c_int,
      n: c_int,
      k: c_int,
      alpha: f32,
      a: *const f32,
      lda: c_int,
      b: *const f32,
      ldb: c_int,
      beta: f32,
      c: *mut f32,
      ldc: c_int,
  );
}

#[link(name = "openblas_parallel", kind = "static")]
extern "C" {
  pub fn openblas_parallel_cblas_snrm2(
      n: c_int,
      x: *const f32,
      incx: c_int,
  ) -> f32;
  pub fn openblas_parallel_cblas_sdot(
      n: c_int,
      alpha: f32,
      x: *const f32,
      incx: c_int,
      y: *const f32,
      incy: c_int,
  ) -> f32;
  pub fn openblas_parallel_cblas_sscal(
      n: c_int,
      alpha: f32,
      x: *mut f32,
      incx: c_int,
  );
  pub fn openblas_parallel_cblas_saxpy(
      n: c_int,
      alpha: f32,
      x: *const f32,
      incx: c_int,
      y: *mut f32,
      incy: c_int,
  );
  pub fn openblas_parallel_cblas_sgemv(
      order: CblasOrder,
      trans: CblasTranspose,
      m: c_int,
      n: c_int,
      alpha: f32,
      a: *const f32,
      lda: c_int,
      x: *const f32,
      incx: c_int,
      beta: f32,
      y: *mut f32,
      incy: c_int,
  );
  pub fn openblas_parallel_cblas_sgemm(
      order: CblasOrder,
      trans_a: CblasTranspose,
      trans_b: CblasTranspose,
      m: c_int,
      n: c_int,
      k: c_int,
      alpha: f32,
      a: *const f32,
      lda: c_int,
      b: *const f32,
      ldb: c_int,
      beta: f32,
      c: *mut f32,
      ldc: c_int,
  );
}*/
