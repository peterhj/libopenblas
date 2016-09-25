use std::env;
//use std::fs::{create_dir_all};
use std::path::{PathBuf};
use std::process::{Command};

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let out_dir = env::var("OUT_DIR").unwrap();
  //let target_triple = env::var("TARGET").unwrap();

  let mut openblas_lib_dst_path = PathBuf::from(&out_dir);
  openblas_lib_dst_path.push("libopenblas_native.a");
  if !openblas_lib_dst_path.exists() {
    /*let mut openblas_src_path = PathBuf::from(&manifest_dir);
    openblas_src_path.push("openblas-0.2.19.tar.gz");
    Command::new("tar")
      .current_dir(&out_dir)
      .arg("-xzkf")
      .arg(openblas_src_path.to_str().unwrap())
      .status().unwrap();*/
    let mut openblas_src_path = PathBuf::from(&manifest_dir);
    openblas_src_path.push("OpenBLAS");
    let mut openblas_build_path = PathBuf::from(&out_dir);
    //openblas_build_path.push("OpenBLAS-0.2.19");
    openblas_build_path.push("OpenBLAS");
    Command::new("cp")
      .current_dir(&out_dir)
      .arg("-r")
      .arg(openblas_src_path.to_str().unwrap())
      .arg(openblas_build_path.to_str().unwrap())
      .status().unwrap();

    // CC=gcc-4.9 CXX=g++-4.9 FC=gfortran-4.9 make USE_THREAD=0 NO_AFFINITY=1
    Command::new("make")
      .current_dir(&openblas_build_path)
      .env("CC",  "gcc-4.9")
      .env("CXX", "g++-4.9")
      .env("FC",  "gfortran-4.9")
      .arg("USE_THREAD=0")
      .arg("NO_AFFINITY=1")
      .status().unwrap();

    let mut openblas_lib_path = PathBuf::from(&openblas_build_path);
    openblas_lib_path.push("libopenblas.a");
    Command::new("cp")
      .current_dir(&out_dir)
      .arg(openblas_lib_path.to_str().unwrap())
      .arg(openblas_lib_dst_path.to_str().unwrap())
      .status().unwrap();
  }

  /*let mut artifacts_path = PathBuf::from(&manifest_dir);
  artifacts_path.push(&format!("artifacts.{}", target_triple));
  create_dir_all(&artifacts_path).ok();

  let mut native_lib_path = PathBuf::from(&artifacts_path);
  native_lib_path.push("libopenblas_native.a");
  if !native_lib_path.exists() {
    unimplemented!();
  }*/

  //println!("cargo:rustc-link-search=native={}", artifacts_path.to_str().unwrap());
  println!("cargo:rustc-link-search=native={}", out_dir);
}
