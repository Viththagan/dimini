use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::process::Command;

pub fn installed() -> Result<HashSet<String>> {
    // Only track user-installed packages (manual installs) and ignore
    // base system packages (required/important/standard).
    let manual_out = Command::new("apt-mark")
        .args(["showmanual"])
        .output()
        .context("failed to run apt-mark showmanual")?;

    if !manual_out.status.success() {
        anyhow::bail!("apt-mark showmanual failed");
    }

    let manual_stdout = String::from_utf8(manual_out.stdout)?;
    let manual = manual_stdout
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect::<HashSet<_>>();

    let priority_out = Command::new("dpkg-query")
        .args(["--show", "--showformat=${binary:Package} ${Priority}\n"])
        .output()
        .context("failed to run dpkg-query for priorities")?;

    if !priority_out.status.success() {
        anyhow::bail!("dpkg-query priority query failed");
    }

    let priority_stdout = String::from_utf8(priority_out.stdout)?;
    let priorities = priority_stdout
        .lines()
        .filter_map(|l| {
            let line = l.trim();
            if line.is_empty() {
                return None;
            }
            let mut parts = line.split_whitespace();
            let name = parts.next()?.to_string();
            let prio = parts.next().map(|p| p.to_string()).unwrap_or_default();
            Some((name, prio))
        })
        .collect::<HashMap<_, _>>();

    let system_priorities = ["required", "important", "standard"];

    let pkgs = manual
        .into_iter()
        .filter(|name| {
            priorities
                .get(name)
                .map(|p| !system_priorities.contains(&p.as_str()))
                .unwrap_or(true)
        })
        .collect::<HashSet<_>>();

    Ok(pkgs)
}
