//! Seeed Studio — reTerminal E-series integrated boards.
//!
//! All four E-series products share the same MCU + chassis platform
//! (ESP32-S3R8, 8 MB PSRAM, 32 MB flash, WiFi 2.4 GHz, BT 5.0, USB-C,
//! microSD ≤ 32 GB, 3 buttons, buzzer, microphone, temp + humidity
//! sensor), so a single firmware crate covers all four via per-board
//! Cargo features that vary only the panel driver + pin map.

use crate::{Board, BoardKind, ColorMode, McuFamily, PackingKind, PowerPolicy};

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
