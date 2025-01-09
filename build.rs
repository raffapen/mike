
use std::process::{Command, Stdio};
use std::error::Error;

extern crate classico;
use classico::wd40;
use classico::wd40::blog;

fn main() -> Result<(), Box<dyn Error>> {
    let mut build = wd40::Build::new()?;

    let root = &build.root();
    let libmake_root = format!("{}/make", root);

    let mut make = Command::new("bash");
    make.arg("-c");
    make.arg(format!("make SEP=0 ROOT={} {} 2>&1",
            &build.mk_root(), if build.is_debug() { "DEBUG=1"} else { "" }));
    make.current_dir(&libmake_root);
    make.stdout(Stdio::piped());

    blog!(build, "\n# in {}, running: {}", libmake_root, wd40::command_str(&make));
    
    let make_out = make.output()    
        .expect("'make' failed to execute");
    build.blog(&make_out.stdout);
    if !make_out.status.success() {
        //println!("\nOutput:\n{}", String::from_utf8_lossy(&make_out.stdout));
        return Err("libmake build failed".into());
        //panic!("libmake build failed");
    }

    let libmake_h = format!("{}/libmake_rs.h", &libmake_root);
    wd40::generate_bindings(&libmake_h, "libmake.rs");

    println!("cargo:rustc-link-lib=libmake");
    println!("cargo:rustc-link-search=native={}/make", build.bindir());

    Ok(())
}
