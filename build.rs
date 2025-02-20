use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use copy_dir::copy_dir;

pub struct BitwuzlaBuild {
    src_dir: PathBuf,
    out_dir: PathBuf,
}

impl BitwuzlaBuild {
    pub fn new() -> Self {
        Self {
            src_dir: Path::new(env!("CARGO_MANIFEST_DIR")).join("bitwuzla"),
            out_dir: Path::new(&env::var_os("OUT_DIR").expect("`OUT_DIR` not set"))
                .join("vendor-build"),
        }
    }

    pub fn prerequisites(mut self) -> Self {
        println!("source dir: {}", self.src_dir.display());
        println!("Build dir: {}", self.out_dir.display());
        if !self.out_dir.exists() {
            self.run_command(
                "Mkdir for vendor data",
                Command::new("mkdir").arg(self.out_dir.clone()),
            );
        }

        self.out_dir = self.out_dir.join("source");
        if !self.out_dir.exists() {
            self.run_command(
                "create symlink",
                Command::new("ln")
                    .arg("-s")
                    .arg(self.src_dir.clone())
                    .arg(self.out_dir.clone()),
            );
            //copy_dir(&self.src_dir, &self.out_dir)
            //    .expect("failed to copy Bitwuzla sources to `OUT_DIR`");
        }

        if !self.out_dir.join("build").exists() {
            self.run_command(
                "Setup Bitwuzla meson project",
                Command::new("/usr/bin/env")
                    .arg("meson")
                    .arg("setup")
                    .arg("build/")
                    .arg("-Dbuildtype=release")
                    .arg("-Ddefault_library=static")
                    .current_dir(&self.out_dir),
            );
        }

        self
    }

    pub fn build(self) -> Self {
        // self.run_command(
        //     "Configure Bitwuzla",
        //     Command::new("meson")
        //         .arg("setup")
        //         .arg("build/")
        //         .arg("-Dbuildtype=release")
        //         .current_dir(&self.out_dir),
        // );

        self.run_command(
            "Build Bitwuzla",
            Command::new("ninja").current_dir(self.out_dir.join("build")),
        );

        // TODO: why are these not included in libbitwuzla.a?
        println!(
            "cargo:rustc-link-search={}",
            self.out_dir.join("build/src/lib").display()
        );
        println!("cargo:rustc-link-lib=static=bitwuzlabb");
        println!("cargo:rustc-link-lib=static=bitwuzlabv");
        println!("cargo:rustc-link-lib=static=bitwuzlals");
        println!("cargo:rustc-link-lib=static=bzlarng");

        println!(
            "cargo:rustc-link-search={}",
            self.out_dir.join("build/src").display()
        );
        println!("cargo:rustc-link-lib=static=bitwuzla");
        println!("cargo:rustc-link-lib=stdc++");
        // println!("cargo:rustc-link-lib=gmp");
        println!("cargo:rustc-link-lib=static=gmp");

        self
    }

    fn run_command(&self, description: &str, command: &mut Command) {
        println!("*** {}", description);

        let status = command.status().unwrap();

        if !status.success() {
            panic!(
                "*** ERROR in action `{}`, exit status {}\n*** Command: {:?}",
                description, status, command,
            );
        }
    }
}

fn main() {
    if std::env::var("BITWUZLA_NO_VENDOR").map_or(true, |value| value == "0") {
        BitwuzlaBuild::new().prerequisites().build();
    } else {
        println!("cargo:rustc-link-lib=static=bitwuzla");
    }
}
