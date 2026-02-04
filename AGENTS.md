
# Project Name
dmini

# Purpose
dmini is a declarative, local-first system configuration manager for Debian-based Linux systems, written in Rust.
It allows users to define the desired system state (packages, services, dotfiles, etc.) in YAML, then:
1. Load configuration
2. Read current system state
3. Compute a diff (plan)
4. Apply changes safely

dmini is inspired by Nix, Terraform, Kubernetes reconciliation, and modern IaC tools, but operates on top of traditional Linux tooling (apt, systemd, filesystem).
It is not a task runner, and not a shell script generator.

# Core Philosophy
- Declarative, not imperative.
- State reconciliation, not task execution.
- Separate plan from apply.
- Idempotent behavior.
- Local-first (single machine).
- Config is the source of truth.
- Modules are composition blocks, not actions.
- The user describes what should exist, not how to do it.

# Non-Goals
Agents must NOT turn dmini into:
- An Ansible clone.
- A shell script wrapper.
- A remote orchestration system.
- A full OS like NixOS.
- No SSH orchestration.
- No YAML task DSL.
- No imperative step pipelines.

# Architecture Model
> Pipeline:
>
> CLI → Config Loader → Module Merger → State Reader → Planner → Executor

Layers:
- cli — argument parsing
- core — orchestration logic (run_plan, run_apply)
- state — read current system state
- planner — compute diffs
- executor — apply changes
- backend — distro abstraction (apt/dpkg, sytemctl etc)

Do not embed distro logic directly into planner; use backend traits when possible.
main.rs should stay small and only wire CLI → core.

# Configuration Model
Users define desired state using YAML:
```
modules:
 - base
 - dev

packages:
  - git
  - neovim

services:
  enable:
    - docker

dotfiles:
  - source: ./dotfiles/zshrc
    target: ~/.zshrc
    mode: symlink
```
Modules are merged before planning.
Modules are reusable config blocks, not executable scripts.

# Plan / Apply Model
Commands:
> - dmini plan - **plan** never mutates the system.
> - dmini apply - **apply** executes a previously computed plan.

All actions must be derived from diffs between desired_state vs current_state Never blindly run commands from config.

# Idempotency
Running dmini apply multiple times must converge to zero changes once the system matches the configuration.
- Skip already-installed packages.
- Skip already-enabled services.
- Skip correct dotfiles.
- Detect conflicts.

# Module System
Modules are YAML files under:
- modules/*.yaml
- hosts/*.yaml

They are merged into a final config.
They are not procedural steps.

# Supported Resources (v1)
- Packages (APT / dpkg)
- Services (systemd)
- Dotfiles (filesystem)

Later:
- Network
- Users
- Firewall
- Hooks

# Coding Guidelines
- Use Rust 2021 edition.
- Use anyhow for errors.
- Use serde_yaml for config.
- Use HashSet for diff logic.
- Keep functions small and composable.
- No logic in main.rs except wiring.
- Prefer pure functions in planner.
- Separate IO from logic where possible.

# Safety Rules
- plan must never mutate the system.
- apply must support --dry-run.
- Backup files before overwriting.
- Avoid destructive defaults.
- Print what will happen before doing it.
