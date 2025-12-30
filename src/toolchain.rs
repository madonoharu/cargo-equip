use crate::process::ProcessBuilderExt as _;
use anyhow::anyhow;
use camino::Utf8Path;
use cargo_util::ProcessBuilder;
use std::{
    env,
    path::{Path, PathBuf},
};

pub(crate) fn rustup_exe(cwd: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
    which::which_in("rustup", env::var_os("PATH"), cwd).map_err(|_| anyhow!("`rustup` not found"))
}

pub(crate) fn active_toolchain(manifest_dir: &Utf8Path) -> anyhow::Result<String> {
    let output = ProcessBuilder::new(rustup_exe(manifest_dir)?)
        .args(&["show", "active-toolchain"])
        .cwd(manifest_dir)
        .read_stdout::<String>()?;
    Ok(output.split_whitespace().next().unwrap().to_owned())
}
