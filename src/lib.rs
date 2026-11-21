//! inventory-profiles-next runtime for Horizon: the inventory grid model,
//! sorting comparators, layouts and the move plan that produces the slot
//! clicks. Exact 1:1 Rust port of Inventory Profiles Next (upstream
//! https://codeberg.org/Inventory-Profiles-Next-Mod/IPN,
//! AGPL-3.0-or-later, upstream license shipped in the package), ported for
//! the Horizon client by sparticle.
//!
//! The GUI/screen glue (opening the player's container screen, reading live
//! slots, turning the move plan into slot clicks) waits on Horizon's
//! container-screen mod API surface; see the gap section in
//! `horizon-mod.toml`.

pub mod grid;
pub mod sort;

pub use grid::{Grid, Stack};
pub use sort::{move_plan, Direction, Layout, SortBy};

/// The mod. No settings yet: the model layer carries upstream behavior, the
/// config surface lands with the GUI glue it drives.
#[derive(Default)]
pub struct InventoryProfilesNext;

#[mod_api::horizon_mod]
impl mod_api::Hooks for InventoryProfilesNext {
    fn register(&self, _r: &mut dyn mod_api::Registrar, _cx: &mod_api::RegistrationContext) {}
}
