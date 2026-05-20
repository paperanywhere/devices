//! Soldered (formerly e-radionica) — Inkplate integrated boards.
//!
//! Inkplate boards pair an ESP32 with parallel-driven e-paper panels
//! (originally salvaged Kindle screens, now custom-fabricated). Different
//! controller family from the rest of the catalog — packing is 2 bpp
//! big-endian for the bundled Gray4 mode.

use crate::{Board, BoardKind, ColorMode, McuFamily, PackingKind, PowerPolicy};

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
