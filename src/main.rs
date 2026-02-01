mod core;
mod state;

use crate::core::plan;
fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    plan::run_plan()?;
    Ok(())
}
