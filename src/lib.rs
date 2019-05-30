use cmake;
use std::path::{Path, PathBuf};

pub struct Artifacts {
    include_dir: PathBuf,
    lib_dir: PathBuf,
}

impl Artifacts {
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }
}

pub struct Build;

impl Build {
    pub fn new() -> Build {
        Build
    }

    pub fn build(&self) -> Artifacts {
        let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("libressl");
        let dst = cmake::build(format!("{}", src_dir.display()));
        Artifacts {
            include_dir: dst.join("include"),
            lib_dir: dst.join("lib"),
        }
    }
}
