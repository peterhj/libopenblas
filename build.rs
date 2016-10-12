use std::env;
//use std::fs::{create_dir_all};
use std::path::{PathBuf};
use std::process::{Command};

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let out_dir = env::var("OUT_DIR").unwrap();

  let cc = env::var("CC").unwrap_or(format!("gcc"));
  let cxx = env::var("CXX").unwrap_or(format!("g++"));
  let fc = env::var("FC").unwrap_or(format!("gfortran"));

  let mut openblas_lib_dst_path = PathBuf::from(&out_dir);
  openblas_lib_dst_path.push("libopenblas_sequential.a");

  let mut openblas_par_lib_dst_path = PathBuf::from(&out_dir);
  openblas_par_lib_dst_path.push("libopenblas_parallel.a");

  if !openblas_lib_dst_path.exists() {
    let mut openblas_src_path = PathBuf::from(&manifest_dir);
    openblas_src_path.push("OpenBLAS");
    let mut openblas_build_path = PathBuf::from(&out_dir);
    openblas_build_path.push("OpenBLAS-sequential-build");
    let mut openblas_exports_path = PathBuf::from(&openblas_build_path);
    openblas_exports_path.push("exports");
    let mut openblas_lib_path = PathBuf::from(&openblas_build_path);
    openblas_lib_path.push("libopenblas.a");

    Command::new("cp")
      .current_dir(&out_dir)
      .arg("-r")
      .arg(openblas_src_path.to_str().unwrap())
      .arg(openblas_build_path.to_str().unwrap())
      .status().unwrap();

    Command::new("make")
      .current_dir(&openblas_build_path)
      .env("CC",  &cc)
      .env("CXX", &cxx)
      .env("FC",  &fc)
      .arg("USE_THREAD=0")
      .arg("NO_AFFINITY=1")
      //.arg("SYMBOLPREFIX=openblas_sequential_")
      .status().unwrap();

    Command::new("make")
      .current_dir(&openblas_build_path)
      .arg("-C").arg(&openblas_exports_path)
      .arg("clean")
      .status().unwrap();

    Command::new("make")
      .current_dir(&openblas_build_path)
      .arg("-C").arg(&openblas_exports_path)
      .arg("objcopy.def")
      .arg("SYMBOLPREFIX=openblas_sequential_")
      .status().unwrap();

    Command::new("objcopy")
      .current_dir(&openblas_build_path)
      .arg("--redefine-syms")
      .arg(&openblas_exports_path.join("objcopy.def"))
      .arg(&openblas_lib_path)
      .status().unwrap();

    Command::new("cp")
      .current_dir(&out_dir)
      .arg(openblas_lib_path.to_str().unwrap())
      .arg(openblas_lib_dst_path.to_str().unwrap())
      .status().unwrap();
  }

  if !openblas_par_lib_dst_path.exists() {
    let mut openblas_src_path = PathBuf::from(&manifest_dir);
    openblas_src_path.push("OpenBLAS");
    let mut openblas_build_path = PathBuf::from(&out_dir);
    openblas_build_path.push("OpenBLAS-parallel-build");
    let mut openblas_exports_path = PathBuf::from(&openblas_build_path);
    openblas_exports_path.push("exports");
    let mut openblas_lib_path = PathBuf::from(&openblas_build_path);
    openblas_lib_path.push("libopenblas.a");

    Command::new("cp")
      .current_dir(&out_dir)
      .arg("-r")
      .arg(openblas_src_path.to_str().unwrap())
      .arg(openblas_build_path.to_str().unwrap())
      .status().unwrap();

    Command::new("make")
      .current_dir(&openblas_build_path)
      .env("CC",  &cc)
      .env("CXX", &cxx)
      .env("FC",  &fc)
      .arg("USE_THREAD=1")
      .arg("NO_AFFINITY=1")
      //.arg("SYMBOLPREFIX=openblas_parallel_")
      .status().unwrap();

    Command::new("make")
      .current_dir(&openblas_build_path)
      .arg("-C").arg(&openblas_exports_path)
      .arg("clean")
      .status().unwrap();

    Command::new("make")
      .current_dir(&openblas_build_path)
      .arg("-C").arg(&openblas_exports_path)
      .arg("objcopy.def")
      .arg("SYMBOLPREFIX=openblas_parallel_")
      .status().unwrap();

    Command::new("objcopy")
      .current_dir(&openblas_build_path)
      .arg("--redefine-syms")
      .arg(&openblas_exports_path.join("objcopy.def"))
      .arg(&openblas_lib_path)
      .status().unwrap();

    Command::new("cp")
      .current_dir(&out_dir)
      .arg(openblas_lib_path.to_str().unwrap())
      .arg(openblas_par_lib_dst_path.to_str().unwrap())
      .status().unwrap();
  }

  //println!("cargo:rustc-link-search=native={}", artifacts_path.to_str().unwrap());
  println!("cargo:rustc-link-search=native={}", out_dir);
}
