use anyhow;

use crate::state::packages;

pub fn run_plan() -> anyhow::Result<()> {
    println!("-----testing---------");
    let pkgs = packages::installed()?;
    println!("Installed packages count: {}", pkgs.len());
    for (i, pkg) in pkgs.iter().enumerate() {
        if i > 10 {
            break;
        }
        println!("{}", pkg)
    }
    Ok(())
}
