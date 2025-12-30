use std::path::PathBuf;

use crate::{Task, BAKERY_BUILD_DIRECTORY, BAKERY_CACHE_DIRECTORY};

pub struct Clean {}

impl Clean {
    pub fn new() -> Self {
        Clean {}
    }

    pub fn remove_cache(&self, base_path: PathBuf) -> Result<(), std::io::Error> {
        let cache = base_path.join(BAKERY_CACHE_DIRECTORY);
        if cache.exists() {
            std::fs::remove_dir_all(cache)?;
        }
        Ok(())
    }

    pub fn remove_build(&self, base_path: PathBuf) -> Result<(), std::io::Error> {
        let build = base_path.join(BAKERY_BUILD_DIRECTORY);
        if build.exists() {
            std::fs::remove_dir_all(build)?;
        }
        Ok(())
    }
}

impl Task for Clean {
    fn id(&self) -> &'static str {
        &"clean"
    }

    fn dependencies(&self) -> &[&'static str] {
        &[]
    }

    fn on_execute(&mut self, context: &super::TaskContext) {
        let base = context.project.base_path.clone();

        if let Err(err) = self.remove_cache(base.clone()) {
            eprintln!("Failed to clean project cache: {}", err);
        }

        if let Err(err) = self.remove_build(base.clone()) {
            eprintln!("Failed to clean project build: {}", err);
        }
    }
}
