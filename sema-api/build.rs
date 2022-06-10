fn main() -> Result<(), nlprule_build::Error> {
  println!("cargo:rerun-if-changed=build.rs");

  let mut opts = built::Options::default();
  opts.set_env(true);
  opts.set_git(true);
  opts.set_time(true);

  let src = std::env::var("CARGO_MANIFEST_DIR").unwrap();
  let dst = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("built.rs");
  built::write_built_file_with_opts(&opts, src.as_ref(), &dst)
    .expect("Failed to acquire build-time information");

  

  let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is set when build.rs is running");
  println!("out_dir dude!: {}", out_dir);

  nlprule_build::BinaryBuilder::new(&["en"], out_dir)
    .build()?
    .validate()
}
