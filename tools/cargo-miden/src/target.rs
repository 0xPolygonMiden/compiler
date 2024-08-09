use std::{
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{bail, Result};

pub const WASM32_WASI_TARGET: &str = "wasm32-wasip1";

pub fn install_wasm32_wasi() -> Result<()> {
    log::info!("Installing {WASM32_WASI_TARGET} target");
    let sysroot = get_sysroot()?;
    if sysroot.join(format!("lib/rustlib/{}", WASM32_WASI_TARGET)).exists() {
        return Ok(());
    }

    if env::var_os("RUSTUP_TOOLCHAIN").is_none() {
        bail!(
            "failed to find the `{WASM32_WASI_TARGET}` target and `rustup` is not available. If \
             you're using rustup make sure that it's correctly installed; if not, make sure to \
             install the `{WASM32_WASI_TARGET}` target before using this command",
        );
    }

    let output = Command::new("rustup")
        .arg("target")
        .arg("add")
        .arg(WASM32_WASI_TARGET)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!("failed to install the `{WASM32_WASI_TARGET}` target");
    }

    Ok(())
}

fn get_sysroot() -> Result<PathBuf> {
    let output = Command::new("rustc").arg("--print").arg("sysroot").output()?;

    if !output.status.success() {
        bail!(
            "failed to execute `rustc --print sysroot`, command exited with error: {output}",
            output = String::from_utf8_lossy(&output.stderr)
        );
    }

    let sysroot = PathBuf::from(String::from_utf8(output.stdout)?.trim());

    Ok(sysroot)
}
