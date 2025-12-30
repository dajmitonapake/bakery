mod build;
mod clean;
mod run;

pub use build::*;
pub use clean::*;
pub use run::*;

use crate::{config::ToolchainConfiguration, Project};

pub struct TaskContext {
    pub project: Project,
    pub toolchain_configuration: ToolchainConfiguration,
}

pub trait Task {
    fn id(&self) -> &'static str;
    fn dependencies(&self) -> &[&'static str];

    fn on_execute(&mut self, context: &TaskContext);
}
