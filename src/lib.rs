//! Canonical board / panel catalog for the paperanywhere stack.
//!
//! This is the **single source of truth** for "what panels and integrated
//! boards exist." Both the backend (which seeds `panel_models` at migration
//! time) and the firmware (which composes pin maps on top of these metadata
//! entries) depend on this crate, so the dashboard's device-creation wizard
//! and the firmware build matrix never drift.
//!
//! ## What lives here vs what doesn't
//!
//! Lives here:
//! - Vendor, model name, panel kind (bare panel vs integrated board)
//! - Physical dimensions + supported color modes + packing kind
//! - Capability flags (battery, buttons, sensors, buzzer, SD card)
//! - MCU family + the Cargo feature flag for integrated boards
//! - Datasheet URL
//! - Stable numeric `id` (matches the `panel_models.id` primary key)
//!
//! Stays in the firmware repo:
//! - GPIO pin numbers (SCK / MOSI / CS / DC / RST / BUSY)
//! - Panel controller IC choice (UC8179 / IT8951 / UC8159)
//! - Per-controller quirks like `panel_data_inverted`
//! - Default power policy + sleep interval (firmware can override per board)
//!
//! ## ID stability
//!
//! `id` values are part of the dashboard's URL space (`/devices/:id`) and
//! the on-device NVS records (`panel_model_id`). They MUST stay stable
//! across catalog revisions. Add new boards at the end of [`CATALOG`];
//! never reuse a retired id.

pub use paperanywhere_proto::{ColorMode, PackingKind, PowerPolicy};
use serde::{Deserialize, Serialize};

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
    /// device of this type. Members of [`supported_color_modes`].
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

// ── Named per-board constants ──────────────────────────────────────────────
//
// Firmware consumes these directly (e.g. `use paperanywhere_devices::
// RETERMINAL_E1001;`) so each board file is a one-liner that pulls shared
// metadata + adds GPIO pins. Backend iterates `CATALOG` for the seed.

pub const RETERMINAL_E1001: Board = Board {
    id: 1,
    slug: "reterminal-e1001",
    vendor: "Seeed Studio",
    model: "reTerminal E1001",
    kind: BoardKind::IntegratedBoard,
    width_px: 800,
    height_px: 480,
    default_color_mode: ColorMode::Mono1bpp,
    supported_color_modes: &[ColorMode::Mono1bpp, ColorMode::Gray4],
    packing_kind: PackingKind::RowMajorMsbFirst1bpp,
    mcu_family: McuFamily::Esp32s3,
    has_battery: true,
    has_buttons: true,
    has_sensors: true,
    has_buzzer: true,
    has_sd_card: true,
    datasheet_url: "https://wiki.seeedstudio.com/getting_started_with_reterminal_e1001/",
    firmware_target: Some("board-reterminal-e1001"),
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600, // 6h
};

pub const RETERMINAL_E1002: Board = Board {
    id: 2,
    slug: "reterminal-e1002",
    vendor: "Seeed Studio",
    model: "reTerminal E1002",
    kind: BoardKind::IntegratedBoard,
    width_px: 800,
    height_px: 480,
    default_color_mode: ColorMode::Color7,
    supported_color_modes: &[ColorMode::Color7],
    packing_kind: PackingKind::AcepIndexed4bpp,
    mcu_family: McuFamily::Esp32s3,
    has_battery: true,
    has_buttons: true,
    has_sensors: true,
    has_buzzer: true,
    has_sd_card: true,
    datasheet_url: "https://wiki.seeedstudio.com/reterminal_e10xx_main_page/",
    firmware_target: Some("board-reterminal-e1002"),
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

pub const RETERMINAL_E1003: Board = Board {
    id: 3,
    slug: "reterminal-e1003",
    vendor: "Seeed Studio",
    model: "reTerminal E1003",
    kind: BoardKind::IntegratedBoard,
    width_px: 1404,
    height_px: 1872,
    default_color_mode: ColorMode::Mono1bpp,
    supported_color_modes: &[ColorMode::Mono1bpp, ColorMode::Gray16],
    packing_kind: PackingKind::RowMajorMsbFirst1bpp,
    mcu_family: McuFamily::Esp32s3,
    has_battery: true,
    has_buttons: true,
    has_sensors: true,
    has_buzzer: true,
    has_sd_card: true,
    datasheet_url: "https://wiki.seeedstudio.com/reterminal_e10xx_main_page/",
    firmware_target: Some("board-reterminal-e1003"),
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

pub const RETERMINAL_E1004: Board = Board {
    id: 4,
    slug: "reterminal-e1004",
    vendor: "Seeed Studio",
    model: "reTerminal E1004",
    kind: BoardKind::IntegratedBoard,
    width_px: 1200,
    height_px: 1600,
    default_color_mode: ColorMode::Color7,
    supported_color_modes: &[ColorMode::Color7],
    packing_kind: PackingKind::AcepIndexed4bpp,
    mcu_family: McuFamily::Esp32s3,
    has_battery: true,
    has_buttons: true,
    has_sensors: true,
    has_buzzer: true,
    has_sd_card: true,
    datasheet_url: "https://wiki.seeedstudio.com/reterminal_e10xx_main_page/",
    firmware_target: Some("board-reterminal-e1004"),
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

pub const WAVESHARE_42_BW: Board = Board {
    id: 5,
    slug: "waveshare-42-bw",
    vendor: "Waveshare",
    model: "4.2\" BW (UC8176)",
    kind: BoardKind::BarePanel,
    width_px: 400,
    height_px: 300,
    default_color_mode: ColorMode::Mono1bpp,
    supported_color_modes: &[ColorMode::Mono1bpp],
    packing_kind: PackingKind::RowMajorMsbFirst1bpp,
    mcu_family: McuFamily::Unspecified,
    has_battery: false,
    has_buttons: false,
    has_sensors: false,
    has_buzzer: false,
    has_sd_card: false,
    datasheet_url: "https://www.waveshare.com/wiki/4.2inch_e-Paper_Module",
    firmware_target: None,
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

pub const WAVESHARE_42_3COLOR: Board = Board {
    id: 6,
    slug: "waveshare-42-3color",
    vendor: "Waveshare",
    model: "4.2\" 3-color (BW+Red)",
    kind: BoardKind::BarePanel,
    width_px: 400,
    height_px: 300,
    default_color_mode: ColorMode::MonoRed1bpp,
    supported_color_modes: &[ColorMode::Mono1bpp, ColorMode::MonoRed1bpp],
    packing_kind: PackingKind::RowMajorMsbFirst1bpp,
    mcu_family: McuFamily::Unspecified,
    has_battery: false,
    has_buttons: false,
    has_sensors: false,
    has_buzzer: false,
    has_sd_card: false,
    datasheet_url: "https://www.waveshare.com/wiki/4.2inch_e-Paper_Module_(B)",
    firmware_target: None,
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

pub const WAVESHARE_75_BW: Board = Board {
    id: 7,
    slug: "waveshare-75-bw",
    vendor: "Waveshare",
    model: "7.5\" BW (800x480)",
    kind: BoardKind::BarePanel,
    width_px: 800,
    height_px: 480,
    default_color_mode: ColorMode::Mono1bpp,
    supported_color_modes: &[ColorMode::Mono1bpp],
    packing_kind: PackingKind::RowMajorMsbFirst1bpp,
    mcu_family: McuFamily::Unspecified,
    has_battery: false,
    has_buttons: false,
    has_sensors: false,
    has_buzzer: false,
    has_sd_card: false,
    datasheet_url: "https://www.waveshare.com/wiki/7.5inch_e-Paper_HAT",
    firmware_target: Some("board-generic-esp32s3-waveshare-75"),
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

pub const WAVESHARE_73_ACEP: Board = Board {
    id: 8,
    slug: "waveshare-73-acep",
    vendor: "Waveshare",
    model: "7.3\" 7-color ACeP (Spectra 6)",
    kind: BoardKind::BarePanel,
    width_px: 800,
    height_px: 480,
    default_color_mode: ColorMode::Color7,
    supported_color_modes: &[ColorMode::Color7],
    packing_kind: PackingKind::AcepIndexed4bpp,
    mcu_family: McuFamily::Unspecified,
    has_battery: false,
    has_buttons: false,
    has_sensors: false,
    has_buzzer: false,
    has_sd_card: false,
    datasheet_url: "https://www.waveshare.com/wiki/7.3inch_e-Paper_HAT_(F)",
    firmware_target: None,
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

pub const INKPLATE_6: Board = Board {
    id: 9,
    slug: "inkplate-6",
    vendor: "Soldered",
    model: "Inkplate 6",
    kind: BoardKind::IntegratedBoard,
    width_px: 800,
    height_px: 600,
    default_color_mode: ColorMode::Mono1bpp,
    supported_color_modes: &[ColorMode::Mono1bpp, ColorMode::Gray4],
    packing_kind: PackingKind::RowMajorBe2bpp,
    mcu_family: McuFamily::Esp32,
    has_battery: true,
    has_buttons: true,
    has_sensors: false,
    has_buzzer: false,
    has_sd_card: true,
    datasheet_url: "https://inkplate.com/products/inkplate-6/",
    firmware_target: Some("board-inkplate-6"),
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

pub const INKPLATE_10: Board = Board {
    id: 10,
    slug: "inkplate-10",
    vendor: "Soldered",
    model: "Inkplate 10",
    kind: BoardKind::IntegratedBoard,
    width_px: 1200,
    height_px: 825,
    default_color_mode: ColorMode::Mono1bpp,
    supported_color_modes: &[ColorMode::Mono1bpp, ColorMode::Gray4],
    packing_kind: PackingKind::RowMajorBe2bpp,
    mcu_family: McuFamily::Esp32,
    has_battery: true,
    has_buttons: true,
    has_sensors: false,
    has_buzzer: false,
    has_sd_card: true,
    datasheet_url: "https://inkplate.com/products/inkplate-10/",
    firmware_target: Some("board-inkplate-10"),
    default_power_policy: PowerPolicy::ScheduledWake,
    default_sleep_interval_sec: 21_600,
};

/// Ordered catalog. Iterate this in the backend's panel_models seed; new
/// entries go at the end so existing ids stay stable.
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
    let mut i = 0;
    while i < CATALOG.len() {
        if CATALOG[i].id == id {
            return Some(CATALOG[i]);
        }
        i += 1;
    }
    None
}

/// Lookup by slug (matches the Cargo feature minus the `board-` prefix).
pub fn by_slug(slug: &str) -> Option<&'static Board> {
    let mut i = 0;
    while i < CATALOG.len() {
        if CATALOG[i].slug.as_bytes() == slug.as_bytes() {
            return Some(CATALOG[i]);
        }
        i += 1;
    }
    None
}

/// Lookup by Cargo firmware-target flag (e.g. `board-reterminal-e1001`).
pub fn by_firmware_target(target: &str) -> Option<&'static Board> {
    let mut i = 0;
    while i < CATALOG.len() {
        if let Some(ft) = CATALOG[i].firmware_target {
            if ft.as_bytes() == target.as_bytes() {
                return Some(CATALOG[i]);
            }
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_are_unique_and_dense() {
        let mut seen = [false; 256];
        for board in CATALOG {
            let i = board.id as usize;
            assert!(!seen[i], "duplicate id {}", board.id);
            seen[i] = true;
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
