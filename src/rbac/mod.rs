//! Role‑Based Access Control (RBAC) utilities.
//!
//! This module groups together the bootstrap logic and permission constants
//! needed to initialise and enforce access control within the application.
//! It re‑exports the `bootstrap` submodule (which seeds the database) and the
//! `permissions` submodule containing permission and role codes.

pub mod bootstrap;
pub mod permissions;
