use bindgen;

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
  println!("Running link-grammar build.rs!");
  println!("{}", env::current_dir().unwrap().display());
  println!("cargo:rerun-if-changed=build.rs");
  // Tell cargo to use the locally installed link-grammar
  // This isn't great, and needs to change in the future, but for now it's fine.
  println!("cargo:rustc-link-lib=link-grammar");
  // Tell cargo to invalidate the built crate whenever the wrapper changes
  println!("cargo:rerun-if-changed=wrapper.h");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  // Build the link-grammar bindings
  let bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");

  Ok(())
}
