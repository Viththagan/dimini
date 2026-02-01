use anyhow::{Context, Result};
use std::collections::HashSet;
use std::process::Command;

pub fn installed() -> Result<HashSet<String>> {
    let out = Command::new("dpkg-query")
        .args(["--show", "--showformat=${binary:Package}\n"])
        .output()
        .context("failed to run dpkg-query")?;

    if !out.status.success() {
        anyhow::bail!("dpkg-query failed");
    }

    let stdout = String::from_utf8(out.stdout)?;

    let pkgs = stdout
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect::<HashSet<_>>();

    Ok(pkgs)
}
