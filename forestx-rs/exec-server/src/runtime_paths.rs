use std::path::PathBuf;

use forestx_utils_absolute_path::AbsolutePathBuf;

/// Runtime paths needed by exec-server child processes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecServerRuntimePaths {
    /// Stable path to the Forestx executable used to launch hidden helper modes.
    pub forestx_self_exe: AbsolutePathBuf,
    /// Path to the Linux sandbox helper alias used when the platform sandbox
    /// needs to re-enter Forestx by argv0.
    pub forestx_linux_sandbox_exe: Option<AbsolutePathBuf>,
}

impl ExecServerRuntimePaths {
    pub fn from_optional_paths(
        forestx_self_exe: Option<PathBuf>,
        forestx_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        let forestx_self_exe = forestx_self_exe.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Forestx executable path is not configured",
            )
        })?;
        Self::new(forestx_self_exe, forestx_linux_sandbox_exe)
    }

    pub fn new(
        forestx_self_exe: PathBuf,
        forestx_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            forestx_self_exe: absolute_path(forestx_self_exe)?,
            forestx_linux_sandbox_exe: forestx_linux_sandbox_exe.map(absolute_path).transpose()?,
        })
    }
}

fn absolute_path(path: PathBuf) -> std::io::Result<AbsolutePathBuf> {
    AbsolutePathBuf::from_absolute_path(path.as_path())
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
}
