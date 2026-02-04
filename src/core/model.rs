use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct DesiredState {
    pub packages: HashSet<String>,
    pub services: HashSet<String>,
    pub dotfiles: HashSet<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CurrentState {
    pub packages: HashSet<String>,
    pub services: HashSet<String>,
    pub dotfiles: HashSet<String>,
}

#[derive(Debug, Clone)]
pub enum Action {
    InstallPackage { name: String },
    RemovePackage { name: String },
}

#[derive(Debug, Clone, Default)]
pub struct Plan {
    pub actions: Vec<Action>,
}

impl Plan {
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}
