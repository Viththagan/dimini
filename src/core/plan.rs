use std::collections::HashSet;

use anyhow;

use crate::{
    core::model::{Action, CurrentState, DesiredState, Plan},
    state::packages,
};

pub fn run_plan() -> anyhow::Result<()> {
    println!("-----testing---------");
    let pkgs = packages::installed()?;

    let current = CurrentState {
        packages: pkgs,
        services: HashSet::new(),
        dotfiles: HashSet::new(),
    };

    let desired = DesiredState {
        packages: ["git", "neovim"].into_iter().map(String::from).collect(),
        services: HashSet::new(),
        dotfiles: HashSet::new(),
    };

    let plan = diff(&desired, &current);
    println!("Plans count: {}", plan.actions.len());
    for action in plan.actions {
        println!("{:?}", action);
    }
    Ok(())
}

fn diff(desired: &DesiredState, current: &CurrentState) -> Plan {
    let mut actions: Vec<Action> = Vec::new();

    let to_install: HashSet<_> = desired
        .packages
        .difference(&current.packages)
        .cloned()
        .collect();
    let to_remove: HashSet<_> = current
        .packages
        .difference(&desired.packages)
        .cloned()
        .collect();

    for name in to_install {
        actions.push(Action::InstallPackage { name });
    }

    for name in to_remove {
        actions.push(Action::RemovePackage { name });
    }

    Plan { actions }
}
