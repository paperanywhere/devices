//! Canonical board / panel catalog for the paperanywhere stack.
//!
//! Single source of truth for "what panels and integrated boards exist."
//! Backend seeds `panel_models` from this; firmware composes pin maps on
//! top of these `Board` consts so the dashboard's device-creation wizard
//! and the firmware build matrix never drift.
//!
//! Boards are grouped by vendor in submodules under [`vendors`]. The
//! flat [`CATALOG`] slice is the iteration order used for migrations and
//! API listings — new boards always go at the end so existing `id`s stay
//! wire-stable.
//!
//! ## What lives here vs in the firmware repo
//!
//! Here:
//! - Vendor, model name, panel kind (bare panel vs integrated board)
//! - Physical dimensions, supported color modes, packing kind
//! - Capability flags (battery, buttons, sensors, buzzer, SD card)
//! - MCU family + Cargo feature flag for integrated boards
//! - Datasheet URL, default power policy + sleep interval
//!
//! Firmware-local:
//! - GPIO pin numbers (SCK / MOSI / CS / DC / RST / BUSY)
//! - Panel controller IC choice + per-controller quirks
//!
//! ## ID stability
//!
//! `id` lands in NVS on real devices and in dashboard URLs. **Never
//! reuse a retired id.** Add new entries at the bottom of the vendor
//! module and append to [`CATALOG`].

pub use paperanywhere_proto::{ColorMode, PackingKind, PowerPolicy};
use serde::{Deserialize, Serialize};

pub mod vendors;

// Re-export the named board consts at the crate root so consumers can
// `use paperanywhere_devices::RETERMINAL_E1001;` without caring which
// vendor module owns it. Vendor modules stay around for grouping +
// readability, but the public API stays flat.
pub use vendors::seeed::{RETERMINAL_E1001, RETERMINAL_E1002, RETERMINAL_E1003, RETERMINAL_E1004};
pub use vendors::soldered::{INKPLATE_6, INKPLATE_10};
pub use vendors::waveshare::{
    WAVESHARE_42_3COLOR, WAVESHARE_42_BW, WAVESHARE_73_ACEP, WAVESHARE_75_BW,
};

/// Whether the entry describes a bare panel (user wires their own carrier)
/// or an integrated board (firmware ships per-product with the GPIO map
/// baked in).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoardKind {
    BarePanel,
    IntegratedBoard,
}

/// Microcontroller family used by an integrated board. Bare panels report
/// [`McuFamily::Unspecified`] since the user picks the carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McuFamily {
    Esp32,
    Esp32s2,
    Esp32s3,
    Esp32c3,
    Esp32c6,
    Unspecified,
}

/// One board/panel entry. Static lifetime everywhere: the whole catalog
/// is `const`, so consumers can reference entries without allocating.
#[derive(Debug, Clone)]
pub struct Board {
    /// Stable primary key. See module docs on stability.
    pub id: i32,
    /// Lowercase slug used for filenames, feature flags, and URL paths
    /// (e.g. `reterminal-e1001`). Matches the Cargo feature name minus
    /// the `board-` prefix.
    pub slug: &'static str,
    pub vendor: &'static str,
    pub model: &'static str,
    pub kind: BoardKind,
    pub width_px: u32,
    pub height_px: u32,
    /// What the dashboard offers by default when the user creates a
    /// device of this type. Always a member of [`Board::supported_color_modes`].
    pub default_color_mode: ColorMode,
    pub supported_color_modes: &'static [ColorMode],
    pub packing_kind: PackingKind,
    pub mcu_family: McuFamily,
    pub has_battery: bool,
    pub has_buttons: bool,
    pub has_sensors: bool,
    pub has_buzzer: bool,
    pub has_sd_card: bool,
    pub datasheet_url: &'static str,
    /// Firmware Cargo feature flag for this board, e.g.
    /// `board-reterminal-e1001`. [`None`] for bare panels (no firmware
    /// build is wired for them — the user provides their own carrier).
    pub firmware_target: Option<&'static str>,
    /// Default power policy the firmware enters when this board first
    /// boots, before the server's `/state` response refines it.
    pub default_power_policy: PowerPolicy,
    /// Default deep-sleep interval in seconds for `ScheduledWake`. The
    /// server can override per-device via `/state`.
    pub default_sleep_interval_sec: u32,
}

/// Ordered catalog. Iterate this in the backend's panel_models seed; new
/// entries go at the end so existing ids stay stable. Order matches the
/// `id` field 1:1 today, which migration code can rely on.
pub const CATALOG: &[&Board] = &[
    &RETERMINAL_E1001,
    &RETERMINAL_E1002,
    &RETERMINAL_E1003,
    &RETERMINAL_E1004,
    &WAVESHARE_42_BW,
    &WAVESHARE_42_3COLOR,
    &WAVESHARE_75_BW,
    &WAVESHARE_73_ACEP,
    &INKPLATE_6,
    &INKPLATE_10,
];

/// Lookup by primary key. Linear scan; the catalog is tiny so a hashmap
/// would be more overhead than benefit.
pub fn by_id(id: i32) -> Option<&'static Board> {
    CATALOG.iter().copied().find(|b| b.id == id)
}

/// Lookup by slug (matches the Cargo feature minus the `board-` prefix).
pub fn by_slug(slug: &str) -> Option<&'static Board> {
    CATALOG.iter().copied().find(|b| b.slug == slug)
}

/// Lookup by Cargo firmware-target flag (e.g. `board-reterminal-e1001`).
pub fn by_firmware_target(target: &str) -> Option<&'static Board> {
    CATALOG
        .iter()
        .copied()
        .find(|b| b.firmware_target == Some(target))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_unique() {
        for (i, a) in CATALOG.iter().enumerate() {
            for b in &CATALOG[i + 1..] {
                assert_ne!(a.id, b.id, "duplicate id {} ({} vs {})", a.id, a.slug, b.slug);
            }
        }
    }

    #[test]
    fn slugs_are_unique() {
        for (i, a) in CATALOG.iter().enumerate() {
            for b in &CATALOG[i + 1..] {
                assert_ne!(a.slug, b.slug, "duplicate slug {}", a.slug);
            }
        }
    }

    #[test]
    fn lookup_round_trips() {
        for board in CATALOG {
            assert_eq!(by_id(board.id).unwrap().slug, board.slug);
            assert_eq!(by_slug(board.slug).unwrap().id, board.id);
        }
    }

    #[test]
    fn default_color_mode_is_supported() {
        for board in CATALOG {
            assert!(
                board.supported_color_modes.contains(&board.default_color_mode),
                "{}: default_color_mode {:?} not in supported list",
                board.slug, board.default_color_mode
            );
        }
    }

    #[test]
    fn integrated_boards_have_firmware_target() {
        for board in CATALOG {
            if matches!(board.kind, BoardKind::IntegratedBoard) {
                assert!(
                    board.firmware_target.is_some(),
                    "{} is an integrated board but has no firmware_target",
                    board.slug
                );
            }
        }
    }
}
